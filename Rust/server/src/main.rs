use std::{net::TcpListener, io::{Read, Write}, env};
fn main() {
    let args:Vec<String> = env::args().collect();
    let port = &args[1];
    let mut ip = "127.0.0.1:".to_string();
    ip += port;
    println!("{ip:?}");

    let listener = TcpListener::bind(ip).unwrap(); //expect("binding failure");
    //let mut stream = listner.accept().expect("Failure in accepting");
    loop{
        println!("TCP server waiting for client request");
        match listener.accept() {
            Ok((mut stream, addr)) => {
                println!("new client: {addr:?}");
                println!("stream: {stream:?}");
                println!("accepted client request");
                loop{
                    let mut buf = [0u8; 1000];
                    stream.read(&mut buf).expect("read not completed");
                    let mut message = std::str::from_utf8(&buf).unwrap();
                    message = message.trim_matches(char::from(0)).trim_end();
                    println!("From Client : {:?}", message);
                    
                    if message == "exit" {
                        println!("Client disconnected from server");
                        break;
                    }
                    
                    let mut input = "".to_string();
                    std::io::stdin().read_line(&mut input).expect("read not performed");
                    stream.write(input.as_bytes()).expect("Write Not Completed");
                    
                    
                }
            },
            Err(e) => println!("couldn't get client: {e:?}"),
        }
    }
    
    /*
    for stream in listener.incoming(){
        match stream {
            Ok(mut stream) => {
                let mut buf = [0u8; 1000];
                stream.read(&mut buf).expect("read not completed");
                let mut message = std::str::from_utf8(&buf).unwrap();
                message = message.trim_matches(char::from(0)).trim_end();
                println!("{:?}", message);

                if message == "exit" {
                    break;
                }
                
                let mut input = "From Server : ".to_string();
                std::io::stdin().read_line(&mut input).expect("read not performed");
                stream.write(input.as_bytes()).expect("Write Not Completed");
                
            },
            Err(e) => println!("couldn't get client: {e:?}")
        }

    }
    */

    
}
