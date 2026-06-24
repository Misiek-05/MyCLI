use chrono::*;
use colored::*;
use std::env;
use std::fs::{self, File};
use std::io::{Seek, SeekFrom, Read};
use std::os::windows::fs::{FileExt, MetadataExt};
use crate::functions::{copy_to_clipboard};

pub fn welcome(count: &Option<u8>) {
    if *count == None {
        println!("Welcome")
    } else {
        for _ in 0..count.unwrap() {
            println!("HI");
        }
    }
}

pub fn touch(name: &str) {
    match File::create(&name) {
        Ok(_) => println!("Successfully created file: {}", name),
        Err(e) => eprintln!("Error creating file {}: {}", name, e),
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
            copy_to_clipboard(&path.display().to_string());
        }
        Err(e) => {
            eprintln!("Error while getting current dir: {}", e);
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
            eprintln!("Error while getting current dir: {}", e);
        }
    }
}

pub fn cp(source_name: &str, dest_name: &str) {
    match std::fs::copy(source_name, dest_name) {
        Ok(_) => {
            println!("Successfully copied file");
        }
        Err(e) => {
            eprintln!("Error during copy: {}", e);
        }
    }
}

pub fn mv(source_name: &str, dest_name: &str) {
    match std::fs::rename(source_name, dest_name) {
        Ok(_) => {
            println!("Successfully moved/rename file");
        }
        Err(e) => {
            eprintln!("Error during moving file: {}", e);
        }
    }
}

pub fn mkdir(dir_name: &str) {
    match std::fs::create_dir_all(dir_name) {
        Ok(_) => {
            println!("Successfully created directory: {}", dir_name);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

pub fn tail(filename: &str, lines: &Option<u8>) {
    let mut max_lines = lines.unwrap_or(10);
    
    let mut file = File::open(filename).unwrap();
    let length = file.metadata().unwrap().len();

    let mut pos = length;
    let mut finded_lines = 0;
    let mut buffer = [0u8; 1];

    while pos > 0 && finded_lines < max_lines {
        pos -= 1;
        file.seek(SeekFrom::Start(pos)).unwrap();
        file.read_exact(&mut buffer).unwrap();

        if buffer[0] == b'\n' {
            finded_lines += 1;
        }
    }

    file.seek(SeekFrom::Start(pos)).unwrap();
    let mut final_output = String::new();
    file.read_to_string(&mut final_output).unwrap();
    print!("{}", final_output);
}

pub fn head(filename: &str, lines: &Option<u8>) {
    let mut max_lines = lines.unwrap_or(10);
    
    let mut file = File::open(filename).unwrap();
    let length = file.metadata().unwrap().len();

    let mut pos = 0;
    let mut finded_lines = 0;
    let mut buffer = [0u8; 1];

    while pos < length && finded_lines < max_lines {
        file.seek(SeekFrom::Start(pos)).unwrap();
        file.read_exact(&mut buffer).unwrap();

        if buffer[0] == b'\n' {
            finded_lines += 1;
        }
        pos += 1;
    }

    file.seek(SeekFrom::Start(0)).unwrap();
    let mut final_output = String::new();
    file.take(pos).read_to_string(&mut final_output).unwrap();
    print!("{}", final_output);
}