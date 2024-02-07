use serde::Deserialize;
use toml::value::Datetime;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum PoolConfig {
    #[serde(rename = "words")]
    Words {
        name: String,
        count_from: u32,
        count_to: u32,
        word_length_from: u32,
        word_length_to: u32,
    },
    #[serde(rename = "hierarchical")]
    Hierarchical {
        name: String,
        top_level_from: char,
        top_level_to: char,
        sub_level_from: u32,
        sub_level_to: u32,
    },
    #[serde(rename = "references")]
    References { name: String },
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ColumnConfig {
    #[serde(rename = "sequence")]
    Sequence { name: String },
    #[serde(rename = "date")]
    Date {
        name: String,
        date_from: Datetime,
        date_to: Datetime,
    },
    #[serde(rename = "word")]
    Word {
        name: String,
        length_from: u32,
        length_to: u32,
    },
    #[serde(rename = "comma-separated")]
    CommaSeparated {
        name: String,
        pool: String,
        count_from: u32,
        count_to: u32,
    },
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub start_id: u32,
    pub end_id: u32,
    pub pools: Vec<PoolConfig>,
    pub columns: Vec<ColumnConfig>,
}
