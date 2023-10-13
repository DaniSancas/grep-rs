use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Pattern to search for
    pattern: String,

    /// Sets the input text to use
    #[arg(short, long, conflicts_with("file"))]
    input: Option<String>,

    /// Sets the input file to use
    #[arg(short, long, conflicts_with("input"))]
    file: Option<String>,
}

fn main() {
    let matches = Args::parse();
    let regex = Regex::new(&matches.pattern).expect("Invalid pattern or regular expression");

    if let Some(input_text) = matches.input.as_deref() {
        let lines = input_text.lines();
        for line in lines {
            if let Some(coincidence) = match_pattern(&regex, &line) {
                println!("{}", coincidence);
            }
        }
    } else if let Some(file_path) = matches.file.as_deref() {
        let lines = read_lines(file_path);
        for line in lines {
            if let Ok(line_content) = line {
                if let Some(coincidence) = match_pattern(&regex, &line_content) {
                    println!("{}", coincidence);
                }
            }
        }
    } else {
        eprintln!("You must provide an input text or a file. Type --help for more info.");
    }
}

pub fn match_pattern(regex: &Regex, line: &str) -> Option<String> {
    if regex.is_match(line) {
        let highlighted = regex.replace_all(line, "\x1b[31m$0\x1b[0m");
        Some(highlighted.to_string())
    } else {
        None
    }
}

/// Function to read lines from a file
fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Failed to read input file");
    io::BufReader::new(file).lines()
}
