use std::fs::{self,File};
use std::io::{self,Read,ErrorKind};

fn read_file_worst() -> Result<String,io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e)
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

fn read_file_better() -> Result<String,io::Error> {

    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}


fn read_file_even_better() -> Result<String,io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_file() -> Result<String,io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_character_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    // let v = vec![1, 2, 3];
    // v[99];
    // panic!("Crash and Burn");
    read_file_worst();
    read_file_better();
    read_file_even_better();
    read_file();
}
