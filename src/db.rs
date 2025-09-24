use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{Months, Utc};
use sha2::{Digest, Sha512};
use sqlx::{Executor, PgPool};
use uuid::Uuid;

use crate::model::{
    error::{AppError, AuthError},
    jwt::Refresh,
    user::User,
};

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

            CREATE TABLE IF NOT EXISTS refresh_tokens (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                hashed_token TEXT UNIQUE NOT NULL,
                expires_at TIMESTAMPTZ NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT now()
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

    pub async fn get_user_email(&self, email: &str) -> Result<User, DBError> {
        let user = sqlx::query_as!(User, r#"SELECT * FROM users WHERE email = $1"#, email)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    async fn get_user_uuid(&self, uuid: &Uuid) -> Result<User, DBError> {
        let user = sqlx::query_as!(User, r#"SELECT * FROM users WHERE id = $1"#, uuid)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn add_refresh_token(&self, token: Refresh, user: Uuid) -> Result<(), DBError> {
        let mut hasher = Sha512::new();
        hasher.update(token.0);
        let hash = format!("{:x}", hasher.finalize());
        let expires_at = Utc::now().checked_add_months(Months::new(6)).unwrap();

        sqlx::query!(
            r#"
            INSERT INTO refresh_tokens (user_id, hashed_token, expires_at)
            VALUES ($1, $2, $3)
            "#,
            user,
            hash,
            expires_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn check_refresh_token(&self, token: Refresh) -> Result<Uuid, AppError> {
        let mut hasher = Sha512::new();
        hasher.update(token.0);
        let hash = format!("{:x}", hasher.finalize());

        let user = sqlx::query!(
            r#"
            SELECT user_id FROM refresh_tokens
            WHERE hashed_token = $1 AND expires_at > NOW()
            "#,
            hash
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::Auth(AuthError::InvalidRefreshToken),
            _ => AppError::Db(DBError::DbError(e)),
        })?;

        sqlx::query!(
            r#"
            DELETE FROM refresh_tokens WHERE hashed_token = $1
            "#,
            hash
        )
        .execute(&self.pool)
        .await
        .map_err(DBError::DbError)?;

        Ok(user.user_id)
    }

    pub async fn get_user_refresh(&self, token: Refresh) -> Result<User, AppError> {
        let user_id = self.check_refresh_token(token).await?;
        Ok(self
            .get_user_uuid(&user_id)
            .await
            .map_err(|e| AppError::Db(e))?)
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
