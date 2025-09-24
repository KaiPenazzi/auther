use axum::Router;
use axum::routing::post;

use crate::{
    AppState,
    api::{login::login, refresh::refresh, register::register},
};

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .with_state(state)
}
