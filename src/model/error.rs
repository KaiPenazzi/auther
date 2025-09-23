use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::db::DBError;

pub enum AuthError {
    InvalidCredentials,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
        };
        (status, error_message).into_response()
    }
}

pub enum AppError {
    Auth(AuthError),
    Db(DBError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Auth(e) => e.into_response(),
            AppError::Db(e) => e.into_response(),
        }
    }
}

impl From<DBError> for AppError {
    fn from(error: DBError) -> Self {
        AppError::Db(error)
    }
}

impl From<AuthError> for AppError {
    fn from(error: AuthError) -> Self {
        AppError::Auth(error)
    }
}
