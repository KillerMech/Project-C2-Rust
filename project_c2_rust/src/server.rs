use std::io;
use std::net::{TcpListener, TcpStream};

pub fn start_server() {
    println!("Starting server..."); // Print a message to the console
    println!("Creating listener..."); 

    let listener = create_listener().expect("Failed to create listener");
    println!("Listener created successfully");
    for stream in listener.incoming() {
        let stream = stream.expect("Failed to accept connection");
        handle_client(stream);
    }

}

fn handle_client(stream: TcpStream) {
    // TODO: Implement this function
    println!("First test complete, closing connection");
    stream.shutdown(std::net::Shutdown::Both).expect("Failed to close connection");
}

fn create_listener() -> io::Result<TcpListener> {
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

