mod game_manager;
mod players;
mod server;
mod constants;

use crate::constants::SERVER_ADDRESS;

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind(SERVER_ADDRESS).await?;
    println!("Server listening on {SERVER_ADDRESS}...");
    



    Ok(())
}
