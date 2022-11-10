use std::env;

use anyhow::Result;
use tokio::io::{self, AsyncBufReadExt};
use tonic_live::client::Client;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let username = env::var("USERNAME")?;

    let mut client = Client::new(username).await;

    client.login().await?;

    client.get_messages().await?;

    let mut stdin = io::BufReader::new(io::stdin()).lines();
    while let Ok(Some(line)) = stdin.next_line().await {
        // let (room, content) = line.split_at(line.find(':').unwrap());
        let (room, content) = ("default_room", line);
        client.send_message(room, content).await?;
    }
    Ok(())
}
