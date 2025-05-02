use anyhow::Result;
use dotenvy::dotenv;

fn main() -> Result<()> {
    let _ = dotenv();

    let pepper = std::env::var("CRYPTO_PEPPER").expect("CRYPTO_PEPPER not found in ENV!");

    println!("cargo::rustc-env=CRYPTO_PEPPER={}", pepper);

    Ok(())
}
