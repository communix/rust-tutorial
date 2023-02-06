//! # Rust tutorial 0008 - Error handling
//! Jay Kim (changjae.kim@gmail.com)
//! 2023 version 0.0.1

// Result enum. T is type and E is error.
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::File;
use std::io::ErrorKind; // This is needed to handle different error type
use std::io::{self, Read};

fn main() {
    // disable them because of panic.
    // tutorial_error_unwrap();
    // tutorial_error_match();
    let mut file_result = tutorial_return_result();
    match file_result {
        Ok(file) => (), // Doing nothing
        Err(e) => println!("Error: {}", e),
    }
    file_result = tutorial_question();
    match file_result {
        Ok(file) => (), // Doing nothing
        Err(e) => println!("Error: {}", e),
    }
}

fn tutorial_error_match() {
    let input_file_result = File::open("hello.txt"); // The return is type of Result<File> https://doc.rust-lang.org/std/fs/struct.File.html
    let input_file = match input_file_result {
        Ok(file) => file,
        Err(e) => panic!("Trigger panic!!! due to {}", e), // panic! macro trigger panic. // This code works
        // Below code is not work. Need to understand why.
        // Err(e) => match e.kind() { // Some errors have detailed explanations: E0308, E0425 at this line. expected struct `File`, found `()`
        //     ErrorKind::NotFound => println!("File not found: {}", e),
        //     other => {
        //         println!("Error: {}", other);
        //         panic!("Trigger panic!!!"); // panic! macro trigger panic.
        //     },
        // },
    };
}

fn tutorial_error_unwrap() {
    let input_file = File::open("hello.txt").unwrap(); // unwrap method return file or panic if there is an error.
}

fn tutorial_return_result() -> Result<File, io::Error> {
    let input_file_result = File::open("hello.txt");
    match input_file_result {
        Ok(file) => Ok(file), // return it
        Err(e) => Err(e),     // return it
    }
}

fn tutorial_question() -> Result<File, io::Error> {
    let file = File::open("hello.txt")?; // ? means return error if there is an error.
    println!("Does this code run?");
    return Ok(file);
}
// we can use ? for Result<> and Option<>