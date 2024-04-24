use std::fmt::Debug;

use crate::jailbreak::{
    models::{JBTC, JBTR},
    traits::BaseJBTrader,
    types::JBItem,
};

const MAX_TRADE_SIZE: usize = 8;

pub struct AppState {
    pub jbtrader: Box<dyn BaseJBTrader>,
    pub your_trade: Vec<JBItem>,
    pub their_trade: Vec<JBItem>,
    pub current_trade: bool, // True = Their False = Your
    pub current_slot: u8,
}

impl AppState {
    pub fn update_this_trade(&self, items: &Vec<JBItem>) -> Vec<JBItem> {
        let mut new_items = Vec::new();
        for item in items {
            if let Some(ref_new_item) = self.jbtrader.get_item(item.name.as_str()).get(0) {
                let mut new_item = ref_new_item.clone();
                new_item.og = item.og.clone();
                new_items.push(new_item);
            }
        }
        new_items
    }

    pub fn update_trade(&mut self) {
        self.your_trade = self.update_this_trade(&self.your_trade);
        self.their_trade = self.update_this_trade(&self.their_trade);
    }

    pub fn get_trade(&self) -> &Vec<JBItem> {
        match self.current_trade {
            true => &self.their_trade,
            false => &self.your_trade,
        }
    }
    pub fn get_mut_trade(&mut self) -> &mut Vec<JBItem> {
        match self.current_trade {
            true => &mut self.their_trade,
            false => &mut self.your_trade,
        }
    }

    pub fn increment(&mut self) {
        let max = self.get_trade().len();
        if max == 0 {
            return;
        }
        self.current_slot = (self.current_slot + 1) % max as u8;
    }

    pub fn decrement(&mut self) {
        let max = self.get_trade().len();
        if max == 0 {
            return;
        }
        if self.current_slot == 0 {
            self.current_slot = (max as u8) - 1;
        } else {
            self.current_slot -= 1;
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            jbtrader: Box::new(JBTC::new()),
            your_trade: Vec::with_capacity(MAX_TRADE_SIZE),
            their_trade: Vec::with_capacity(MAX_TRADE_SIZE),
            current_trade: false,
            current_slot: 0,
        }
    }
}

impl Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState")
            .field(
                "jb_trader",
                &format!("BaseJBTrader<{}>", self.jbtrader.get_info()),
            )
            .field("your_trade", &self.your_trade)
            .field("their_trade", &self.their_trade)
            .finish()
    }
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        let jbtrader: Box<dyn BaseJBTrader> = match self.jbtrader.get_info().as_str() {
            "JBTC" => Box::new(JBTC::new()),
            _ => Box::new(JBTR::new()),
        };
        AppState {
            jbtrader,
            your_trade: self.your_trade.clone(),
            their_trade: self.their_trade.clone(),
            current_trade: self.current_trade,
            current_slot: self.current_slot,
        }
    }
}
