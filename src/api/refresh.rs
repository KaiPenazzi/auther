use axum::{Json, extract::State};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, Header, encode};

use crate::{
    AppState,
    model::{
        error::AppError,
        jwt::{Claims, JWT, Refresh, Tokens, generate_refresh_token},
    },
};

#[axum::debug_handler]
pub async fn refresh(
    State(state): State<AppState>,
    Json(token): Json<String>,
) -> Result<Json<Tokens>, AppError> {
    let user = state.db_client.get_user_refresh(Refresh(token)).await?;
    let header = Header::new(Algorithm::RS256);

    let exp = Utc::now() + Duration::hours(1);
    let claims = Claims {
        id: user.id.clone().unwrap(),
        name: user.name.clone(),
        email: user.email,
        roles: user.roles,
        iat: Utc::now().timestamp() as u64,
        exp: exp.timestamp() as u64,
    };

    let jwt = encode(&header, &claims, &state.encoding_key.clone()).unwrap();
    let refresh = generate_refresh_token();
    state
        .db_client
        .add_refresh_token(refresh.clone(), user.id.clone().unwrap())
        .await?;

    Ok(Json(Tokens {
        jwt: JWT(jwt),
        refresh,
    }))
}
