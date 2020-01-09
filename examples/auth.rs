use irssi_wire::net::wire_api::auth::*;
use irssi_wire::net::wire_api::conversations::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = WireClient::new(
        String::from("marco_thomas@genua.de"),
        String::from("MarcMK1337#"),
        Config::Default,
    );

    let auth_response = client.authentification().await.unwrap();

    let conversations = client.fetch_conversations(&auth_response).await.unwrap();

    Ok(())
}
