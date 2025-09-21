use std::fs::read_to_string;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

// TODO: Optimize this
const BUF_LEN: usize = 1024;

// TODO: make this parameterized
const PORT: &str = "8080";
const IP: &str = "0.0.0.0";

enum ResponseCode {
    Okay,               // 200
    ServerError,        //500
    NotFound,           // 404
    NotImplemented,     //501
    UnsupportedVersion, // 505
}

// TODO: this feels gross? Maybe move to it's own file or figure out a better
// way to do it.
impl ResponseCode {
    pub fn numeric(&self) -> u16 {
        match self {
            ResponseCode::Okay => 200,
            ResponseCode::NotFound => 404,
            ResponseCode::ServerError => 500,
            ResponseCode::NotImplemented => 501,
            ResponseCode::UnsupportedVersion => 505,
        }
    }
}

fn clean_path(path: &str) -> String {
    let mut cleaned_path: &str = path.trim_start_matches("/");
    println!("Removed leading slash {}", cleaned_path);
    // TODO: clean off double dots, extra slashes, etc.
    // TODO: Figure out working directory?
    if cleaned_path == "" {
        // TODO look for index.html?
        cleaned_path = "index.html";
    }

    return cleaned_path.to_string();
}

/*
* Parse an HTTP Request Header
*
* If we've parsed successfully, return the path for the request.
* Else, return the relevant error code
*/
fn parse_header(buffer: &str) -> Result<(String, ResponseCode), ResponseCode> {
    let mut lines = buffer.lines();

    if let Some(request_line) = lines.next() {
        let parts: Vec<&str> = request_line.split_whitespace().collect();

        if parts.len() == 3 {
            let method = parts[0];
            let path = parts[1].to_string();
            let version = parts[2];

            if method != "GET" {
                println!("Unsupported HTTP: {}", method);
                return Err(ResponseCode::NotImplemented);
            }

            if version != "HTTP/1.1" {
                println!("Only HTTP/1.1 supported; got {}", version);
                return Err(ResponseCode::UnsupportedVersion);
            }

            let cleaned_path = clean_path(&path);
            // Check that path exists
            // TODO: Does it make sense to do this here?
            if !Path::new(&cleaned_path).exists() {
                println!("File not found!");
                return Err(ResponseCode::NotFound);
            }
            return Ok((cleaned_path, ResponseCode::Okay));
        } else {
            println!("Badly formatted HTTP request string");
            return Err(ResponseCode::ServerError);
        }
    } else {
        println!("Badly formatted HTTP request");
        return Err(ResponseCode::ServerError);
    }
}

fn build_response_header(status_code: &ResponseCode) -> String {
    return format!("HTTP/1.1 {} OK", status_code.numeric());
}

fn build_response_body(status_code: &ResponseCode, path: String) -> String {
    let response: String;

    println!("Trying to open {}", path);
    match status_code {
        ResponseCode::Okay => {
            // TODO: parse the path and see if there's a file to serve there
            // TODO: handle case if path is a directory
            match read_to_string(path) {
                Ok(string) => response = string,
                Err(error) => {
                    println!("Error!: {}", error);
                    // TODO Figure out what to do here...
                    response = format!("<h2>Error {}: something went wrong!</h2>", error);
                }
            }
        }
        _ => {
            response = format!(
                "<h2>Error {}: something went wrong!</h2>",
                status_code.numeric()
            );
        }
    }

    return response;
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer: [u8; BUF_LEN] = [0; BUF_LEN];
    let read_size;

    match stream.read(&mut buffer) {
        Ok(size) => read_size = size,
        Err(error) => panic!("{error:?}"),
    };

    println!("Received {} bytes", read_size);

    // Now convert the buffer to a string to make it easier to work with
    // TODO: How do we handle requests that are >BUF_LEN?
    let string_buf = match str::from_utf8(&buffer) {
        Ok(v) => v,
        Err(e) => panic!("Tried to print invalid utf8 sequence {}", e),
    };

    let split: Vec<&str> = string_buf.split("\r\n\r\n").collect();

    let header: &str;
    let body: &str;

    if split.len() == 2 {
        header = split[0];
        body = split[1];
    } else {
        println!("Unexpected request found: {} parts", split.len());
        return;
    }

    // i now points to the end of the header/start of the body
    println!("Found header!");
    println!("{}", header);

    // Parse the header
    // TODO: Do something with the error
    let status_code: ResponseCode;
    let mut path = String::new();
    match parse_header(header) {
        Ok(response) => {
            path = response.0;
            status_code = response.1;
        }
        Err(error) => {
            status_code = error;
        }
    };

    println!();
    println!("Found body!");
    println!("{}", body);

    // Parse the body?

    println!("");

    // Send response
    let response_header = build_response_header(&status_code);
    println!("Returning path {:?}", path);
    println!("Response header: {}", response_header);

    let response_body = build_response_body(&status_code, path);

    let response = format!("{}\r\n\r\n{}\r\n\r\n", response_header, response_body);
    match stream.write(&response.as_bytes()) {
        Ok(result) => result,
        Err(error) => panic!("Couldn't send response {}", error),
    };
}

fn main() -> std::io::Result<()> {
    // TODO: read in address, port, directory(?) to use off command line

    // TODO: Make this lower level
    let listener = TcpListener::bind(format!("{}:{}", IP, PORT))?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
