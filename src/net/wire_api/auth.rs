use hyper::body::Buf;
use hyper::client::connect::HttpConnector;
use hyper::{header, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};

use crate::net::wire_api::error::ApiError;

const USER_AGENT: &str = "irssi";
const CONTENT_TYPE: &str = "application/json";

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
    email: String,
    password: String,
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

pub struct WireClient {
    login_info: LoginInfo,
    pub client: Client<HttpsConnector<HttpConnector>>,
    pub config: Config,
}

impl WireClient {
    pub fn new(email: String, password: String, config: Config) -> WireClient {
        let https_client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());
        WireClient {
            login_info: LoginInfo { email, password },
            client: https_client,
            config: config,
        }
    }

    pub async fn authentification(&mut self) -> Result<AuthResponse, ApiError> {
        let endpoint = [
            self.config.fetch().rest_url,
            "/login?persist=true".to_string(),
        ]
        .concat();
        let json = serde_json::to_string(&self.login_info).unwrap();

        let auth_request = Request::builder()
            .method(Method::POST)
            .uri(endpoint)
            .header(header::CONTENT_TYPE, CONTENT_TYPE)
            .header(header::USER_AGENT, USER_AGENT)
            .body(Body::from(json))
            .unwrap();

        let auth_result = self
            .client
            .request(auth_request)
            .await
            .map_err(|e| ApiError::HttpError(e))?;

        let auth_body = hyper::body::aggregate(auth_result)
            .await
            .map_err(|e| ApiError::HttpError(e))?;

        let auth_response = serde_json::from_reader(auth_body.bytes())
            .map_err(|e| ApiError::JsonParseError(Box::new(e)))?;

        Ok(auth_response)
    }
}
