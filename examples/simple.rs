use optics::composers::{ComposableLens, ComposablePrism};
use optics::{FallibleIsoImpl, LensImpl, NoFocus, Optic, PrismImpl};

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct HttpConfig {
    bind_address: Option<String>,
    workers: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct AppConfig {
    http: HttpConfig,
    name: String,
}

struct MyError;

impl From<MyError> for NoFocus {
    fn from(_: MyError) -> Self {
        NoFocus
    }
}

impl From<NoFocus> for MyError {
    fn from(_: NoFocus) -> Self {
        MyError
    }
}

fn main() {
    // Define lenses to focus on subfields
    let http_lens =
        LensImpl::<AppConfig, HttpConfig>::new(|app| app.http.clone(), |app, http| app.http = http);

    let bind_address_prism = PrismImpl::<HttpConfig, String>::new(
        |http| http.bind_address.clone(),
        |http, addr| http.bind_address = Some(addr),
    );

    let minimum_port = 1024;
    // Define a fallible isomorphism between String and u16 (parsing a port)
    let port_fallible_iso = FallibleIsoImpl::<String, u16, MyError, _, _>::new(
        |addr: &String| {
            addr.rsplit(':')
                .next()
                .and_then(|port| port.parse::<u16>().ok())
                .ok_or(MyError)
        },
        move |port: &u16| {
            if *port > minimum_port {
                Ok(format!("0.0.0.0:{}", port))
            } else {
                Err(MyError)
            }
        },
    );

    // Compose lens and fallible iso into a ComposedFallibleIso

    let http_bind_address_prism = http_lens.compose_lens_with_prism(bind_address_prism);
    let http_bind_address_port_prism =
        http_bind_address_prism.compose_prism_with_fallible_iso::<MyError>(port_fallible_iso);

    let mut config = AppConfig {
        http: HttpConfig {
            bind_address: Some("127.0.0.1:8080".to_string()),
            workers: 4,
        },
        name: "my_app".into(),
    };

    // Use the composed optic to get the port
    let port = http_bind_address_port_prism.try_get(&config).unwrap();
    println!("Current port: {}", port);

    // Use it to increment the port and update the config
    http_bind_address_port_prism.set(&mut config, port + 1);

    println!("Updated config: {:?}", config);
}
