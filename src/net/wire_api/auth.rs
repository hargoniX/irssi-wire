// Dependencies
use hyper::{
    body::Buf,
    {header, Body, Method, Request},
};
use serde::{Deserialize, Serialize};

use crate::net::wire_api::{error::ApiError, wire_client::WireClient};

#[derive(Debug, Clone)]
pub struct ConnectionUrls {
    pub websocket: String,
    pub rest_url: String,
}

#[derive(Debug, Clone)]
pub enum Config {
    Default,
    Custom(ConnectionUrls),
}

impl Config {
    pub fn fetch(&mut self) -> ConnectionUrls {
        match self {
            Config::Default => ConnectionUrls {
                websocket: String::from("wss://prod-nginz-ssl.wire.com"),
                rest_url: String::from("https://prod-nginz-https.wire.com"),
            },
            Config::Custom(urls) => urls.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthResponse {
    expires_in: u32,
    token_type: String,
    user: String,
    access_token: String,
}

impl AuthResponse {
    pub fn token_string(&self) -> String {
        [&self.token_type, " ", &self.access_token].concat()
    }
}

// TODO move auth here
impl WireClient {
    pub async fn authentification(&mut self) -> Result<(), ApiError> {
        let endpoint = [
            self.config.fetch().rest_url,
            String::from("/login?persist=true"),
        ]
        .concat();

        let json = serde_json::to_string(&self.login_info).unwrap();

        let auth_request = Request::builder()
            .method(Method::POST)
            .uri(endpoint)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::USER_AGENT, &self.user_agent)
            .body(Body::from(json))
            .unwrap();

        let auth_response = self
            .client
            .request(auth_request)
            .await
            .map_err(|e| ApiError::HttpError(e))?;

        let body = hyper::body::aggregate(auth_response)
            .await
            .map_err(|e| ApiError::HttpError(e))?;

        let parsed_json = serde_json::from_reader(body.bytes())
            .map_err(|e| ApiError::JsonParseError(Box::new(e)))?;

        self.auth_token = Some(parsed_json);

        Ok(())
    }
}
