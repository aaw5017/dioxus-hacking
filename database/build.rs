use anyhow::Result;

fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let db_url = std::env::var("DATABASE_URL")?;

    println!("cargo:rustc-env=DATABASE_URL={}", db_url);

    // run migrations
    std::process::Command::new("sqlx")
        .arg("migrate")
        .arg("run")
        .status()
        .expect("Failed to run migrations");

    Ok(())
}
