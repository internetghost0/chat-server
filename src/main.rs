use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// Address for server
const ADDR: &str = "127.0.0.1:8080";


#[tokio::main]
async fn main() {
    // start to listening
    let listener = TcpListener::bind(ADDR).await.unwrap();
    loop {
        // new connection
        let (mut socket, _addr) = listener.accept().await.unwrap();
        // create buffer to receiving bytes from user
        let mut buffer = [0u8; 1024];
        // to handle multiple clients simultaneously
        tokio::spawn(async move {
            loop {
                // receive bytes from user
                let size = socket.read(&mut buffer).await.unwrap();
                // send bytes to user
                socket.write_all(&buffer[..size]).await.unwrap();
            }
        });
    }
}

