use std::fs;

use reqwest::blocking as breqwest;
use serde_json::Value;

use crate::jailbreak::traits::{self, ReplaceAll};
use crate::jailbreak::types::{JBItem, JBItemDemand, JBItemType, JBTerminalConfig};
// TODO

const SPREADSHEET_ID: &str = "12aPBmrHP5MLwoht9QcBiERPYUmXTJiTbBF43630togE";

fn get_spreadsheet_data(range: &str) -> Result<Vec<Value>, reqwest::Error> {
    let api_key = JBTerminalConfig::new().api_key;
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?key={}",
        SPREADSHEET_ID, range, api_key
    );

    let response = breqwest::get(&url)?;
    let body = response.text()?;
    let data: serde_json::Value = serde_json::from_str(&body).unwrap();
    let cells = data.get("values").unwrap().as_array().unwrap();
    Ok(cells.clone())
}

pub fn parse_values(
    range: &str,
    values: &mut Vec<JBItem>,
    mut category: JBItemType,
    str_add: &str,
) {
    let cells = get_spreadsheet_data(range).unwrap();
    for cell in cells {
        let cell = cell.as_array().unwrap();
        let mut name = cell.get(0).unwrap().as_str().clone().unwrap().to_string();
        let value = cell
            .get(1)
            .unwrap()
            .as_str()
            .clone()
            .unwrap()
            .to_string()
            .replace_all(",", "")
            .parse::<u32>()
            .unwrap();
        let mut duped_value = None;
        let mut demand = JBItemDemand::None;
        let mut notes = None;
        if category != JBItemType::Hyperchrome {
            duped_value = cell
                .get(2)
                .unwrap()
                .as_str()
                .clone()
                .unwrap()
                .to_string()
                .replace_all(",", "")
                .parse::<u32>()
                .ok();
            demand = JBItemDemand::from(cell.get(3).unwrap().as_str().unwrap().to_string());
            if let Some(value) = cell.get(4) {
                notes = Some(value.as_str().unwrap().to_string());
            }
        } else {
            name = "Hyper ".to_owned() + &name;
            demand = JBItemDemand::from(cell.get(2).unwrap().as_str().unwrap().to_string());
            if let Some(value) = cell.get(3) {
                notes = Some(value.as_str().unwrap().to_string());
            }
        }
        // TODO notes be optional not ""

        if name.to_lowercase().contains("horn") {
            category = JBItemType::VehicleHorn;
        } else if name.to_lowercase().contains("drift") {
            category = JBItemType::DriftParticle;
        } else if name.to_lowercase().contains("radiant")
            || name.as_str() == "Gold"
            || name.as_str() == "Fall chrome"
        {
            category = JBItemType::Color;
        }

        // println!("{name} {value} {duped_value:?} {notes}");
        name.push_str(str_add);
        values.push(JBItem {
            name,
            value,
            demand,
            duped_value,
            notes,
            og: None,
            category,
        })
    }
}

pub fn parse() -> Result<(), reqwest::Error> {
    let api_key = JBTerminalConfig::new().api_key;
    if api_key.is_empty() {
        return Ok(()); // Not ok but idc
    }
    let mut values: Vec<JBItem> = Vec::new();
    parse_values("Value List!C20:G67", &mut values, JBItemType::Vehicle, "");
    parse_values("Value List!C71:G120", &mut values, JBItemType::Texture, "");
    parse_values("Value List!C124:G183", &mut values, JBItemType::Rim, "");
    parse_values("Value List!C187:G245", &mut values, JBItemType::Spoiler, "");
    parse_values("Value List!C249:G267", &mut values, JBItemType::Tires, "");
    parse_values(
        "Value List!C271:G308",
        &mut values,
        JBItemType::Furniture,
        "",
    );
    parse_values(
        "Value List!C312:G321",
        &mut values,
        JBItemType::GunTexture,
        "",
    );

    // Hyperchromes
    parse_values(
        "Hyperchromes!C22:F30",
        &mut values,
        JBItemType::Hyperchrome,
        " level V",
    );
    parse_values(
        "Hyperchromes!C34:F41",
        &mut values,
        JBItemType::Hyperchrome,
        " level IV",
    );
    parse_values(
        "Hyperchromes!C45:F52",
        &mut values,
        JBItemType::Hyperchrome,
        " level III",
    );
    parse_values(
        "Hyperchromes!C56:F63",
        &mut values,
        JBItemType::Hyperchrome,
        " level II",
    );
    // No hyperchromes for lvl 1

    fs::write("data/jbtc.json", serde_json::to_string(&values).unwrap()).unwrap();

    Ok(())
}
