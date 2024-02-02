use rand::prelude::*;
use std::collections::HashMap;
use time::format_description::well_known::Iso8601;
use time::Date;

fn main() {
    for i in 0..10 {
        generate_row(i);
    }
}

fn generate_row(index: u32) {
    let mut rng = thread_rng();
    let mut row = HashMap::new();

    // Sequencial number
    row.insert("id".to_string(), index.to_string());

    // Date
    let date = Date::from_ordinal_date(2024, 33);
    let date_string = date.unwrap().format(&Iso8601::DATE).unwrap();
    row.insert("date".to_string(), date_string);

    // Word
    let length = rng.gen_range(3..20);
    row.insert("word".to_string(), generate_word(length));

    println!("{:?}", row);
}

fn generate_word(length: u32) -> String {
    let mut rng = thread_rng();
    let mut word: String = "".to_string();

    for i in 0..length {
        let letter: char = match i {
            0 => rng.gen_range(b'A'..b'Z') as char,
            _ => rng.gen_range(b'a'..b'z') as char,
        };
        word.push(letter);
    }

    word
}
