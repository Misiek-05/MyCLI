use crate::cli_struct::{Cli};
use crate::enums_file::{Commands};
use std::fs::{self, File};

pub fn welcome(count: &Option<u8>) {
    if *count == None {
        println!("Welcome")
    } else {
        for _ in 0..count.unwrap() {
            println!("HI");
        }
    }
}

pub fn touch(name: &str, extension: &str) {
    let filename = format!("{}.{}", name, extension);

    match File::create(&filename) {
        Ok(_) => println!("Successfully created file: {}", filename),
        Err(e) => eprintln!("Error creating file {}: {}", filename, e),
    }
}

pub fn cat(filename: &str, amount: &Option<i32>) {
    match fs::read_to_string(filename) {
        Ok(message) => {
            if *amount == None {
                println!("{}", message);
            } else {
                let parts: Vec<&str> = message.split(' ').collect();
                let size = parts.len();
                
                if amount.unwrap() as usize > size {
                    println!("{} ", message);
                } else {
                    for i in 0..(amount.unwrap() - 1) as usize {
                        print!("{} ", parts[i]);
                    }
                }

            }
        }
        Err(e) => {
            eprintln!("Couldn't read file {}: {}", filename, e);
        }
    }
}