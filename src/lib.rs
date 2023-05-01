use clap::{Parser, ValueEnum};
use colored::*;
use mapping::*;
use std::{fmt, process};

pub mod mapping;

/// A simple program to encode to /decode from morse code.
#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    /// mode
    #[arg(short, long, default_value_t=Mode::Encode)]
    pub mode: Mode,

    #[arg(value_name = "Word/Sentence")]
    pub sentence: Option<String>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    /// Encode to Dots and Dash
    Encode,

    /// Decode to String
    Decode,

    /// Print morse code mappings
    Print,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mode = format!("{:?}", self).to_lowercase();
        write!(f, "{}", mode)
    }
}

impl Cli {
    pub fn new() -> Self {
        Cli::parse()
    }

    pub fn parse_sentence(&self) -> String {
        match &self.sentence {
            Some(s) => s.clone(),
            None => {
                println!(
                    "{}",
                    "Please provide a word/sentence to encode/decode".red()
                );
                println!("Try using {} flag for usage instructions", "--help".green());
                process::exit(1);
            }
        }
    }
}

impl Default for Cli {
    fn default() -> Self {
        Cli::new()
    }
}

pub fn encode_data(sentence: String) -> String {
    let mapping = Mapping::generate_mapping(Mapping::StrToDotsAndDash);
    let mut encoded_sentence = Vec::new();
    let mut encoded_word = Vec::new();

    for word in sentence.split(' ') {
        for char in word.chars() {
            let data = char.to_uppercase().to_string();
            match mapping.get(&data) {
                Some(e) => encoded_word.push(e.clone()),
                None => {
                    eprintln!(
                        "Invalid character '{}' found in the word '{}', skipping to next character",
                        char.to_string().red(),
                        word.red()
                    );
                    encoded_word.push(char.to_string());
                }
            }
        }
        encoded_sentence.push(encoded_word.join(" "));
        encoded_word.clear();
    }
    encoded_sentence.join(" | ")
}

pub fn decode_data(sentence: String) -> String {
    let mapping = Mapping::generate_mapping(Mapping::DotsAndDashToStr);
    let mut decoded_sentence = Vec::new();

    'code_list: for code in sentence.split(' ') {
        match mapping.get(code) {
            Some(e) => decoded_sentence.push(e.clone()),
            None => {
                eprintln!("Invalid code '{}' found, skipping to next code", code.red());
                decoded_sentence.push(code.to_string());
                continue 'code_list;
            }
        }
    }
    decoded_sentence.join("")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_valid_encoding() {
        assert_eq!(
            encode_data(String::from("hello world")),
            ".... . .-.. .-.. --- | .-- --- .-. .-.. -.."
        )
    }

    #[test]
    fn test_invalid_encoding() {
        assert_eq!(
            encode_data(String::from("he!lo world")),
            ".... . ! .-.. --- | .-- --- .-. .-.. -.."
        )
    }

    #[test]
    fn test_valid_decoding() {
        assert_eq!(
            decode_data(String::from(".... . .-.. .-.. --- | .-- --- .-. .-.. -..")),
            "HELLO WORLD"
        )
    }

    #[test]
    fn test_invalid_decoding() {
        assert_eq!(
            decode_data(String::from(
                "...... . .-.. .-..-. --- | .-- --- .-. .-.. -.."
            )),
            "......EL.-..-.O WORLD"
        )
    }

    #[test]
    fn test_parsing_sentence() {
        let sentence: String = String::from("Test Sentence");
        let cli = Cli {
            mode: Mode::Encode,
            sentence: Some(sentence.clone()),
        };
        assert_eq!(&cli.parse_sentence(), &sentence)
    }
}
