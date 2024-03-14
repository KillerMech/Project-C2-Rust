use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    collections::HashMap,
    sync::{Arc, Mutex},
};
use url::ParseError;
use project_c2_rust::ThreadPool;
mod request_handler;

struct AgentListener {
    listen_type: String,
    port: String,
    listener: TcpListener,
}

type AgentPool = Arc<Mutex<HashMap<String, AgentListener>>>;

// Start server function
pub fn start_server() {
    let mut port = "9001";
    let mut input = String::new();

    println!("Starting server..."); // Print a message to the console

    // Check if the user wants to change listen_port and listen_ip
    println!("Enter the listen port (default: 9001): ");
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    if input == "\n" {
        println!("Using default listen port: 9001");
    } else {
        port = &input;
    }

    println!("Opening webserver..."); 
    // Create the listener. Rust returns an error if the listener can't be created.
    // This is the listener for the server and handles client connections
    let listener = create_listener("0.0.0.0".to_string(), port.to_string()).expect("Failed to create listener");
    let pool = ThreadPool::new(4);

    println!("Webserver started, use browser to connect to http://localhost:{}/", listener.local_addr().unwrap().port());
    for stream in listener.incoming() {
        let stream = stream.expect("Failed to accept connection");

        // Create a thread for each client connection
        pool.execute(|| {
            handle_client(stream);
        });
    }
}

// This function handles the client connection
// It's important to note that the stream is mutable
fn handle_client(mut stream: TcpStream) {
    // Create a buffer reader to read the stream
    let mut buf_reader = BufReader::new(&mut stream);
    let mut line_buffer = Vec::new();
    buf_reader.read_until(b'\n', &mut line_buffer).unwrap();
    let (mut status_line, mut filename, mut content_type) = (String::new(), String::new(), String::new());
    let request_line = String::from_utf8(line_buffer.clone()).unwrap().trim().to_string();

    // Parse the requested URL
    let requested_url = parse_requested_url(&request_line).unwrap();
    println!("Request line: {}", request_line);

    // Check if the request is a GET request

    if request_line.starts_with("GET /agent_heartbeat") {
        println!("Agent heartbeat received");
        (status_line, filename) = ("HTTP/1.1 200 OK".to_string(), requested_url);
    } else if request_line.starts_with("GET") {
        // Get the content type of the requested file
        content_type = get_content_type(&requested_url);
        (status_line, filename) = ("HTTP/1.1 200 OK".to_string(), requested_url);
    }

    if request_line.starts_with("POST /new_listener") {
        let mut headers_end = false;
        let mut content_length: i32 = 0;

        while !headers_end {
            let mut line_buffer = Vec::new();
            buf_reader.read_until(b'\n', &mut line_buffer).unwrap();
            let line = String::from_utf8(line_buffer.clone()).unwrap();

            if line == "\r\n" {
                headers_end = true;
            } else {
                if line.starts_with("Content-Length") {
                    let mut length_str = line.trim_start_matches("Content-Length: ");
                    length_str = length_str.trim();
                    content_length = length_str.parse().unwrap();
                }
            }
        }

        let mut post_body = String::new();
        let bytes_read_total = 0;

        if content_length != 0 {
            while bytes_read_total < content_length {
                loop {
                    let mut buffer = [0; 1024];
                    let bytes_read = buf_reader.read(&mut buffer).unwrap();

                    if bytes_read_total >= content_length {
                        break;
                    }
                  post_body.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
                  break;
                }
                break;
            }
        }
        // I had to create my own form parser because every crate I found
        // was too complicated for what we are doing here.
        if post_body != "" {
            // Carve out the name field for the type of listener.
            let mut name_position = post_body.find("name\":\"").unwrap();
            let mut name_field = &post_body[name_position+7..];
            name_position = name_field.find("\"").unwrap();
            name_field = &name_field[..name_position];

            // Carve out the port field for the listener.
            let mut port_position = post_body.find("port\":\"").unwrap();
            let mut port_field = &post_body[port_position+7..];
            port_position = port_field.find("\"").unwrap();
            port_field = &port_field[..port_position];
            
            create_agent_listener(port_field.to_string(), name_field.to_string());
        }
    }

        

    // Read the file at the directory location
    if let Ok(contents) = fs::read(&filename) {
        let length = contents.len();
        // Create the response
        let response = format!("{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {length}\r\n\r\n");
        // Write the response to the stream
        stream.write(response.as_bytes()).unwrap();
        // Write the file contents to the stream
        stream.write(&contents).unwrap();
    } else {
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
pub fn create_listener(ip_address: String, port: String) -> std::io::Result<TcpListener>{
    let listen_port = port.trim();
    let listen_ip = ip_address;

    //listen_port = listen_port.trim();
    // save the listener. The ? operator will return an error if the listener can't be created
    let listener = TcpListener::bind(format!("{}:{}", listen_ip, listen_port))?;
    Ok(listener)
}

// TODO: Listener types need to be added in the future. Currently only support
// unencrypted TCP/http listeners.
fn create_agent_listener(port: String, listener_type: String) {
    let listen_ip = "0.0.0.0";
    let listen_port = port;
    let pool = ThreadPool::new(5);
    //let agent_pool = AgentPool::new(Mutex::new(HashMap::new()));

    //let new_listener = create_listener(listen_ip.to_string(), listen_port.to_string()).expect("Failed to create listener");
    if let Ok(new_listener) = create_listener(listen_ip.to_string(), listen_port.to_string()) {
        println!("Agent listener started on port: {}", listen_port);

        for stream in new_listener.incoming() {
            let stream = stream.expect("Failed to accept connection");

            // Create a thread for each client connection
            pool.execute(|| {
                handle_client(stream);
            });
        }
    } else {
        println!("Failed to create listener on port: {}", listen_port);
    }


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
