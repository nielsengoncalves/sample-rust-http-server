use std::net::{TcpListener, TcpStream};
use std::io::Read;

pub struct Server {
    address: String
}

impl Server {

    pub fn new(address: String) -> Self {
        Self { address }
    }
    
    pub fn run(self) {
        let listener = TcpListener::bind(&self.address).unwrap();
        println!("Listening TCP connections on `{}`", self.address);

        loop {
            self.listen(&listener);
        }
    }

    fn listen(&self, listener: &TcpListener) {
        match listener.accept() {
            Ok((mut tcp_stream, _)) => {
                self.read(&mut tcp_stream);
            },
            Err(e) => {
                println!("Failed to establish a connection: {}", e);
            }
        }
    }

    fn read(&self, tcp_stream: &mut TcpStream) {
        let mut buffer = [0; 1024];
        match tcp_stream.read(&mut buffer) {
            Ok(_) => {
                println!("Received a request: {}", String::from_utf8_lossy(&buffer));
            }
            Err(e) => {
                println!("Failed to read from connection: {}", e);
            }
        }
    }
}