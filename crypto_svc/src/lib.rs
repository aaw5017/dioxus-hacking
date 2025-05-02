use argon2::{
    Argon2,
    password_hash::{
        Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng,
    },
};

fn get_argon<'a>() -> Result<Argon2<'a>, Error> {
    let argon2 = Argon2::new_with_secret(
        config::CRYPTO_PEPPER_BYTES,
        argon2::Algorithm::Argon2id,
        argon2::Version::default(),
        argon2::Params::default(),
    )?;

    Ok(argon2)
}

pub fn hash_it(secret: impl Into<String>) -> Result<String, Error> {
    let pw_str: String = secret.into();
    let pw_bytes = pw_str.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = get_argon()?;

    let password_hash = argon2.hash_password(pw_bytes, &salt)?.to_string();

    // TODO: remove me
    println!("Password hash: {}", &password_hash);

    // TODO: remove assertion once POC testing is done
    // TODO: add actual test for this
    let parsed_hash = PasswordHash::new(&password_hash)?;
    assert!(get_argon()?.verify_password(pw_bytes, &parsed_hash).is_ok());

    Ok(password_hash)
}
