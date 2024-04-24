mod parsers;
pub use parsers::parse;
use serde_json::from_str;

const FILE_PATH: &str = "data/dupers.json";

pub fn get_values() -> std::io::Result<Vec<String>> {
    let file = std::fs::read_to_string(FILE_PATH)?;
    let mut values: Vec<String> = from_str(&file)?;

    for val in &mut values {
        *val = val.to_lowercase();
    }

    Ok(values)
}
