use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
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
    // TODO: Implement this function
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line.starts_with("GET / HTTP/1.1") {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = 
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write(response.as_bytes()).unwrap();
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

