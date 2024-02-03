use rand::prelude::*;
use std::collections::HashMap;
use std::ops::Range;
use time::format_description::well_known::Iso8601;
use time::{Date, Month};

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
    let date = generate_random_date(2020..2024);
    let date_string = date.format(&Iso8601::DATE).unwrap();
    row.insert("date".to_string(), date_string);

    // Word
    let length = rng.gen_range(3..20);
    row.insert("word".to_string(), generate_word(length));

    // Comma separated words
    let count = rng.gen_range(5..10);
    let word_pool = generate_word_pool(count, 3, 20);
    row.insert(
        "words".to_string(),
        generate_string_from_words(1..5, word_pool, ','),
    );

    // Hierarchical data
    let hierarchical_data_pool = generate_hierarchical_data_pool('A'..'F', 1..7);
    row.insert(
        "hierarchical".to_string(),
        generate_string_from_words(1..5, hierarchical_data_pool, ','),
    );

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

fn generate_word_pool(count: u32, length_from: u32, length_to: u32) -> Vec<String> {
    let mut rng = thread_rng();
    let mut words: Vec<String> = vec![];

    for _i in 0..count {
        words.push(generate_word(rng.gen_range(length_from..length_to)));
    }

    words
}

fn generate_hierarchical_data_pool(top_range: Range<char>, sub_range: Range<u32>) -> Vec<String> {
    let mut words: Vec<String> = vec![];

    for top_level in top_range {
        words.push(top_level.to_string());

        for sub_level in sub_range.clone() {
            let mut combined: String = top_level.to_string();
            combined.push(char::from_digit(sub_level, 10).unwrap());
            words.push(combined);
        }
    }

    words
}

fn generate_string_from_words(
    count_range: Range<u32>,
    pool: Vec<String>,
    separator: char,
) -> String {
    let mut rng = thread_rng();
    let mut result: String = "".to_string();

    for _i in 0..rng.gen_range(count_range) {
        result.push_str(pool.get(rng.gen_range(0..pool.len())).unwrap());
        result.push(separator);
    }

    result.pop();

    result
}

fn generate_random_date(year_range: Range<u32>) -> Date {
    let mut rng = thread_rng();
    let year: i32 = rng.gen_range(year_range).try_into().unwrap();
    let month: Month = Month::try_from(rng.gen_range(1..12)).unwrap();

    Date::from_calendar_date(year, month, 1).unwrap()
}
