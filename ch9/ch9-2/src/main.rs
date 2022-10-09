extern crate core;

use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

fn main() {
    println!("ch9-2 start!!");

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        }
    };

    // let f = File::open("hello2.txt").unwrap();
    // let f = File::open("hello2.txt").expect("Failed to open hello2.txt");
    let result = read_username_from_file().expect("read file error");
    println!("read contents are \n{}", result);
    let result = read_username_from_file2("hello.txt").expect("read file error");
    println!("read contents are \n{}", result);
    let result = read_username_from_file3("hello.txt").expect("read file error");
    println!("read contents are \n{}", result);
}

fn read_username_from_file()-> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2(file_name: &str)-> Result<String, io::Error> {
    let mut f = File::open(file_name)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3(file_name: &str)-> Result<String, io::Error> {
    let mut s = String::new();

    File::open(file_name)?.read_to_string(&mut s)?;
    Ok(s)
}
