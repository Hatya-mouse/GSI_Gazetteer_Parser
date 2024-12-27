use std::env;
use std::fs::File;
use std::io::Write;
use lopdf::Document;
use anyhow::{Result, Context};

/// Main function to parse and extract text from a PDF file
/// starting from a specified page number.
fn main() -> Result<()> {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Validate command line arguments
    if args.len() < 3 {
        println!("Usage: {} <pdf_path> <start_page>", args[0]);
        std::process::exit(1);
    }

    // Extract path and starting page number from arguments
    let path = &args[1];
    let start_page: u32 = args[2]
        .parse()
        .context("Failed to parse start page number")?;

    // Load the PDF document
    let doc = Document::load(path)?;
    let pages = doc.get_pages();
    
    // Display basic information about the document
    println!("Total pages: {}", pages.len());
    println!("Starting from page: {}", start_page);

    let mut japanese_names: Vec<String> = Vec::new();
    let mut english_names: Vec<String> = Vec::new();

    // Iterate through all pages in the document
    for (page_num, _) in pages.iter() {
        // Skip pages before the specified start page
        if *page_num < start_page {
            continue;
        }

        println!("Processing page {}", page_num);
        // Extract and process text from the current page
        match extract_text_from_page(&doc, *page_num) {
            Ok(text) => {
                let parts: Vec<String> = text.split('\n')
                    .map(String::from)
                    .collect();

                // Process text in chunks of 8 lines (based on PDF structure)
                for chunk in parts.chunks(8) {
                    if chunk.len() == 8 {
                        // Convert chunk to array
                        let line_array = [
                            chunk[0].clone(),
                            chunk[1].clone(),
                            chunk[2].clone(),
                            chunk[3].clone(),
                            chunk[4].clone(),
                            chunk[5].clone(),
                            chunk[6].clone(),
                            chunk[7].clone(),
                        ];
                        
                        // Check if this is a city entry and process it
                        if is_city(&line_array) {
                            let (japanese_name, english_name) = process_line(&line_array);
                            japanese_names.push(japanese_name);
                            english_names.push(english_name);
                        }
                    }
                }
            },
            Err(e) => println!("Error processing page {}: {}", page_num, e),
        }
    }

    // Save to CSV file
    save_to_csv(&japanese_names, &english_names)?;

    Ok(())
}

/// Checks if the given line represents a city entry
fn is_city(line_string: &[String; 8]) -> bool {
    let classification = &line_string[7];
    classification == "Municipality" || classification == "Populated Area"
}

/// Processes a line and extracts Japanese and English names
fn process_line(line_string: &[String; 8]) -> (String, String) {
    // Extract English name (first word only)
    let english_name = line_string[4].split(' ').next().unwrap_or("").to_string();
    
    // Get Japanese name and remove city/town/village suffixes
    let japanese_name = line_string[2].clone();
    let cleaned_japanese_name = remove_municipality_suffix(&japanese_name);
    
    (cleaned_japanese_name, english_name)
}

/// Removes municipality suffixes from Japanese place names
fn remove_municipality_suffix(name: &str) -> String {
    let suffixes = ["市", "町", "村", "区", "郡"];
    for suffix in suffixes.iter() {
        if name.ends_with(suffix) {
            return name[..name.len() - suffix.len()].to_string();
        }
    }
    name.to_string()
}

/// Saves the extracted names to a CSV file
fn save_to_csv(japanese_names: &[String], english_names: &[String]) -> Result<()> {
    let mut file = File::create("city_names.csv")?;
    
    // Write CSV header
    writeln!(file, "ja,en")?;
    
    // Write data rows
    for (ja, en) in japanese_names.iter().zip(english_names.iter()) {
        writeln!(file, "{},{}", ja, en)?;
    }
    
    println!("CSV file has been created successfully!");
    Ok(())
}

/// Extracts text content from a specific page in the PDF document
///
/// # Arguments
///
/// * `doc` - Reference to the PDF document
/// * `page_num` - Page number to extract text from
///
/// # Returns
///
/// * `Result<String>` - The extracted text or an error
fn extract_text_from_page(doc: &Document, page_num: u32) -> Result<String> {
    Ok(doc.extract_text(&[page_num])?)
}
