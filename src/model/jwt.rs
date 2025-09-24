use base64::{Engine, engine::general_purpose};
use rand_core::{OsRng, RngCore};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tokens {
    pub jwt: JWT,
    pub refresh: Refresh,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWT(pub String);
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Refresh(pub String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub roles: Option<Vec<String>>,
    pub iat: u64,
    pub exp: u64,
}

pub fn generate_refresh_token() -> Refresh {
    let mut bytes = [0u8; 64];
    OsRng.fill_bytes(&mut bytes);
    Refresh(general_purpose::URL_SAFE_NO_PAD.encode(&bytes))
}
