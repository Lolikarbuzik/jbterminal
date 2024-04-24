use super::consts::MAX_SEARCH_ITEMS_COUNT;
use super::types::{JBItem, JBItemType, JBTraderInfo};
use super::util::string_count;
use std::cmp::Ordering;
use std::fmt::format;

pub trait ReplaceAll {
    fn replace_all(&self, pat: &str, to: &str) -> String;
}

impl ReplaceAll for String {
    fn replace_all(&self, pat: &str, to: &str) -> String {
        return self.replacen(pat, to, 9999);
    }
}

pub trait BaseJBTrader {
    fn new() -> Self
    where
        Self: Sized;

    // Values
    fn get_values(&self) -> Vec<JBItem>;
    fn get_dupers(&self) -> Vec<String> {
        use super::models::dupers::get_values;
        let res = get_values();
        if let Ok(dupers) = res {
            return dupers;
        } else {
            panic!("Failed to load default dupers! {}", res.err().unwrap());
        }
    }
    fn get_info(&self) -> JBTraderInfo;

    // Operations
    fn has_duped(&self, username: &String) -> bool {
        let username = username.to_lowercase();
        let dupers = self.get_dupers();
        return dupers.contains(&username);
    }

    fn get_item(&self, search: &str) -> Vec<JBItem> {
        let mut search = search
            .trim()
            .to_lowercase()
            .replace_all(" ", "")
            .replace_all("level", "l")
            .replace_all("lvl", "l")
            .replace("molten", "")
            .replace("rocket", "")
            .replace("the", "");
        if search.contains("radiant") {
            search = search.replace_all("rad", "radiant");
        }
        let mut items: Vec<JBItem> = Vec::new();
        let mut filters = Vec::new();

        for category_str in [
            "rim",
            "vehicle",
            "hyperchrome",
            "furniture",
            "color",
            "spoiler",
            "driftparticle",
            "guntexture",
            "vehiclehorn",
            "texture",
        ] {
            if search.contains(format!("@{category_str}").as_str()) {
                search = search.replace_all(format!("@{category_str}").as_str(), "");
                filters.push(JBItemType::from(category_str));
            }
        }

        // search = search.replace("hyper", "");

        let values = self.get_values();
        let available_items: Vec<(String, JBItem)> = values
            .clone()
            .iter_mut()
            .filter(|v| {
                if filters.len() == 0 {
                    return true;
                }
                return filters.contains(&v.category);
            })
            .map(|v| {
                (
                    v.name
                        .to_lowercase()
                        .replace_all(" ", "")
                        .replace("level", "l"),
                    v.clone(),
                )
            })
            .collect();

        for (k, item) in available_items {
            if k.contains(&search) {
                items.push(item);
                if items.len() >= MAX_SEARCH_ITEMS_COUNT {
                    break;
                }
            }
        }
        // println!("{search} {filters:?} {available_items:?}");

        items.sort_by(|a, b| {
            let a_count = string_count(&a.name, &search);
            let b_count = string_count(&b.name, &search);
            if a_count > b_count {
                return Ordering::Greater;
            } else if a_count < b_count {
                return Ordering::Less;
            }
            return Ordering::Equal;
        });

        return items;
    }
    fn calc_value(&self, items: &Vec<JBItem>) -> u32 {
        let mut value: u32 = 0;

        for item in items {
            if let Some(og) = &item.og {
                if self.has_duped(og) {
                    value += item.duped_value.unwrap_or(item.value);
                } else {
                    value += item.value;
                }
            } else {
                value += item.value;
            }
        }

        return value;
    }

    // Helper functions
    // TODO
}
