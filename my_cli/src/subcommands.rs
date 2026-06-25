use chrono::*;
use colored::*;
use std::{env, vec};
use std::fs::{self, File, OpenOptions};
use std::io::{Seek, SeekFrom, Read, Write};
use std::os::windows::fs::{MetadataExt};
use std::path::Path;
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

pub fn rm(filename: &str) {
    let path = Path::new(filename);
    
    if path.is_file() {
        match std::fs::remove_file(filename) {
            Ok(_) => {
                println!("File {} removed successfully", filename);
            }
            Err(e) => {
                eprintln!("Error while removing file: {}", e);
            }
        }
    } else if path.is_dir() {
        match std::fs::remove_dir(filename) {
            Ok(_) => {
                println!("directory {} removed successfully", filename)
            }
            Err(e) => {
                eprintln!("Error while removing directory: {}", e);
            }
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
    let max_lines = lines.unwrap_or(10);
    
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
    let max_lines = lines.unwrap_or(10);
    
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

pub fn echo(text: &str, maybe_file: Option<String>) {
    let file = maybe_file.as_deref().unwrap_or("No file");

    if file == "No file" {
        println!("{}", text);
    } else {
        match std::fs::write(file, text) {
            Ok(_) => {
                println!("Successfully")
            }
            Err(e) => {
                eprintln!("Error {}", e);
            }
        }
    }
}

pub fn cal() {
    let today = Local::now().date_naive();
    let current_year = today.year();
    let current_month = today.month();
    let current_day = today.day();

    let mut current_date = NaiveDate::from_ymd_opt(current_year, current_month, 1).expect("Invalid date components");

    println!("          {}", current_date.format("%b %Y"));
    let week: Vec<String> = vec!["Mon".to_string(), "Tue".to_string(), "Wed".to_string(), "Thu".to_string(), "Fri".to_string(), "Sat".to_string(), "Sun".to_string()];  
    for day in &week {
        print!("{} ", day);
    }
    println!();
    while current_date.month() == current_month {
        let formatted = current_date.format("%e-%a").to_string();
        let parts: Vec<&str> = formatted.split("-").collect();
        if parts[0] == " 1" {
            let mut i = 0;
            while parts[1].to_string() != week[i] {
                print!("    ");
                i+=1;
            }
        }
        
        if current_date.day() == current_day {
            print!(" {} ", parts[0].green());
        } else {
            print!(" {} ", parts[0]);
        }
        
        if parts[1] == "Sun" {
            println!();
        }

        current_date += Duration::days(1);
    }
}

