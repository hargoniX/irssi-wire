use serde::{Serialize, Deserialize};

use crate::net::wire_api::{error::ApiError};

#[derive(Serialize, Deserialize, Debug)]
pub struct Members {
    user: Option<String>,
    created_by: Option<String>,
    legalhold_status: Option<String>,
    created_at: Option<String>,
    permissions: Option<Permissions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Permissions {
    copy: Option<u32>,
    #[serde(rename = "self")]
    self_: Option<u32>,
}
