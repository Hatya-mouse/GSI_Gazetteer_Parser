use std::env;
use lopdf::Document;
use anyhow::Result;

fn main() -> Result<()> {
    // Get the path from arguments
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    // Load the PDF document
    let doc = Document::load(path)?;

    // Process each page
    for page_num in 1..=doc.get_pages().len() {
        if let Ok(text) = extract_text_from_page(&doc, page_num.try_into().unwrap()) {
            println!("=== Page {} ===", page_num);
            println!("{}", text);
        }
    }

    Ok(())
}

fn extract_text_from_page(doc: &Document, page_num: u32) -> Result<String> {
    Ok(doc.extract_text(&[page_num])?)
}
