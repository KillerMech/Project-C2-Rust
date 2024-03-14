use std::thread::sleep;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    loop {
        sleep(std::time::Duration::from_secs(5));
        let mut res = reqwest::blocking::get("http://localhost:9001/agent_heartbeat").unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body)?;

        println!("Response: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());
        println!("Body:\n{}", body);
        
    }

    Ok(())
}
