use axum::{Json, extract::State};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};

use crate::{
    AppState,
    model::{
        error::AppError,
        jwt::{Claims, JWT},
        user::UserLogin,
    },
};

#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(user): Json<UserLogin>,
) -> Result<Json<JWT>, AppError> {
    let db_user = state.db_client.get_user(&user.email).await?;
    db_user.verify_psw(&user.password)?;
    let key = EncodingKey::from_rsa_pem(include_bytes!("../../jwt.key")).unwrap();
    let header = Header::new(Algorithm::RS256);

    let exp = Utc::now() + Duration::hours(1);
    let claims = Claims {
        id: db_user.id.clone().unwrap(),
        name: db_user.name.clone(),
        email: db_user.email,
        roles: db_user.roles,
        iat: Utc::now().timestamp() as u64,
        exp: exp.timestamp() as u64,
    };

    let token = encode(&header, &claims, &key).unwrap();
    return Ok(Json(JWT { token: token }));
}
