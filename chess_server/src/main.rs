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
    
    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            match socket.read(&mut buffer).await {
                Ok(n) if n > 0 => {
                    println!("Received: {:?}", &buffer[..n]);
                    socket.write_all(b"Message received").await.unwrap();
                }
                Ok(_) => println!("Connection closed by client"),
                Err(e) => eprintln!("Error reading from socket: {}", e),
            }
        });
    }
}
