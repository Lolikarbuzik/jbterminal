use serde_json::from_str;

use crate::jailbreak::types::JBDuper;

const FILE_PATH: &str = "data/dupers.json";

pub fn get_values() -> std::io::Result<Vec<JBDuper>> {
    let file = std::fs::read_to_string(FILE_PATH)?;
    let mut values: Vec<JBDuper> = from_str(&file)?;

    for val in &mut values {
        val.name = val.name.to_lowercase();
    }

    Ok(values)
}
