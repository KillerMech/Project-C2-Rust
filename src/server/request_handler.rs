
fn request_parser(request: &String) -> Result<&String, String> {
    if request == request {
        println!("Request: {}", request);
        Result::Ok(request)
    } else {
        println!("Error: {}", request);
        Result::Err("This didn't work".to_string())
    }
}
