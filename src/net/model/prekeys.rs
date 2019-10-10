#![allow(non_camel_case_types)]
use std::collections::HashMap;

use proteus::keys::PreKeyBundle;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PreKeyMap {
    pub value: HashMap<Uuid, HashMap<String, Option<PreKey>>>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PreKey {
    pub key: PreKeyBundle
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LastPreKey(PreKey);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientPreKey {
    pub id: String,
    pub key: PreKey
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientPreKeys(Vec<ClientPreKey>);
