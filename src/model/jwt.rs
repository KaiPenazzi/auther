use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct JWT {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub roles: Option<Vec<String>>,
    pub iat: u64,
    pub exp: u64,
}
