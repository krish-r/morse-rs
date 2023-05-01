use colored::*;
use morse_rs::*;

fn main() {
    let cli = Cli::new();

    match cli.mode {
        Mode::Encode => {
            let s = cli.parse_sentence();
            println!("{}", encode_data(s).green())
        }
        Mode::Decode => {
            let s = cli.parse_sentence();
            println!("{}", decode_data(s).green())
        }
        Mode::Print => {
            mapping::Mapping::print_mappings();
        }
    };
}
