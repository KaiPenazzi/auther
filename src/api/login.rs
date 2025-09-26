use axum::{Json, extract::State};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, Header, encode};
use serde::Deserialize;

use crate::{
    AppState,
    model::{
        error::AppError,
        jwt::{Claims, JWT, Tokens, generate_refresh_token},
    },
};

#[derive(Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(user): Json<UserLogin>,
) -> Result<Json<Tokens>, AppError> {
    let db_user = state.db_client.get_user_email(&user.email).await?;
    db_user.verify_psw(&user.password)?;
    let header = Header::new(Algorithm::RS256);

    let exp = Utc::now() + Duration::minutes(15);
    let claims = Claims {
        id: db_user.id.clone().unwrap(),
        name: db_user.name.clone(),
        email: db_user.email,
        roles: db_user.roles,
        iat: Utc::now().timestamp() as u64,
        exp: exp.timestamp() as u64,
    };

    let jwt = encode(&header, &claims, &state.encoding_key.clone()).unwrap();
    let refresh = generate_refresh_token();
    state
        .db_client
        .add_refresh_token(refresh.clone(), db_user.id.clone().unwrap())
        .await?;

    Ok(Json(Tokens {
        jwt: JWT(jwt),
        refresh,
    }))
}
