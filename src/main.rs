use std::fs::File;
use std::error::Error;
use std::env;
use std::path::Path;
use pdf_extract::extract_text;

fn main() -> Result<(), Box<dyn Error>> {
    // Get the path from the argument
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);

    // Extract the content text
    let text = extract_text(path)?;
    println!("Extracted text: {}", text);
    
    Ok(())
}