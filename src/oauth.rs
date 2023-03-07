use reqwest::Client;

use std::collections::HashMap;

use crate::{models};

pub type Error<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;


pub const API_URL: &str = "https://discord.com/api/v10";



#[derive(Debug)]
pub struct OauthClient{
    client: Client,
    access_token: Option<String>,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

impl OauthClient{
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            client: Client::new(),
            client_id,
            access_token: None,
            client_secret: client_secret,
            redirect_uri: redirect_uri,
        }
    }

    /// Authorizes a code
    /// # Arguments
    /// * 'code' - A reference of a string that should be returned from an web server
    /// # Note
    /// this function sets the access_token paramater so unless you need to use other data returned, you should not need to put this in a variable
    #[allow(unused_must_use)]
    pub async fn authorize_code(&mut self, code: &String) -> Error<models::AuthorizedToken> {
        let json = self
            .client
            .post(format!("{}/oauth2/token", API_URL))
            .form(&models::AuthorizeTokenPayload {
                client_id: self.client_id.to_owned(),
                client_secret: self.client_secret.to_owned(),
                grant_type: "authorization_code".to_string(),
                code,
                redirect_uri: self.redirect_uri.to_owned(),
            })
            .send()
            .await?
            .json::<models::AuthorizedToken>()
            .await?;

        self.access_token = Some(json.access_token.to_string());

        Ok(json)
    }

    
    pub async fn refresh_token(&self, refresh_token: &String) -> Error<models::AuthorizedToken> {
        let json = self
            .client
            .post(format!("{}/oauth2/token", API_URL))
            .form(&models::RefreshTokenPayload {
                client_id: self.client_id.to_owned(),
                client_secret: self.client_secret.to_owned(),
                grant_type: "refresh_token".to_owned(),
                refresh_token: refresh_token,
            })
            .send()
            .await?
            .json::<models::AuthorizedToken>()
            .await?;

        Ok(json)
    }

    pub async fn fetch_current_user(&mut self) -> Error<models::User> {
        let json = self
            .client
            .get("https://discord.com/api/users/@me")
            .header("Authorization", format!("Bearer {}", self.access_token.as_ref().unwrap()))
            .send()
            .await?
            .json::<models::User>()
            .await?;

        Ok(json)
    }

    pub async fn fetch_user(
        &self,
        user_id: u64,
    ) -> Error<HashMap<String, String>> {
        let json = self
            .client
            .get(format!("{}/users/{}", API_URL, user_id))
            .header("Authorization", format!("Bearer {}", self.access_token.as_ref().unwrap()))
            .send()
            .await?
            .json::<HashMap<String, String>>()
            .await;

        Ok(json?)
    }
}
