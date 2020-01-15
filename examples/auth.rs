use std::env;

use irssi_wire::net::wire_api::{auth::Config, wire_client::WireClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = WireClient::new(
        env::var("EMAIL").unwrap(),
        env::var("PW").unwrap(),
        Config::Default,
    );

    let _auth_response = client.authentification().await.unwrap();

    dbg!(client);

    Ok(())
}
