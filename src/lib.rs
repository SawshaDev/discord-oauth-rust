mod models;
mod oauth;

pub use oauth::{OauthClient};

pub type Error<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;


pub const API_URL: &str = "https://discord.com/api/v10";