use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use url::{Url, ParseError};
use project_c2_rust::ThreadPool;

// Start server function
pub fn start_server() {
    println!("Starting server..."); // Print a message to the console
    println!("Opening webserver..."); 

    // Create the listener. Rust returns an error if the listener can't be created.
    // This is the listener for the server and handles client connections
    let listener = create_listener().expect("Failed to create listener");
    let pool = ThreadPool::new(4);

    println!("Webserver started, use browser to connect to http://localhost:{}/", listener.local_addr().unwrap().port());
    for stream in listener.incoming() {
        let stream = stream.expect("Failed to accept connection");

        pool.execute(|| {
            handle_client(stream);
        });
    }

}

// This function handles the client connection
// It's important to note that the stream is mutable
fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let requested_url = parse_requested_url(&request_line).unwrap();

    let (status_line, filename, content_type) = if request_line.starts_with("GET") {
        let content_type = get_content_type(&requested_url);
        ("HTTP/1.1 200 OK", requested_url, content_type)
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html".to_string(), "text/html".to_string())
    };

    if let Ok(contents) = fs::read(&filename) {
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {length}\r\n\r\n");
        stream.write(response.as_bytes()).unwrap();
        stream.write(&contents).unwrap();
    } else {
        println!("File not found: {}", filename);
        let contents = fs::read("./404.html").unwrap();
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {length}\r\n\r\n");
        stream.write(response.as_bytes()).unwrap();
        stream.write(&contents).unwrap();
    }

    stream.shutdown(std::net::Shutdown::Both).unwrap();
}
// TODO: Change function to accept listen_port and listen_ip as arguments
// This will allow the function to be used for opening other listeners
fn create_listener() -> std::io::Result<TcpListener> {
    let mut listen_port = "9001";
    let listen_ip = "0.0.0.0";
    let mut input = String::new();

    // Check if the user wants to change listen_port and listen_ip
    println!("Enter the listen port (default: 9001): ");
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    if input == "\n" {
        println!("Using default listen port: 9001");
    } else {
        listen_port = &input;
    }
    
    listen_port = listen_port.trim();
    // save the listener. The ? operator will return an error if the listener can't be created
    let listener = TcpListener::bind(format!("{}:{}", listen_ip, listen_port))?;
    Ok(listener)
}

fn parse_requested_url(request_dir: &String) -> Result<String, ParseError> {
    let mut base_url = String::new();
    let mut url_position = (0, 0);
    let mut first_char = '_';

    for (i, char) in request_dir.chars().enumerate() {
        if char == '/' && first_char != '/'{
            url_position.0 = i;
            first_char = char;
        } else if first_char == '/' && char == ' ' {
            url_position.1 = i;
            break;
        }
    }

    base_url = request_dir[url_position.0..url_position.1].to_string();

    if base_url == "/" {
        base_url = "index.html".to_string();
    } else {
        base_url = ".".to_string() + &base_url;
    }

    Ok(base_url)
}

fn get_content_type(filename: &str) -> String {
    if filename.ends_with(".html") {
        "text/html".to_string()
    } else if filename.ends_with(".css") {
        "text/css".to_string()
    } else if filename.ends_with(".js") {
        "application/javascript".to_string()
    } else if filename.ends_with(".ttf") {
        "font/ttf".to_string()
    } else if filename.ends_with(".woff2") {
        "font/woff2".to_string()
    } else if filename.ends_with(".woff") {
        "font/woff".to_string()
    } else if filename.ends_with(".eot") {
        "application/vnd.ms-fontobject".to_string()
    } else {
        "application/octet-stream".to_string()
    }
}
