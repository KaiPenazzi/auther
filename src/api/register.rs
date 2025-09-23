use axum::{Json, extract::State};

use crate::{
    AppState,
    db::DBError,
    model::user::{User, UserRegistration},
};

#[axum::debug_handler]
pub async fn register(
    State(state): State<AppState>,
    Json(user): Json<UserRegistration>,
) -> Result<Json<User>, DBError> {
    let db_user = User::new(user);
    let added_user = state.db_client.add_user(db_user).await?;
    Ok(Json(added_user))
}
