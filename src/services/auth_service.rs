use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub struct AuthService<'a> {
    argon2: Argon2<'a>,
    salt: SaltString,
}

pub trait AuthServiceImpl<'a> {
    fn new() -> Self;

    async fn hash_password(&self, password: String)
        -> Result<String, argon2::password_hash::Error>;

    async fn verify_password(
        &self,
        password: String,
        hash: String,
    ) -> Result<bool, argon2::password_hash::Error>;
}

impl<'a> AuthServiceImpl<'a> for AuthService<'a> {
    fn new() -> Self {
        Self {
            argon2: Argon2::default(),
            salt: SaltString::generate(&mut OsRng),
        }
    }

    async fn hash_password(
        &self,
        password: String,
    ) -> Result<String, argon2::password_hash::Error> {
        let password_hash = self
            .argon2
            .hash_password(password.as_bytes(), &self.salt)?
            .to_string();
        Ok(password_hash.to_string())
    }

    async fn verify_password(
        &self,
        password: String,
        hash: String,
    ) -> Result<bool, argon2::password_hash::Error> {
        let password_hash = PasswordHash::new(&hash)?;
        let matches = self
            .argon2
            .verify_password(password.as_bytes(), &password_hash);

        if matches.is_ok() {
            Ok(true)
        } else {
            tracing::error!("Failed to verify password: {:?}", matches);
            Err(matches.err().unwrap())
        }
    }
}
