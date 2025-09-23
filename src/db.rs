use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::{Executor, PgPool};

use crate::model::user::User;

pub struct PostgresClient {
    pool: PgPool,
}

impl PostgresClient {
    pub async fn new(pool: PgPool) -> Self {
        pool.execute(
            r#"
            CREATE EXTENSION IF NOT EXISTS "pgcrypto";

            CREATE TABLE IF NOT EXISTS users (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                name TEXT NOT NULL,
                email TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                roles TEXT[],
                created_at TIMESTAMPTZ DEFAULT now(),
                updated_at TIMESTAMPTZ DEFAULT now()
            );
        "#,
        )
        .await
        .expect("could not migrate database");
        Self { pool }
    }

    pub async fn add_user(&self, user: User) -> Result<User, DBError> {
        let rec = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (name, email, password_hash, roles)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, email, password_hash, roles, created_at, updated_at
            "#,
            user.name,
            user.email,
            user.password_hash,
            user.roles.as_deref(),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn get_user(&self, email: &str) -> Result<User, DBError> {
        let user = sqlx::query_as!(User, r#"SELECT * FROM users WHERE email = $1"#, email)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }
}

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
