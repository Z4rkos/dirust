use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::error::Error;


pub struct Wordlist;
impl Wordlist {
    pub fn from_path(path: String) -> Result<Vec<String>, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut output: Vec<String> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            output.push(line);
        }
        Ok(output)
    }
    // This is mostly for testing
    pub fn from_range(range: std::ops::Range<i32>) -> Result<Vec<String>, Box<dyn Error>> {
        let output: Vec<String> = range.map(|i| i.to_string()).collect();
        Ok(output)
    }
}
