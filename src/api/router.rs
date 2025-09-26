use axum::routing::post;
use axum::{Router, routing::get};

use crate::api::profile;
use crate::{
    AppState,
    api::{login::login, refresh::refresh, register::register},
};

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route(
            "/profile",
            get(profile::get_profile).post(profile::post_profile),
        )
        .with_state(state)
}
