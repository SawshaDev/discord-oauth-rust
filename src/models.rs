use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizeTokenPayload<T> {
    pub client_id: String,
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
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshTokenPayload<T> {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
    pub refresh_token: T,
}
