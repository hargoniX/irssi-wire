#![allow(non_camel_case_types)]
use std::collections::HashMap;

use proteus::keys::{IdentityKey, PreKeyBundle};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PreKeyMap {
    pub value: HashMap<Uuid, HashMap<String, Option<PreKey>>>
}

impl PreKeyMap {
    pub fn new() -> PreKeyMap {
        PreKeyMap {
            value: HashMap::new()
        }
    }

    pub fn get(&self, id: &Uuid) -> Option<&HashMap<String, Option<PreKey>>> {
        self.value.get(id)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PreKey {
    pub key: PreKeyBundle
}

impl PreKey {
    pub fn last_resort(k: &IdentityKey) -> LastPreKey {
        let pk = proteus::PreKey::last_resort();
        LastPreKey(PreKey { key: PreKeyBundle::new(k.clone(), &pk) })
    }

    pub fn from_str(s: &str) -> Option<PreKey> {
        base64::decode(s).ok().and_then(|xs| {
              PreKeyBundle::deserialise(xs.as_slice()).ok().map(|k| PreKey { key: k })
          })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LastPreKey(PreKey);

impl LastPreKey {
     pub fn new(p: PreKey) -> Option<LastPreKey> {
         if p.key.prekey_id == proteus::MAX_PREKEY_ID {
             Some(LastPreKey(p))
         } else {
             None
         }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientPreKey {
    pub id: String,
    pub key: PreKey
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientPreKeys(Vec<ClientPreKey>);
