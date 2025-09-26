use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use axum::{Json, extract::State};
use rand_core::OsRng;
use serde::Deserialize;

use crate::{AppState, db::DBError, model::user::User};

#[derive(Deserialize)]
pub struct UserRegistration {
    pub name: String,
    pub email: String,
    pub password: String,
    pub roles: Option<Vec<String>>,
}

impl UserRegistration {
    pub fn to_db_user(self) -> User {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(self.password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        User {
            id: None,
            name: self.name,
            email: self.email,
            password_hash,
            roles: self.roles,
            created_at: None,
            updated_at: None,
        }
    }
}

#[axum::debug_handler]
pub async fn register(
    State(state): State<AppState>,
    Json(user): Json<UserRegistration>,
) -> Result<Json<User>, DBError> {
    let db_user = user.to_db_user();
    let added_user = state.db_client.add_user(db_user).await?;
    Ok(Json(added_user))
}
