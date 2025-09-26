use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use crate::model::error::AuthError;

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
    pub fn verify_psw(&self, psw: &str) -> Result<(), AuthError> {
        let parsed_hash = PasswordHash::new(&self.password_hash).map_err(|e| {
            eprintln!("Corrupt password hash: {:?}", e);
            AuthError::InvalidCredentials
        })?;

        Argon2::default()
            .verify_password(psw.as_bytes(), &parsed_hash)
            .map_err(|e| {
                eprintln!("Password verification failed: {:?}", e);
                AuthError::InvalidCredentials
            })
    }
}

pub struct UserUpdate {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
}
