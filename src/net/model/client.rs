#![allow(non_camel_case_types)]
use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

 use crate::net::model::prekeys::{PreKey, LastPreKey};
// Client
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Client {
    pub id: String,
    pub class: Option<Class>,
    pub time: Option<DateTime<Utc>>,
    pub r#type: Option<ClientType>,
    pub cookie_label: Option<String>,
    pub label: Option<String>,
    pub model: Option<String>
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum Class {
    phone,
    tablet,
    desktop
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum ClientType {
    permanent,
    temporary
}

// PubClientView
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PubClientView {
    pub id: String,
    pub class: Class
}

// SignalingKeys
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignalingKeys {
    #[serde(serialize_with = "b64_encode", deserialize_with = "b64_decode")]
    pub enc: Vec<u8>,
    #[serde(serialize_with = "b64_encode", deserialize_with = "b64_decode")]
    pub mac: Vec<u8>
}

fn b64_encode<'a, S>(bytes: &'a Vec<u8>, serialzer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    serialzer.serialize_str(&base64::encode(&bytes))
}

fn b64_decode<'de, D>(deserialzer: D) -> Result<Vec<u8>, D::Error>
    where D: Deserializer<'de>
{
    use serde::de::Error;

    String::deserialize(deserialzer)
        .and_then(|string|
                  base64::decode(&string)
                  .map_err(|e| Error::custom(e.to_string()))
        )
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User2Clients {
    pub value: HashMap<Uuid, HashSet<String>>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientMismatch {
    pub time: DateTime<Utc>,
    pub redundant: User2Clients,
    pub missing: User2Clients,
    pub deleted: User2Clients
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegisterParams{
    pub prekeys: Vec<PreKey>,
    pub last_prekey: LastPreKey,
    pub sig_keys: SignalingKeys,
    pub ctype: ClientType,
    pub class: Class,
    pub cookie_label: String,
    pub label: Option<String>,
    pub password: Option<String>,
    pub model: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct DeleteParams {
    password: String
}
