use dotenvy::dotenv;

fn main() {
    let _ = dotenv();

    let server_port = std::env::var("SERVER_PORT").unwrap_or("80".into());

    println!("cargo::rustc-env=SERVER_PORT={}", server_port);
}
