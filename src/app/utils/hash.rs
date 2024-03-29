use argon2::{
    password_hash, password_hash::SaltString, Argon2, PasswordHash, PasswordHasher,
    PasswordVerifier,
};
use rand_core::OsRng;

pub struct Hash {
    salt: SaltString,
    argon2: Box<Argon2<'static>>,
}

impl Hash {
    pub fn new() -> Hash {
        Self {
            salt: SaltString::generate(&mut OsRng),
            argon2: Box::new(Argon2::default()),
        }
    }

    pub fn hash_password(&self, password: &String) -> Result<String, password_hash::Error> {
        let hashed_password = self
            .argon2
            .hash_password(password.as_bytes(), &self.salt)?
            .to_string();

        Ok(hashed_password)
    }

    pub fn verify_password(
        &self,
        password: String,
        hash: String,
    ) -> Result<bool, password_hash::Error> {
        let parsed_hash = PasswordHash::new(&hash)?;

        let pass = self
            .argon2
            .verify_password(password.as_bytes(), &parsed_hash);

        Ok(pass.is_ok())
    }
}
