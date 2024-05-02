use super::super::traits::BaseJBTrader;
use crate::jailbreak::types::JBItem;
use serde_json::from_str;

const FILE_PATH: &str = "data/jbtc.json";

pub struct JBTC {}

impl BaseJBTrader for JBTC {
    fn new() -> Self {
        JBTC {}
    }

    fn get_values(&self) -> Vec<JBItem> {
        let file = std::fs::read_to_string(FILE_PATH).unwrap();
        let values = from_str(&file).unwrap();
        return values;
    }

    fn get_info(&self) -> crate::jailbreak::types::JBTraderInfo {
        return "JBTC".to_string();
    }
}
