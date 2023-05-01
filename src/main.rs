use colored::*;
use morse_rs::*;

fn main() {
    let cli = Cli::new();

    let parsed_data = match cli.mode {
        Mode::Encode => {
            let s = cli.parse_sentence();
            encode_data(s)
        }
        Mode::Decode => {
            let s = cli.parse_sentence();
            decode_data(s)
        }
    };
    println!("{}", parsed_data.green())
}
