use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use chrono::{DateTime, Utc};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<Uuid>,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub roles: Option<Vec<String>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(reg: UserRegistration) -> Self {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(reg.password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        User {
            id: None,
            name: reg.name,
            email: reg.email,
            password_hash,
            roles: reg.roles,
            created_at: None,
            updated_at: None,
        }
    }

    pub fn verify_psw(&self, psw: &str) -> bool {
        let parsed_hash = match PasswordHash::new(&self.password_hash) {
            Ok(hash) => hash,
            Err(_) => return false,
        };

        Argon2::default()
            .verify_password(psw.as_bytes(), &parsed_hash)
            .is_ok()
    }
}

#[derive(Deserialize)]
pub struct UserRegistration {
    pub name: String,
    pub email: String,
    pub password: String,
    pub roles: Option<Vec<String>>,
}
