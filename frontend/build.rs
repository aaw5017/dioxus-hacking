use dotenvy::dotenv;

fn main() {
    let _ = dotenv();

    let server_base_uri =
        std::env::var("SERVER_BASE_URI").expect("SERVER_BASE_URI not found in ENV!");
    let server_port = std::env::var("SERVER_PORT").unwrap_or("80".into());

    println!(
        "cargo::rustc-env=SERVER_BASE_URI={}:{}",
        server_base_uri, server_port
    );
}
