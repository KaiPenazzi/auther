use std::sync::Arc;

use axum::Json;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Router, extract::FromRef};
use axum_jwt_auth::{Claims, JwtDecoderState, LocalDecoder};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MyClaims {
    name: String,
    iat: u64,
    exp: u64,
}

#[derive(Clone, FromRef)]
struct AppState {
    decoder: JwtDecoderState<MyClaims>,
}

#[tokio::main]
async fn main() {
    let keys = vec![DecodingKey::from_rsa_pem(include_bytes!("../jwt.pub")).unwrap()];
    let validation = Validation::new(Algorithm::RS256);
    let decoder = LocalDecoder::builder()
        .keys(keys)
        .validation(validation)
        .build()
        .unwrap();

    let state = AppState {
        decoder: JwtDecoderState {
            decoder: Arc::new(decoder),
        },
    };

    let app = Router::new()
        .route("/login", post(login))
        .route("/user", get(user))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn login(Json(user): Json<User>) -> impl IntoResponse {
    let key = EncodingKey::from_rsa_pem(include_bytes!("../jwt.key")).unwrap();
    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some("test".to_string());

    let exp = Utc::now() + Duration::hours(1);
    let claims = MyClaims {
        name: user.name.to_string(),
        iat: 12345,
        exp: exp.timestamp() as u64,
    };

    let token = encode(&header, &claims, &key).unwrap();
    token
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
}

async fn user(Claims(claims): Claims<MyClaims>) -> Json<User> {
    let user = match claims.name.as_str() {
        "kai" => User {
            name: "kai".to_string(),
            email: "kai@mail.com".to_string(),
        },
        "test" => User {
            name: "test".to_string(),
            email: "test@mail.com".to_string(),
        },
        _ => User {
            name: "".to_string(),
            email: "".to_string(),
        },
    };

    Json(user)
}
