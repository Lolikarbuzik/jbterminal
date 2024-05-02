use crate::jailbreak::types::{JBItem, JBTraderInfo};

use super::super::traits::BaseJBTrader;
use serde_json::from_str;

const FILE_PATH: &str = "data/jbtr.json";

pub struct JBTR {}

impl BaseJBTrader for JBTR {
    fn new() -> Self {
        JBTR {}
    }

    fn get_values(&self) -> Vec<JBItem> {
        let file = std::fs::read_to_string(FILE_PATH).unwrap();
        let values = from_str(&file).unwrap();
        return values;
    }

    fn get_info(&self) -> JBTraderInfo {
        return "JBTR".to_string();
    }
}
