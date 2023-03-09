use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizeTokenPayload<T> {
    pub client_id: u64,
    pub client_secret: String,
    pub grant_type: String,
    pub code: T,
    pub redirect_uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizedToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub discriminator: String,
    pub display_name: Option<String>
    
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshTokenPayload {
    pub client_id: u64,
    pub client_secret: String,
    pub grant_type: String,
    pub refresh_token: String,
}
