use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    // Open the file for reading
    let file = File::open("../output.txt")?;
    let reader = io::BufReader::new(file);

    // Create a HashSet to store unique lines
    let mut unique_lines = HashSet::new();

    // Iterate through the lines in the file
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
        // If the line is already in the HashSet, it's a duplicate, so print it
        if !unique_lines.insert(line.clone()) {
            println!("Duplicate line: {}", line);
        }
    }

    Ok(())
}
