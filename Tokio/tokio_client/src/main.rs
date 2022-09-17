use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::spawn;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let port = &args[1];
    let mut ip = "127.0.0.1:".to_string();
    ip += port;
    // println!("{ip:?}");
    for i in 1..8 {
        let mut stream = TcpStream::connect(ip.to_ascii_lowercase()).await.expect("connection failed");
        spawn(async move {
            println!("{stream:?}");
            loop {
                print!("Client{} enter your message: ", i);
                let mut input = "".to_string();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("read not performed");
                stream.write(input.as_bytes()).await.expect("Write Not Completed");
                
                if input.trim_end() == "exit" {
                    println!("Client{} Terminated", i);
                    break;
                }

                let mut server_message_byte = [0u8; 1000];
                stream.read(&mut server_message_byte).await.unwrap();
                let mut server_message_string = std::str::from_utf8(&server_message_byte).unwrap();
                server_message_string =
                    server_message_string.trim_matches(char::from(0)).trim_end();
                println!("Server reply to Client{} : {:?}", i, server_message_string);
            }
        });
    }
}
