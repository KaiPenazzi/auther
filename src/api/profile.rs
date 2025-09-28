use axum::{Json, extract::State};
use axum_jwt_auth::Claims;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    db::error::DBError,
    model::{
        jwt,
        user::{User, UserUpdate},
    },
};

#[derive(Serialize)]
pub struct UserProfile {
    name: String,
    email: String,
    roles: Option<Vec<String>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl UserProfile {
    pub fn from_user(user: &User) -> Self {
        Self {
            name: user.name.clone(),
            email: user.email.clone(),
            roles: user.roles.clone(),
            created_at: user.created_at.unwrap().clone(),
            updated_at: user.updated_at.unwrap().clone(),
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateProfile {
    name: Option<String>,
    email: Option<String>,
}

pub async fn get_profile(
    State(state): State<AppState>,
    Claims(claims): Claims<jwt::Claims>,
) -> Result<Json<UserProfile>, DBError> {
    let user = state.db_client.get_user_id(&claims.id).await?;
    Ok(Json(UserProfile::from_user(&user)))
}

pub async fn post_profile(
    State(state): State<AppState>,
    Claims(claims): Claims<jwt::Claims>,
    Json(update_user): Json<UpdateProfile>,
) -> Result<Json<UserProfile>, DBError> {
    let user = UserUpdate {
        id: claims.id,
        name: update_user.name,
        email: update_user.email,
    };

    let updateted_user = state.db_client.update_user(user).await?;
    Ok(Json(UserProfile::from_user(&updateted_user)))
}
