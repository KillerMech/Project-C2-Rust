use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub fn start_server() {
    println!("Starting server..."); // Print a message to the console
    println!("Creating listener..."); 

    // Create the listener. Rust returns an error if the listener can't be created.
    // This is the listener for the server and handles client connections
    let listener = create_listener().expect("Failed to create listener");
    println!("Listener created successfully");
    for stream in listener.incoming() {
        let stream = stream.expect("Failed to accept connection");

        handle_client(stream);
    }

}

// This function handles the client connection
// It's important to note that the stream is mutable
fn handle_client(mut stream: TcpStream) {
    // TODO: Implement this function
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response = 
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    stream.shutdown(std::net::Shutdown::Both).unwrap();
}

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
