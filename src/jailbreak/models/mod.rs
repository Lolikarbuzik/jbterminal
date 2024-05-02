pub mod dupers;
pub mod jbtc;
pub mod jbtrading;
use std::fs;

pub use jbtc::JBTC;
pub use jbtrading::JBTR;

const DUPES_URL: &str =
    "https://raw.githubusercontent.com/Lolikarbuzik/jbtradingvalues/master/cached/dupers.json";
const JBTRADING_URL: &str =
    "https://raw.githubusercontent.com/Lolikarbuzik/jbtradingvalues/master/cached/jbtrading.json";
const JBTC_URL: &str =
    "https://raw.githubusercontent.com/Lolikarbuzik/jbtradingvalues/master/cached/jbtc.json";

fn download_from_url(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(url)?;
    fs::write(file_path, resp.text()?)?;
    Ok(())
}

pub fn update() -> Result<(), Box<dyn std::error::Error>> {
    download_from_url(DUPES_URL, "data/dupers.json")?;
    download_from_url(JBTRADING_URL, "data/jbtr.json")?;
    download_from_url(JBTC_URL, "data/jbtc.json")?;

    Ok(())
}
