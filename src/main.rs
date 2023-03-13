use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};
use tokio;

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();

    // default configuration is to join chat as anonymous.
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
    let channel_name= "ciubix8513";
    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            tracing::info!("Received message: {:?}", message);
        }
    });

    client.join(channel_name.to_owned()).unwrap();
    client.say(channel_name.to_owned(), "test".to_owned()).await.unwrap();

    join_handle.await.unwrap();
} 
