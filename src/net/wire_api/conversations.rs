// Dependencies
use serde::{Deserialize, Serialize};

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
