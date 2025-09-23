use std::sync::Arc;

use axum::extract::FromRef;
use axum_jwt_auth::{JwtDecoderState, LocalDecoder};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use sqlx::PgPool;

use crate::api::router;
use crate::db::PostgresClient;
use crate::model::jwt::Claims;

mod api;
mod db;
mod model;

#[derive(Clone, FromRef)]
struct AppState {
    decoder: JwtDecoderState<Claims>,
    db_client: Arc<PostgresClient>,
}

#[tokio::main]
async fn main() {
    let pool = PgPool::connect("postgres://postgres:default_psw@localhost:5432/postgres")
        .await
        .expect("could not connect to db");

    let db_client = PostgresClient::new(pool).await;

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
        db_client: Arc::new(db_client),
    };

    let app = router::get_router(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// async fn user(Claims(claims): Claims<MyClaims>) -> Json<User> {
//     let user = match claims.name.as_str() {
//         "kai" => User {
//             name: "kai".to_string(),
//             email: "kai@mail.com".to_string(),
//         },
//         "test" => User {
//             name: "test".to_string(),
//             email: "test@mail.com".to_string(),
//         },
//         _ => User {
//             name: "".to_string(),
//             email: "".to_string(),
//         },
//     };
//
//     Json(user)
// }
