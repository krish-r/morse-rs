use colored::*;
use std::collections::HashMap;

pub enum Mapping {
    StrToDotsAndDash,
    DotsAndDashToStr,
}

impl Mapping {
    pub fn print_mappings() {
        let mapping = Mapping::generate_mapping(Mapping::StrToDotsAndDash);

        let alphabets: Vec<String> = ('A'..='Z').map(|c| c.to_string()).collect();
        for alphabet in alphabets {
            println!(
                "{}: {}",
                alphabet.green(),
                mapping.get(&alphabet).unwrap().green()
            );
        }
    }

    pub fn generate_mapping(mapping: Mapping) -> HashMap<String, String> {
        match mapping {
            Mapping::StrToDotsAndDash => HashMap::from([
                (String::from("A"), String::from(".-")),
                (String::from("B"), String::from("-...")),
                (String::from("C"), String::from("-.-.")),
                (String::from("D"), String::from("-..")),
                (String::from("E"), String::from(".")),
                (String::from("F"), String::from("..-.")),
                (String::from("G"), String::from("--.")),
                (String::from("H"), String::from("....")),
                (String::from("I"), String::from("..")),
                (String::from("J"), String::from(".---")),
                (String::from("K"), String::from("-.-")),
                (String::from("L"), String::from(".-..")),
                (String::from("M"), String::from("--")),
                (String::from("N"), String::from("-.")),
                (String::from("O"), String::from("---")),
                (String::from("P"), String::from(".--.")),
                (String::from("Q"), String::from("--.-")),
                (String::from("R"), String::from(".-.")),
                (String::from("S"), String::from("...")),
                (String::from("T"), String::from("-")),
                (String::from("U"), String::from("..-")),
                (String::from("V"), String::from("...-")),
                (String::from("W"), String::from(".--")),
                (String::from("X"), String::from("-..-")),
                (String::from("Y"), String::from("-.--")),
                (String::from("Z"), String::from("--..")),
                (String::from("1"), String::from(".----")),
                (String::from("2"), String::from("..---")),
                (String::from("3"), String::from("...--")),
                (String::from("4"), String::from(".....-")),
                (String::from("5"), String::from(".....")),
                (String::from("6"), String::from("-....")),
                (String::from("7"), String::from("--...")),
                (String::from("8"), String::from("---..")),
                (String::from("9"), String::from("----.")),
                (String::from("0"), String::from("-----")),
            ]),
            Mapping::DotsAndDashToStr => HashMap::from([
                (String::from(".-"), String::from("A")),
                (String::from("-..."), String::from("B")),
                (String::from("-.-."), String::from("C")),
                (String::from("-.."), String::from("D")),
                (String::from("."), String::from("E")),
                (String::from("..-."), String::from("F")),
                (String::from("--."), String::from("G")),
                (String::from("...."), String::from("H")),
                (String::from(".."), String::from("I")),
                (String::from(".---"), String::from("J")),
                (String::from("-.-"), String::from("K")),
                (String::from(".-.."), String::from("L")),
                (String::from("--"), String::from("M")),
                (String::from("-."), String::from("N")),
                (String::from("---"), String::from("O")),
                (String::from(".--."), String::from("P")),
                (String::from("--.-"), String::from("Q")),
                (String::from(".-."), String::from("R")),
                (String::from("..."), String::from("S")),
                (String::from("-"), String::from("T")),
                (String::from("..-"), String::from("U")),
                (String::from("...-"), String::from("V")),
                (String::from(".--"), String::from("W")),
                (String::from("-..-"), String::from("X")),
                (String::from("-.--"), String::from("Y")),
                (String::from("--.."), String::from("Z")),
                (String::from(".----"), String::from("1")),
                (String::from("..---"), String::from("2")),
                (String::from("...--"), String::from("3")),
                (String::from(".....-"), String::from("4")),
                (String::from("....."), String::from("5")),
                (String::from("-...."), String::from("6")),
                (String::from("--..."), String::from("7")),
                (String::from("---.."), String::from("8")),
                (String::from("----."), String::from("9")),
                (String::from("-----"), String::from("0")),
                (String::from("|"), String::from(" ")),
            ]),
        }
    }
}
