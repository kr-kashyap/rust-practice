use std::fs;

fn main() {
    let res = fs::read_to_string("example.txt");

    match res {
        Ok(content) => {
            println!("Current file content \n{:?}", content);
        }
        Err(err) => {
            println!("Error reading Current file \n{:?}", err);
        } 
    }
}