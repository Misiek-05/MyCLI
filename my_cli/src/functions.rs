use arboard::Clipboard;
use chrono::{Datelike, Duration, NaiveDate};
use colored::Colorize;
use std::io::{stdout};
use crossterm::{cursor, execute};

pub fn copy_to_clipboard(text: &str) {
    match Clipboard::new() {
        Ok(mut clipboard) => {
            match clipboard.set_text(text) {
                Ok(_) => {
                    println!("Successfully put in Clipboard");
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

pub fn print_month(date: &mut NaiveDate, current_day: &u32, current_month: &u32, current_year: &i32, week: &Vec<String>, n: &u16) -> Result<(), std::io::Error> {
    let month_to_print = date.month();
    let mut stdout = stdout();

    if *n > 0 {
        execute!(stdout, cursor::MoveUp(7))?;
    }

    execute!(stdout, cursor::MoveRight(*n))?;
    println!("          {}", date.format("%b %Y"));
    execute!(stdout, cursor::MoveRight(*n))?;
    
    for day in week {
        print!("{} ", day);
    }
    println!();
    execute!(stdout, cursor::MoveRight(*n))?;

    while date.month() == month_to_print {
        let formatted = date.format("%e-%a").to_string();
        let parts: Vec<&str> = formatted.split("-").collect();
        if parts[0] == " 1" {
            let mut i = 0;
            while parts[1].to_string() != *week[i] {
                print!("    ");
                i+=1;
            }
        }
        
        if date.day() == *current_day && date.month() == *current_month && date.year() == *current_year {
            print!(" {} ", parts[0].green().bold());
        } else {
            print!(" {} ", parts[0]);
        }
        
        if parts[1] == "Sun" {
            println!();
            execute!(stdout, cursor::MoveRight(*n))?;
        }

        *date = *date + Duration::days(1);
    }
    println!();
    Ok(())
}