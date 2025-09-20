use std::io::Read;
use std::net::{TcpListener, TcpStream};

// TODO: Optimize this
const BUF_LEN: usize = 1024;

// TODO: make this parameterized
const PORT: &str = "8080";
const IP: &str = "0.0.0.0";

fn pretty_print_buffer(buffer: &[u8]) {
    let s = match str::from_utf8(buffer) {
        Ok(v) => v,
        Err(e) => panic!("Tried to print invalid utf8 sequence {}", e),
    };
    println!("{}", s);
}

fn handle_client(mut stream: TcpStream) {
    // TODO: maybe this shoud be an http server?
    let mut buffer: [u8; BUF_LEN] = [0; BUF_LEN];
    let read_size;

    match stream.read(&mut buffer) {
        Ok(size) => read_size = size,
        Err(error) => panic!("{error:?}"),
    };

    println!("Received {} bytes", read_size);

    // Split into header/body
    // This is HTTP, so a \r\n\r\n divides the header from the body
    // Read until the first \r\n\r\n
    let mut i = 0;
    const WINDOW_SIZE: usize = 4;
    let mut window: [u8; WINDOW_SIZE] = [0; WINDOW_SIZE];
    while &window != b"\r\n\r\n" {
        let window_idx = i + WINDOW_SIZE;
        window = buffer[i..window_idx]
            .try_into()
            .expect("Received incorrect size slice");
        i += 1;
    }

    // i now points to the end of the header/start of the body
    let header = &buffer[0..i];
    let body = &buffer[i..];
    println!("Found header!");
    pretty_print_buffer(header);

    println!();
    println!("Found body!");
    pretty_print_buffer(body);

    println!("");
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
