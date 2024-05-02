use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct JBItem {
    pub name: String,
    pub value: u32,
    pub demand: JBItemDemand,
    pub duped_value: Option<u32>,
    pub notes: Option<String>,
    pub og: Option<String>,
    pub category: JBItemType,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct JBDuper {
    pub name: String,
    pub item: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum JBItemType {
    Vehicle,
    Color,
    Tires,
    Hyperchrome,
    Texture,
    GunTexture,
    VehicleHorn,
    Spoiler,
    Rim,
    Furniture,
    DriftParticle,
    Unknown,
}

impl From<String> for JBItemType {
    fn from(value: String) -> Self {
        let value = value.to_lowercase();
        match value.as_str() {
            "vehicle" => Self::Vehicle,
            "color" => Self::Color,
            "texture" => Self::Texture,
            "guntexture" => Self::GunTexture,
            "vehiclehorn" => Self::VehicleHorn,
            "spoiler" => Self::Spoiler,
            "rim" => Self::Rim,
            "furniture" => Self::Furniture,
            "driftparticle" => Self::DriftParticle,
            "hyperchrome" => Self::Hyperchrome,
            "tires" => Self::Tires,
            _ => Self::Unknown,
        }
    }
}

impl From<&str> for JBItemType {
    fn from(value: &str) -> Self {
        JBItemType::from(value.to_owned())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum JBItemDemand {
    None,
    VeryLow,
    Low,
    Mid,
    Decent,
    High,
    VeryHigh,
    Decreasing,
}

impl From<String> for JBItemDemand {
    fn from(value: String) -> Self {
        let value = value.replace(".", "");
        match value.as_str() {
            "Very Low" => JBItemDemand::VeryLow,
            "Very low" => JBItemDemand::VeryLow,
            "Low" => JBItemDemand::Low,
            "Mid" => JBItemDemand::Mid,
            "Decent" => JBItemDemand::Decent,
            "High" => JBItemDemand::High,
            "Extremely Low" => JBItemDemand::VeryLow,
            "Below Average" => JBItemDemand::Mid,
            "Mainly Average" => JBItemDemand::Mid,
            "Above Average" => JBItemDemand::Decent,
            "Slowly Decreasing" => JBItemDemand::Decreasing,
            "Close to none" => JBItemDemand::VeryLow, // or None but wont
            _ => {
                // eprintln!("Demand parsing error got {}", value);
                return JBItemDemand::None;
            }
        }
    }
}

impl From<JBItemDemand> for String {
    fn from(value: JBItemDemand) -> Self {
        use super::consts::DEMAND_NAMES;
        unsafe {
            let idx: u8 = std::mem::transmute(value);

            return DEMAND_NAMES[idx as usize].to_string();
        }
    }
}

// struct JBItemDemandInfo(&'static str, f32);

pub type JBTraderInfo = String;
