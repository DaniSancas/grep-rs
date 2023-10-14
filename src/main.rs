use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Struct to parse command line arguments.
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
    let lines = get_lines_from_input(&matches);
    match_over_line_iterator(lines, &regex);
}

/// Function to get an iterator over the lines of the input.
/// The input can be a string, a file or stdin.
fn get_lines_from_input(matches: &Args) -> Box<dyn Iterator<Item = String> + '_> {
    match (matches.input.as_deref(), matches.file.as_deref()) {
        (Some(input_text), None) => {
            Box::new(input_text.lines().map(std::string::ToString::to_string))
        }
        (None, Some(file_path)) => Box::new(read_lines(file_path).flatten()),
        (None, None) => Box::new(io::stdin().lock().lines().flatten()),
        _ => panic!("Cannot use both input and file"),
    }
}

/// Function which takes an iterator of String or &str and prints each line
/// that matches the given regex.
fn match_over_line_iterator<T, S>(lines: T, regex: &Regex)
where
    T: Iterator<Item = S>,
    S: AsRef<str>,
{
    for line in lines {
        if let Some(coincidence) = match_pattern(regex, line.as_ref()) {
            println!("{coincidence}");
        }
    }
}

/// Function to match a pattern in a line.
///
/// Returns the line with the pattern highlighted.
/// If the pattern is not found, returns None.
#[must_use]
pub fn match_pattern(regex: &Regex, line: &str) -> Option<String> {
    if regex.is_match(line) {
        let highlighted = regex.replace_all(line, "\x1b[31m$0\x1b[0m");
        Some(highlighted.to_string())
    } else {
        None
    }
}

/// Function to read lines from a file.
///
/// Returns an iterator over the lines of the file.
/// If the file cannot be opened, returns an error.
fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Failed to read input file");
    io::BufReader::new(file).lines()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_pattern() {
        let regex = Regex::new("pattern").unwrap();
        let line = "This is a pattern";
        let expected = Some("This is a \x1b[31mpattern\x1b[0m".to_string());
        let result = match_pattern(&regex, &line);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_match_pattern_no_match() {
        let regex = Regex::new("pattern").unwrap();
        let line = "This is not a match";
        let expected = None;
        let result = match_pattern(&regex, &line);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_match_pattern_multiple_matches() {
        let regex = Regex::new("pattern").unwrap();
        let line = "This is a pattern and another pattern";
        let expected =
            Some("This is a \x1b[31mpattern\x1b[0m and another \x1b[31mpattern\x1b[0m".to_string());
        let result = match_pattern(&regex, &line);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_match_pattern_multiple_matches_same_word() {
        let regex = Regex::new("pattern").unwrap();
        let line = "This is a patternpattern";
        let expected = Some("This is a \x1b[31mpattern\x1b[0m\x1b[31mpattern\x1b[0m".to_string());
        let result = match_pattern(&regex, &line);
        assert_eq!(result, expected);
    }
}
