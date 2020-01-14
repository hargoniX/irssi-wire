use hyper::body::Buf;
use hyper::{
    client::connect::HttpConnector,
    {header, Body, Client, Method, Request}};
use hyper_tls::HttpsConnector;

use crate::net::wire_api::error::ApiError;
use crate::net::wire_api::auth::{LoginInfo, AuthResponse, Config};
use crate::net::wire_api::{
    self_info::SelfInfo,
    conversations::Conversations,
    members::Members,
};

#[derive(Debug)]
pub struct WireClient {
    pub user_agent: String,
    pub login_info: LoginInfo,
    pub auth_token: Option<AuthResponse>,
    pub client: Client<HttpsConnector<HttpConnector>>,
    pub config: Config,
    pub self_info: Option<SelfInfo>,
    pub conversations: Option<Conversations>,
    pub members: Option<Members>
}

impl WireClient {
    pub fn new(email: String, password: String, config: Config) -> WireClient {
        let https_client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());
        WireClient {
            user_agent: String::from("irssi"),
            login_info: LoginInfo { email, password },
            auth_token: None,
            client: https_client,
            config: config,
            self_info: None,
            conversations: None,
            members: None,
        }
    }

    pub async fn api_post<T>(
        &mut self,
        endpoint: String,
        body_content: String) -> Result<T, ApiError> 
    where T: serde::de::DeserializeOwned {
        let api_request = Request::builder()
            .method(Method::POST)
            .uri(endpoint)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::USER_AGENT, &self.user_agent)
            .header(header::AUTHORIZATION, self.auth_token.as_ref().unwrap().token_string())
            .body(Body::from(body_content))
            .unwrap();

        let api_response = self.client
            .request(api_request)
            .await
            .map_err(|e| ApiError::HttpError(e))?;

        let body = hyper::body::aggregate(api_response)
            .await
            .map_err(|e| ApiError::HttpError(e))?;

        let parsed_json = serde_json::from_reader(body.bytes())
            .map_err(|e| ApiError::JsonParseError(Box::new(e)))?;

        Ok(parsed_json)
    }

    pub async fn api_get<T>(
        &mut self,
        endpoint: String) -> Result<T, ApiError>
    where T: serde::de::DeserializeOwned {
        let api_request = Request::builder()
            .method(Method::GET)
            .uri([self.config.fetch().rest_url, endpoint].concat())
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::USER_AGENT, &self.user_agent)
            .header(header::AUTHORIZATION, self.auth_token.as_ref().unwrap().token_string())
            .body(Body::empty())
            .unwrap();

        let api_response = self.client
            .request(api_request)
            .await
            .map_err(|e| ApiError::HttpError(e))?;

        let body = hyper::body::aggregate(api_response)
            .await.map_err(|e| ApiError::HttpError(e))?;

        let parsed_json = serde_json::from_reader(body.bytes())
            .map_err(|e| ApiError::JsonParseError(Box::new(e)))?;

        Ok(parsed_json)
    }

    /*
    // TODO delete this
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
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::USER_AGENT, self.user_agent)
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
    */
}
