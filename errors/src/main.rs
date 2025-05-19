use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
fn main() {
    // let file = File::open("hello.txt");
    // let file = match file {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(er) => panic!("Error creating the file: {:?}", er),
    //         },
    //         other_error => {
    //             panic!("Error opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    let mut content = read_username_from_file();
    match content {
        Ok(c) => println!("Content: {}", c),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
