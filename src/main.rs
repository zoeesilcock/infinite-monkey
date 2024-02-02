use std::collections::HashMap;
use time::format_description::well_known::Iso8601;
use time::Date;

fn main() {
    let mut row = HashMap::new();
    let date = Date::from_ordinal_date(2024, 33);
    let date_string = date.unwrap().format(&Iso8601::DATE).unwrap();

    row.insert("id".to_string(), "1".to_string());
    row.insert("date".to_string(), date_string);

    for (key, value) in &row {
        println!("{}: {}", key, value);
    }
}
