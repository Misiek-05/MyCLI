use crate::cli_struct::{Cli};
use crate::enums_file::{Commands};
use chrono::*;
use colored::*;
use std::env;
use std::fs::{self, File};
use std::os::windows::fs::MetadataExt;

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
                    for i in 0..(amount.unwrap()) as usize {
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

pub fn pwd() {
    match env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
        }
        Err(e) => {
            eprintln!("Error while getting current dir: {}", e)
        }
    }
}

pub fn ls() {
    match env::current_dir() {
        Ok(path) => {

            match fs::read_dir(&path) {
            Ok(entries) => {
                println!("Current directory: {}", &path.display());
                for entry in entries {
                    let file = entry.unwrap();

                    let file_name = file.file_name();                    
                    let file_type = file.file_type().unwrap();
                    let file_metadata = file.metadata().unwrap();

                    let time_modified: DateTime<Local> = file_metadata.modified().unwrap().into();
                    let name = file_name.to_string_lossy();

                    if file_type.is_dir() { print!("{} ", name.blue().bold()) }
                    else if file_type.is_file() { print!("{} ", name.green()) }
                    else { print!("{} ", name); }
                    
                    println!(
                        "{} bytes | Permissions: {:?} | Modified: {:?}", 
                        file_metadata.len().to_string(), 
                        file_metadata.file_attributes(), 
                        time_modified.format("%d %b %Y %H:%M:%S").to_string()
                    );
                }
            }
            Err(e) => eprintln!("Error reading files: {}", e),
        }
        }
        Err(e) => {
            eprintln!("Error while getting current dir: {}", e)
        }
    }
}