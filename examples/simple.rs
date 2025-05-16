use optics::{PartialGetter, Setter, mapped_fallible_iso, mapped_lens, mapped_prism};

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

fn main() {
    // Define lenses to focus on subfields
    let http_lens = mapped_lens(
        |app: &AppConfig| app.http.clone(),
        |app, http| app.http = http,
    );

    let bind_address_prism = mapped_prism(
        |http: &HttpConfig| http.bind_address.clone().ok_or(()),
        |http, addr| http.bind_address = Some(addr),
    );

    let minimum_port = 1024;

    // Define a fallible isomorphism between String and u16 (parsing a port)
    let port_fallible_iso = mapped_fallible_iso(
        |addr: &String| {
            addr.rsplit(':')
                .next()
                .and_then(|port| port.parse::<u16>().ok())
                .ok_or(())
        },
        move |port: &u16| {
            (*port > minimum_port)
                .then_some(format!("0.0.0.0:{}", port))
                .ok_or(())
        },
    );

    // Compose lens and fallible iso into a ComposedFallibleIso
    let http_bind_address_prism = http_lens.compose_with_prism(bind_address_prism);
    let http_bind_address_port_prism =
        http_bind_address_prism.compose_with_fallible_iso::<(), _, _>(port_fallible_iso);

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
