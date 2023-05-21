/*
Sentence or Word Mode (default):

    - Type a sentence or a word. Enter causes morris code print out.
    - Prints two lines. Top line is the message, second line is morris code
    - Each word is lined up with corresponding code
    - End of line is CR/LF

Character mode:
- -c
- Interactive. As soon as the key is pressed the code is printed.
- CR/LF exits program
*/

pub mod models;

use crate::models::morris_code::morris_code_table;

fn main() {
    let morris_table = morris_code_table();

    println!("Quote is {}", morris_table.get("\"").unwrap());
    println!("Capitable A is {}", morris_table.get("A").unwrap());
}

#[cfg(test)]
mod tests {

    use crate::morris_code_table;

    #[test]
    fn test_morris_code_table_length_53() {
        let m_table = morris_code_table();
        assert_eq!(53, m_table.len());
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
}
