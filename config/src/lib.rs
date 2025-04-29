use dotenvy_macro::dotenv;

#[derive(Debug)]
pub struct Configuration {
    pub server_base_uri: String,
    pub server_port: u16,
}

pub fn get() -> Configuration {
    let port = dotenv!("SERVER_PORT", "SERVER_PORT not found in ENV!");
    let base_uri = dotenv!("SERVER_BASE_URI", "SERVER_BASE_URI not found in ENV!");
    let port_num = port.parse::<u16>().unwrap_or_else(|err| {
        println!("Error parsing PORT from environment variable: {}", err);
        ::std::process::exit(1);
    });

    let thing = Configuration {
        server_base_uri: format!("{}:{}", base_uri, port_num),
        server_port: port_num,
    };
    return thing;
}
