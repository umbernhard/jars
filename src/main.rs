use std::io::Read;
use std::net::{TcpListener, TcpStream};

const PORT: &str = "8080";
const IP: &str = "0.0.0.0";

fn handle_client(mut stream: TcpStream) {
    // 20-byte buffer
    // TODO: handle more than 20 bytes at a time
    // TODO: maybe this shoud be an http server?
    let mut buffer = [0; 20];
    let read_size;

    // TODO: do stuff with data (echo to start?)
    match stream.read(&mut buffer) {
        Ok(size) => read_size = size,
        Err(error) => panic!("{error:?}"),
    };

    println!("Received {} bytes", read_size);

    if read_size > 0 {
        // TODO use buffer
        for i in 0..read_size {
            println!("Data in buffer: {}", buffer[i] as char);
        }
        println!("");
    } else {
        panic!("No data!");
    }
}

fn main() -> std::io::Result<()> {
    // TODO: read in address, port, directory(?) to use off command line

    // TODO: open/listen on port (fd?)
    // TODO: Make this lower level
    let listener = TcpListener::bind(format!("{}:{}", IP, PORT))?;

    // TODO: read data off fd
    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
