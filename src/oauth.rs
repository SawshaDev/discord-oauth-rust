use reqwest::Client;

use std::{collections::HashMap};

use crate::{models::{User, AuthorizeTokenPayload, AuthorizedToken}, Error};



#[derive(Debug)]
pub struct OauthClient<'a>{
    client: Client,
    client_id: &'a String,
    client_secret: &'a String,
    redirect_uri: &'a String
}


impl<'a> OauthClient<'a> {
    pub fn new(client_id: &'a String, client_secret: &'a String, redirect_uri: &'a String) -> Self {
        OauthClient {
            client: Client::new(),
            client_id,
            client_secret: client_secret,
            redirect_uri: redirect_uri     
        }
    }

    pub async fn authorize_token(&self, code: String) -> Error<AuthorizedToken> {
        println!("{}", self.client_id.to_owned());

        let json = self
            .client
            .post("https://discord.com/api/v10/oauth2/token")
            .form(&AuthorizeTokenPayload {
                client_id: self.client_id.to_owned(),
                client_secret: self.client_secret.to_owned(),
                grant_type: "authorization_code".to_string(),
                code,
                redirect_uri: self.redirect_uri.to_owned(),
            })
            .send()
            .await? 
            .json::<AuthorizedToken>()
            .await?;

        Ok(json)
    }

    pub async fn fetch_current_user(&self, code: String) -> Error<User> {
        let json = self
            .client
            .get("https://discord.com/api/users/@me")
            .header("Authorization", format!("Bearer {code}"))
            .send()
            .await?
            .json::<User>()
            .await;

        Ok(json?)
    }

    pub async fn fetch_user(&self, code: String, _user_id: u64) -> Error<HashMap<String, String>> {
        let json = self
            .client
            .get("https://discord.com/api/v10/users/439095710810505226")
            .header("Authorization", format!("Bearer {code}"))
            .send()
            .await?
            .json::<HashMap<String, String>>()
            .await;

        Ok(json?)
    }
}
