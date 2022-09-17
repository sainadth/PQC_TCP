use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::spawn;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let port = &args[1];
    let mut ip = "127.0.0.1:".to_string();
    ip += port;
    // println!("{ip:?}");
    let listener = TcpListener::bind(ip).await.expect("Failed to bind IP");
    // println!("{:?}", listener.ttl());
    loop {
        let (mut stream, _addr) = listener.accept().await.expect("Failed to accept the client request");
        spawn(async move {
            // println!("new client: {addr:?}");
            // println!("stream: {stream:?}");
            // println!("accepted client request");
            loop {
                //pickable to fetch size of buffer
                let mut buf = [0u8; 1000];
                stream.read(&mut buf).await.expect("read not completed");
                let mut message = std::str::from_utf8(&buf).unwrap();
                message = message.trim_matches(char::from(0)).trim_end();
                println!("From Client : {:?}", message);

                if message == "exit" {
                    println!("Client disconnected from server");
                    break;
                }

                let mut input = "".to_string();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("read not performed");
                stream
                    .write(input.as_bytes())
                    .await
                    .expect("Write Not Completed");
            }
        });
    }
}
