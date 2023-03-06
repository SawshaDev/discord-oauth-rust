mod models;
mod oauth;

pub use oauth::{OauthClient};

pub type Error<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
