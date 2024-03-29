use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn manage_listener_file(request: &String, identifier: &String) -> Result<String, String> {
    if request == "new_listener" {
        let mut file = File::options()
            .append(true)
            .create(true)
            .open("listener.txt").unwrap();

        if (file.metadata().unwrap().len() as i64) == 0 {
            file.write(format!("{}", identifier).as_bytes()).unwrap();
            Result::Ok("Success".to_string())
        } else if (file.metadata().unwrap().len()) != 0 {
            file.write(format!(",{}",identifier).as_bytes()).expect("write failed");
            Result::Ok("Success".to_string())
        } else {
            Result::Err("Couldn't save new listener to file".to_string())
        }
    } else if request == "delete_listener" {
        let mut file = File::options()
            .write(true)
            .open("listener.txt").expect("cannot open file");


        file.write_all(identifier.as_bytes()).unwrap();
        Result::Ok("Success".to_string())
    } else if request == "get_listeners" {
        if Path::new("listener.txt").exists() {
            let file = fs::read_to_string("listener.txt").expect("No such file or directory");
            Result::Ok(file)
        } else {
            Result::Err("No listeners found".to_string())
        }        
    } else {
        Result::Err("This didn't work".to_string())
    }

}
