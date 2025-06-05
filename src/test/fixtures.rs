#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Timespan {
    Seconds(u32),
    Minutes(u32),
    Hours(u32),
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct Config {
    pub delay: Timespan,
    pub filename: String,
    pub main: DatabaseConfig,
    pub aux: Vec<DatabaseConfig>,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: Option<u16>,
    pub create_result: Result<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            delay: Timespan::Minutes(14),
            filename: "abcd".to_string(),
            main: DatabaseConfig {
                host: "main".to_string(),
                port: None,
                create_result: Ok("ok".to_string()),
            },
            aux: vec![
                DatabaseConfig {
                    host: "aux1".to_string(),
                    port: Some(2345),
                    create_result: Err("f1".to_string()),
                },
                DatabaseConfig {
                    host: "aux2".to_string(),
                    port: None,
                    create_result: Err("f2".to_string()),
                },
            ],
        }
    }
}
