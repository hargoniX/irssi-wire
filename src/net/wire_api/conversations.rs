use hyper::body::Buf;
use hyper::client::connect::HttpConnector;
use hyper::{header, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};

use crate::net::wire_api::auth::{AuthResponse, Config, WireClient};
use crate::net::wire_api::error::ApiError;

const CONTENT_TYPE: &str = "application/json";
const USER_AGENT: &str = "irssi";

#[derive(Serialize, Deserialize, Debug)]
pub struct Conversations {
    has_more: Option<bool>,
    conversations: Option<Vec<Conversation>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Conversation {
    access: Option<Access>,
    creator: Option<String>,
    access_role: Option<String>,
    members: Option<ConversationMembers>,
    name: Option<String>,
    team: Option<String>,
    id: Option<String>,
    r#type: Option<u32>,
    receipt_mode: Option<u32>,
    last_event_time: String,
    message_timer: Option<u32>,
    last_event: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Access {
    #[serde(rename = "0")]
    _0: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConversationMembers {
    #[serde(rename = "self")]
    self_: Option<Member>,
    others: Option<Vec<OtherMember>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Member {
    hidden_ref: Option<String>,
    stauts: Option<u32>,
    service: Option<ServiceRef>,
    otr_muted_ref: Option<String>,
    conversation_role: Option<String>,
    status_time: Option<String>,
    hidden: Option<bool>,
    status_ref: Option<String>,
    id: Option<String>,
    otr_archived: Option<bool>,
    otr_muted_status: Option<String>,
    otr_muted: Option<bool>,
    otr_archived_ref: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtherMember {
    status: Option<u32>,
    conversation_role: Option<String>,
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceRef {
    id: String,
    provider: String,
}

impl WireClient {
    pub async fn fetch_conversations(
        &mut self,
        auth_response: &AuthResponse,
    ) -> Result<Conversations, ApiError> {
        let endpoint = [
            self.config.fetch().rest_url,
            String::from("/conversations?size=500"),
        ]
        .concat();
        let auth_token = auth_response.token_string();

        let fetch_request = Request::builder()
            .method(Method::GET)
            .uri(endpoint)
            .header(header::CONTENT_TYPE, CONTENT_TYPE)
            .header(header::USER_AGENT, USER_AGENT)
            .header(header::AUTHORIZATION, auth_token)
            .body(Body::empty())
            .unwrap();

        let fetch_result = self
            .client
            .request(fetch_request)
            .await
            .map_err(|e| ApiError::HttpError(e))?;

        let fetch_body = hyper::body::aggregate(fetch_result)
            .await
            .map_err(|e| ApiError::HttpError(e))?;

        let fetch_response = serde_json::from_reader(fetch_body.bytes())
            .map_err(|e| ApiError::JsonParseError(Box::new(e)))?;

        Ok(fetch_response)
    }
}
