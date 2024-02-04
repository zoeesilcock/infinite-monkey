use crate::generate::{
    generate_hierarchical_data_pool, generate_reference_pool, generate_row, generate_word_pool,
    FakeData,
};
use indexmap::IndexMap;
use rand::prelude::*;
use std::fs::File;

mod generate;

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
