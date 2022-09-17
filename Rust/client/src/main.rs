use std::{
    io::{Read, Write},
    net::TcpStream, env,
};

fn main() {
    let args:Vec<String> = env::args().collect();
    let port = &args[1];
    let mut ip = "127.0.0.1:".to_string();
    ip += port;
    println!("{ip:?}");
    let mut stream = TcpStream::connect(ip).expect("connection failed");
    println!("{stream:?}");
    loop {
        let mut input = "".to_string();
        std::io::stdin()
            .read_line(&mut input)
            .expect("read not performed");
        stream.write(input.as_bytes()).expect("Write Not Completed");
        if input.trim_end() == "exit" {
            println!("Client Terminated");
            break;
        }
        let mut server_message_byte = [0u8; 1000];
        stream.read(&mut server_message_byte).unwrap();
        let mut server_message_string = std::str::from_utf8(&server_message_byte).unwrap();
        server_message_string = server_message_string.trim_matches(char::from(0)).trim_end();
        println!("From Server : {:?}", server_message_string);
    }
}
