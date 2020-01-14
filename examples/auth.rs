use std::env;

use irssi_wire::net::wire_api::{
    wire_client::WireClient,
    auth::Config,
    conversations::Conversations,
    members::Members,
    self_info::SelfInfo,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = WireClient::new(
        env::var("EMAIL").unwrap(),
        env::var("PW").unwrap(),
        Config::Default,
    );

    let auth_response = client
        .authentification()
        .await
        .unwrap();

    dbg!(client);

    Ok(())
}
