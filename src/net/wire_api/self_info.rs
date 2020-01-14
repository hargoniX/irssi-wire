use serde::{Serialize, Deserialize};

use crate::net::wire_api::wire_client::WireClient;
use crate::net::wire_api::error::ApiError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelfInfo {
    email: Option<String>,
    handle: Option<String>,
    locale: Option<String>,
    managed_by: Option<String>,
    accent_id: Option<u32>,
    picture: Option<Vec<u32>>,
    name: Option<String>,
    pub team: Option<String>,
    id: Option<String>,
    assests: Option<Vec<UserAssets>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAssets {
    size: Option<String>,
    key: Option<String>,
    r#type: Option<String>,
}

impl WireClient {
    pub async fn fetch_self(&mut self) -> Result<(), ApiError> {
        self.self_info = Some(self
            .api_get(String::from("/self"))
            .await
            .unwrap());

        Ok(())
    }

    pub async fn fetch_conversations(&mut self) -> Result<(), ApiError> {
        self.conversations = Some(self
                                  .api_get(String::from("/conversations?size=500"))
                                  .await
                                  .unwrap());

        Ok(())
    }

    pub async fn fetch_members(&mut self) -> Result<(), ApiError>{
        self.members = Some(self
                            .api_get(String::from(
                                    [String::from("/teams/"),
                                    self.self_info.clone().unwrap().team.unwrap(),
                                    String::from("/members"),].concat()
                                                 ))
                            .await
                            .unwrap());

        Ok(())
    }
}
