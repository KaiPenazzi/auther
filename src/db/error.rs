use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum DBError {
    EmailExists,
    NotFound,
    DbError(sqlx::Error),
}

impl From<sqlx::Error> for DBError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
                DBError::EmailExists
            }
            sqlx::Error::RowNotFound => DBError::NotFound,
            other => DBError::DbError(other),
        }
    }
}

impl IntoResponse for DBError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            DBError::EmailExists => (StatusCode::CONFLICT, "Email already exists".to_string()),
            DBError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            DBError::DbError(e) => {
                eprintln!("Database error occurred: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error occurred".to_string(),
                )
            }
        };

        (status, body).into_response()
    }
}
