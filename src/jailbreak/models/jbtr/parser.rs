use crate::jailbreak::types::{JBItemDemand, JBItemType};

use super::JBItem;
use reqwest::blocking as breqwest;
use scraper::{Html, Selector};

const SELECTOR: &str = "div.tyJCtd.mGzaTb.Depvyb.baZpAe";
const MIN_VALUE: u32 = 10_000;

fn parse_url(
    section: &str,
    data: &mut Vec<JBItem>,
    category: JBItemType,
) -> Result<(), reqwest::Error> {
    let url = "https://www.jailbreaktrading.net/".to_owned() + section;
    let req = breqwest::get(url)?;
    let text = req.text()?;

    let root = Html::parse_document(&text);
    let selector = Selector::parse(SELECTOR).unwrap();

    for element in root.select(&selector) {
        if element.inner_html().contains("Copyright") {
            continue;
        }
        let mut texts = Vec::new();
        for child in element.child_elements() {
            let mut text = String::new();
            for subchild in child.child_elements() {
                // println!("{}", subchild.inner_html());
                text.push_str(&subchild.text().collect::<Vec<_>>().join(""));
            }
            texts.push(text);
            // println!("{text} {}", child.value().name());
        }
        if texts.len() < 8 {
            continue;
        }
        let value_text = texts[7]
            .replace("JailbreakTrading.net: 💸", "")
            .replace("$", "")
            .replacen(",", "", 10)
            .trim()
            .to_owned();
        let duped_value_text = texts[6]
            .replace("Duped Item Value: ⚠\u{fe0f}", "")
            .replace("$", "")
            .replacen(",", "", 10)
            .trim()
            .to_owned();
        if value_text.parse::<u32>().unwrap_or(0) <= MIN_VALUE {
            continue;
        }
        data.push(JBItem {
            name: texts[0].clone(),
            value: value_text.parse::<u32>().unwrap_or(0),
            demand: JBItemDemand::from(texts[2].replace("Demand: ⭐ ", "")),
            duped_value: duped_value_text.parse::<u32>().ok(),
            notes: None,
            og: None,
            category,
        });
    }

    Ok(())
}

pub fn parse() -> Result<(), reqwest::Error> {
    let mut items = Vec::new();

    parse_url("vehicles", &mut items, JBItemType::Vehicle)?;
    parse_url("textures", &mut items, JBItemType::Texture)?;
    parse_url("spoilers", &mut items, JBItemType::Spoiler)?;
    parse_url("rims", &mut items, JBItemType::Rim)?;
    parse_url("tires", &mut items, JBItemType::Tires)?;
    parse_url("colors", &mut items, JBItemType::Color)?;
    parse_url("other-values/furniture", &mut items, JBItemType::Furniture)?;
    parse_url(
        "other-values/gun-textures",
        &mut items,
        JBItemType::GunTexture,
    )?;
    parse_url(
        "other-values/vehicle-horns",
        &mut items,
        JBItemType::VehicleHorn,
    )?;
    parse_url(
        "other-values/drift-particles",
        &mut items,
        JBItemType::DriftParticle,
    )?;
    std::fs::write("data/jbtr.json", serde_json::to_string(&items).unwrap()).unwrap();

    Ok(())
}
