use axum::Router;
use axum::routing::post;

use crate::{
    AppState,
    api::{login, register},
};

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/register", post(register::register))
        .route("/login", post(login::login))
        .with_state(state)
}
