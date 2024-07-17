use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub struct AuthService;

impl AuthService {
    pub async fn hash_password(password: String) -> Result<String, argon2::password_hash::Error> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        Ok(password_hash.to_string())
    }

    pub async fn verify_password(
        password: String,
        hash: String,
    ) -> Result<bool, argon2::password_hash::Error> {
        let argon2 = Argon2::default();
        let password_hash = PasswordHash::new(&hash)?;
        let matches = argon2.verify_password(password.as_bytes(), &password_hash);

        Ok(matches.is_ok())
    }
}
