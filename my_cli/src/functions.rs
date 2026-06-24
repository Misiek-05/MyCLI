use arboard::Clipboard;

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