use crate::config::{ColumnConfig, Config, PoolConfig};
use indexmap::IndexMap;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Range;
use time::format_description::well_known::Iso8601;
use time::{Date, Month, OffsetDateTime, Time};
use toml::value::Datetime;

#[derive(Serialize, Deserialize)]
pub struct FakeData {
    pub data: Vec<IndexMap<String, String>>,
}

pub struct Pool<'a> {
    pub contents: Vec<String>,
    pub config: &'a PoolConfig,
}

pub fn generate_rows(config: Config) -> FakeData {
    let mut rows: Vec<IndexMap<String, String>> = vec![];
    let pools = generate_pools(&config);

    for id in config.start_id..=config.end_id {
        rows.push(generate_row(id, &config.columns, &pools));
    }

    FakeData { data: rows }
}

fn generate_pools(config: &Config) -> HashMap<String, Pool> {
    let mut pools: HashMap<String, Pool> = HashMap::new();
    let mut rng = thread_rng();

    for pool in &config.pools {
        match pool {
            PoolConfig::Words {
                ref name,
                count_from,
                count_to,
                word_length_from,
                word_length_to,
            } => {
                pools.insert(
                    name.to_string(),
                    Pool {
                        contents: generate_word_pool(
                            rng.gen_range(*count_from..*count_to),
                            *word_length_from,
                            *word_length_to,
                        ),
                        config: pool,
                    },
                );
            }
            PoolConfig::Hierarchical {
                ref name,
                top_level_from,
                top_level_to,
                sub_level_from,
                sub_level_to,
            } => {
                pools.insert(
                    name.to_string(),
                    Pool {
                        contents: generate_hierarchical_data_pool(
                            *top_level_from..*top_level_to,
                            *sub_level_from..*sub_level_to,
                        ),
                        config: pool,
                    },
                );
            }
            PoolConfig::References { ref name } => {
                pools.insert(
                    name.to_string(),
                    Pool {
                        contents: generate_reference_pool(config.start_id, config.end_id),
                        config: pool,
                    },
                );
            }
        }
    }

    pools
}

pub fn generate_row(
    id: u32,
    column_configs: &Vec<ColumnConfig>,
    pools: &HashMap<String, Pool>,
) -> IndexMap<String, String> {
    let mut rng = thread_rng();
    let mut row = IndexMap::new();

    for column in column_configs {
        match column {
            ColumnConfig::Sequence { name } => {
                row.insert(name.to_string(), id.to_string());
            }
            ColumnConfig::Date {
                name,
                date_from,
                date_to,
            } => {
                let date = generate_random_date(*date_from, *date_to);
                let date_string = date.format(&Iso8601::DATE_TIME_OFFSET).unwrap();
                row.insert(name.to_string(), date_string);
            }
            ColumnConfig::Word {
                name,
                length_from,
                length_to,
            } => {
                let length = rng.gen_range(*length_from..*length_to);
                row.insert(name.to_string(), generate_word(length));
            }
            ColumnConfig::CommaSeparated {
                name,
                pool,
                count_from,
                count_to,
            } => {
                let pool: &Pool = pools.get(pool).unwrap();
                let skip_word: Option<String> = match pool.config {
                    PoolConfig::References { .. } => Some(id.to_string()),
                    _ => None,
                };

                row.insert(
                    name.to_string(),
                    generate_string_from_words(
                        *count_from..*count_to,
                        &pool.contents,
                        ',',
                        skip_word,
                    ),
                );
            }
        }
    }

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

pub fn generate_word_pool(count: u32, length_from: u32, length_to: u32) -> Vec<String> {
    let mut rng = thread_rng();
    let mut words: Vec<String> = vec![];

    for _i in 0..count {
        words.push(generate_word(rng.gen_range(length_from..length_to)));
    }

    words
}

pub fn generate_hierarchical_data_pool(
    top_range: Range<char>,
    sub_range: Range<u32>,
) -> Vec<String> {
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

pub fn generate_reference_pool(start_id: u32, end_id: u32) -> Vec<String> {
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

fn generate_random_date(date_from: Datetime, date_to: Datetime) -> OffsetDateTime {
    let mut rng = thread_rng();
    let from_year = date_from.date.unwrap().year;
    let to_year = date_to.date.unwrap().year;
    let year: u16 = rng.gen_range(from_year..to_year).into();
    let month_range = match year {
        year if year == from_year => {
            let from_month = date_from.date.unwrap().month;
            from_month..12
        }
        year if year == to_year => {
            let to_month = date_to.date.unwrap().month;
            1..to_month
        }
        _ => 1..12,
    };
    let month: Month = Month::try_from(rng.gen_range(month_range)).unwrap();
    let date = Date::from_calendar_date(year.into(), month, 1).unwrap();

    OffsetDateTime::new_utc(date, Time::MIDNIGHT)
}
