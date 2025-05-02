use argon2::{
    password_hash::{
        rand_core::OsRng, Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};

pub fn hash_it(secret: impl Into<String>) -> Result<String, Error> {
    let pw_str = secret.into();
    let pw_bytes = pw_str.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::new_with_secret(
        argon2::Algorithm::Argon2id,
        argon2::Version::default(),
        argon2::Params::default(),
    )?;

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password(pw_bytes, &salt)?.to_string();

    // Verify password against PHC string.
    //
    // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
    // `Argon2` instance.
    let parsed_hash = PasswordHash::new(&password_hash)?;
    assert!(Argon2::default()
        .verify_password(pw_bytes, &parsed_hash)
        .is_ok());

    println!("Password hash: {}", &password_hash);

    Ok(password_hash)
}
