use indexmap::IndexMap;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::ops::Range;
use time::format_description::well_known::Iso8601;
use time::{Date, Month};

#[derive(Serialize, Deserialize)]
struct FakeData {
    data: Vec<IndexMap<String, String>>,
}

fn main() -> serde_json::Result<()> {
    let start_id: u32 = 0;
    let end_id: u32 = 10;
    let mut rows: Vec<IndexMap<String, String>> = vec![];
    let mut rng = thread_rng();

    let word_pool = generate_word_pool(rng.gen_range(5..10), 3, 20);
    let hierarchical_data_pool = generate_hierarchical_data_pool('A'..'F', 1..7);
    let reference_pool = generate_reference_pool(start_id, end_id);

    for id in start_id..=end_id {
        rows.push(generate_row(
            id,
            &word_pool,
            &hierarchical_data_pool,
            &reference_pool,
        ));
    }

    let fake_data = FakeData { data: rows };

    // let json = serde_json::to_string_pretty(&fake_data).unwrap();
    // println!("{}", json);

    let file = File::create("fake_data.json").unwrap();
    serde_json::to_writer_pretty(file, &fake_data)
}

fn generate_row(
    id: u32,
    word_pool: &Vec<String>,
    hierarchical_data_pool: &Vec<String>,
    reference_pool: &Vec<String>,
) -> IndexMap<String, String> {
    let mut rng = thread_rng();
    let mut row = IndexMap::new();

    // Sequencial number
    row.insert("id".to_string(), id.to_string());

    // Date
    let date = generate_random_date(2020..2024);
    let date_string = date.format(&Iso8601::DATE).unwrap();
    row.insert("date".to_string(), date_string);

    // Word
    let length = rng.gen_range(3..20);
    row.insert("word".to_string(), generate_word(length));

    // Comma separated words
    row.insert(
        "words".to_string(),
        generate_string_from_words(1..5, word_pool, ',', None),
    );

    // Hierarchical data
    row.insert(
        "hierarchical".to_string(),
        generate_string_from_words(1..5, hierarchical_data_pool, ',', None),
    );

    // References
    row.insert(
        "references".to_string(),
        generate_string_from_words(0..5, reference_pool, ',', Some(id.to_string())),
    );

    row
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

fn generate_reference_pool(start_id: u32, end_id: u32) -> Vec<String> {
    (start_id..=end_id).map(|id: u32| id.to_string()).collect()
}

fn generate_string_from_words(
    count_range: Range<u32>,
    pool: &Vec<String>,
    separator: char,
    skip_word: Option<String>,
) -> String {
    let mut rng = thread_rng();
    let mut result: String = "".to_string();

    for _i in 0..rng.gen_range(count_range) {
        let word = pool.get(rng.gen_range(0..pool.len())).unwrap();

        match &skip_word {
            Some(skip) if word.eq(skip) => continue,
            _ => {
                result.push_str(word);
                result.push(separator);
            }
        }
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
