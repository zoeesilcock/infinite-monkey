use crate::generate::generate_rows;
use std::fs::File;

mod generate;

fn main() -> serde_json::Result<()> {
    let fake_data = generate_rows();

    // let json = serde_json::to_string_pretty(&fake_data).unwrap();
    // println!("{}", json);

    let file = File::create("fake_data.json").unwrap();
    serde_json::to_writer_pretty(file, &fake_data)
}
