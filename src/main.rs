use colored::*;
use morse_rs::*;
use std::process;

fn main() {
    let cli = Cli::new();

    let parsed_data = match cli.sentence {
        Some(s) => match cli.mode {
            Mode::Encode => encode_data(s),
            Mode::Decode => decode_data(s),
        },
        None => {
            println!(
                "{}",
                "Please provide a word/sentence to encode/decode".red()
            );
            println!("Try using {} flag for usage instructions", "--help".green());
            process::exit(1);
        }
    };
    println!("{}", parsed_data.green())
}
