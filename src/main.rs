/*
Sentence or Word Mode (default):

    Type a sentence or a word. Enter causes morris code print out.
    Prints two lines. Top line is the message, second line is morris code
    Each word is lined up with corresponding code
    End of line is CR/LF

Character mode:
--interactive
Interactive. As soon as the key is pressed the code is printed.
CR/LF exits program
*/

pub mod models;

use std::process;
use std::time::Duration;
use std::thread;



use crate::models::morris_code::morris_code_table;
use clap::Parser;

/// Get the string to convert to code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// string to convert
    code: Option<String>,
}

fn main() {
    let args = Args::parse();

    let input = match args.code {
        Some(x) => x,
        None => {
            println!("You need at least on argument. If it's a sentance use double-quotes");
            process::exit(1);
        }
    };

    let morris = build_code(&input);
    let wait_time = Duration::from_millis(500);

    for found in  morris.chars(){
        thread::sleep(wait_time);
        print_morris_char(found);

    }
}

fn print_morris_char(dd: char) {
    print!(char) ;
}

fn build_code(input_str: &str) -> String {
    let morris_table = morris_code_table();
    let mut result: String = String::new();

    for item in input_str.chars() {
        result.push_str(morris_table.get(&item.to_string().to_uppercase()).unwrap());
    }
    result
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::morris_code_table;

    #[test]
    fn test_morris_code_table_length_55() {
        let m_table = morris_code_table();
        assert_eq!(55, m_table.len());
    }

    #[test]
    fn test_morris_code_not_empty() {
        let m_table = morris_code_table();
        assert!(!m_table.is_empty())
    }

    #[test]
    fn test_letter_a() {
        let m_table = morris_code_table();
        assert_eq!(m_table.get("A").unwrap(), ".-")
    }

    #[test]
    fn test_number_42() {
        let m_table = morris_code_table();
        let forty_two = format!("{}{}", m_table.get("4").unwrap(), m_table.get("2").unwrap());
        let expected = "....-..---".to_string();
        assert_eq!(forty_two, expected);
    }

    #[test]
    fn test_build_code_letter() {
        let expected = String::from(".-.");
        assert_eq!(build_code("R"), expected);
    }
    #[test]
    fn test_build_code_word() {
        let expected = String::from("-----.-..-......");
        assert_eq!(build_code("Morris"), expected);
    }
    #[test]
    fn test_build_code_sentance() {
        let expected = String::from("-----.-..-...... -.-.---.. ..... ...-..-.-.-- -.-.------.-..");
        assert_eq!(build_code("Morris Code is very cool"), expected);
    }
}
