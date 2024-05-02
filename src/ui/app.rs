use std::cmp::Ordering;
use std::thread;
use std::thread::JoinHandle;

use crate::jailbreak::consts::MAX_SEARCH_ITEMS_COUNT;
use crate::jailbreak::models::{dupers, update, JBTC, JBTR};
use crate::jailbreak::traits::BaseJBTrader;
use crate::jailbreak::types::JBDuper;
use crate::jailbreak::util::{shorten_number, string_count};
use crate::jailbreak::{consts::MAX_ITEMS_COUNT, types::JBItem};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::text::{Line, Text};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use super::util::centered_rect;
use super::AppState;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum AppMenu {
    #[default]
    None,

    Search,
    Help,
    Edit,
    Update,
}

#[derive(Default, Debug)]
pub struct App {
    pub state: AppState,
    pub closed: bool,
    pub menu: AppMenu,

    pub search: String,
    pub search_row: usize,

    pub edit_og: String,
    pub closest_dupers: Vec<JBDuper>,
    pub help_text: Vec<String>,

    pub update_join_handle: Option<JoinHandle<()>>,
}
impl App {
    pub fn search_dupers(&mut self) {
        let mut closest = Vec::new();
        for duper in dupers::get_values().unwrap() {
            if duper.name.contains(&self.edit_og) {
                closest.push(duper);
                if closest.len() >= MAX_SEARCH_ITEMS_COUNT {
                    break;
                }
            }
        }
        closest.sort_by(|a, b| {
            let a_count = string_count(&a.name, &self.edit_og);
            let b_count = string_count(&b.name, &self.edit_og);
            if a_count > b_count {
                return Ordering::Greater;
            } else if a_count < b_count {
                return Ordering::Less;
            }
            return Ordering::Equal;
        });
        self.closest_dupers = closest;
    }

    pub fn draw(&mut self, f: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Fill(6),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Fill(6),
            ])
            .split(f.size());

        let sum1 = self.state.jbtrader.calc_value(&self.state.your_trade) as f64;
        let sum2 = self.state.jbtrader.calc_value(&self.state.their_trade) as f64;

        self.render_search(f);
        self.render_edit(f);
        self.render_help(f);
        self.render_update(f);

        if self.menu != AppMenu::None {
            return;
        }
        f.render_widget(
            Paragraph::new(format!(
                "Your trade (using {})",
                self.state.jbtrader.get_info()
            ))
            .wrap(Wrap { trim: true })
            .on_blue(),
            layout[0],
        );
        self.render_trade(
            &self.state.your_trade,
            f,
            layout[2],
            !self.state.current_trade,
        );
        f.render_widget(
            Paragraph::new(format!(
                "{} for {} profit {}",
                shorten_number(sum1),
                shorten_number(sum2),
                shorten_number(sum2 - sum1)
            )),
            layout[1],
        );

        f.render_widget(Paragraph::new("Their trade").on_blue(), layout[3]);

        self.render_trade(
            &self.state.their_trade,
            f,
            layout[5],
            self.state.current_trade,
        );
        f.render_widget(
            Paragraph::new(format!(
                "{} for {} profit {}",
                shorten_number(sum2),
                shorten_number(sum1),
                shorten_number(sum1 - sum2)
            )),
            layout[4],
        );
    }

    pub fn render_search(&mut self, f: &mut Frame) {
        if self.menu != AppMenu::Search {
            return;
        }
        let area = centered_rect(f.size(), 30, 60);
        let page = (self.search_row as u16) / (area.height - 2);
        let results = self.state.jbtrader.get_item(&self.search);
        let text = Text::from(
            results
                .iter()
                .enumerate()
                .filter(|(index, _)| {
                    let index = *index as u16;
                    index >= page * (area.height - 2) && index <= (page + 1) * (area.height - 2)
                })
                .map(|(index, item)| {
                    let mut line =
                        Line::from(format!("{}({:?})\n", item.name.to_owned(), item.category));
                    if index == self.search_row {
                        line = line.on_blue();
                    }
                    return line;
                })
                .collect::<Vec<_>>(),
        );

        f.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().title(format!("Search - {}", self.search.as_str()))),
            area,
        );
    }

    pub fn render_help(&self, f: &mut Frame) {
        if self.menu != AppMenu::Help {
            return;
        }
        // let content = fs::read_to_string("models/help.txt").unwrap();
        // let lines = content.lines().collect::<Vec<&str>>();
        f.render_widget(
            Paragraph::new(
                self.help_text
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| {
                        let index = *index as u16;
                        index >= self.search_row as u16
                            && index <= (self.search_row as u16) + f.size().height
                    })
                    .map(|(_, v)| v.to_owned())
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
            f.size(),
        );
    }

    pub fn render_update(&self, f: &mut Frame) {
        if self.menu != AppMenu::Update {
            return;
        }
        f.render_widget(
            Paragraph::new(
                "Updating this can take a while.\nMake sure you have internet connection",
            ),
            f.size(),
        );
    }

    pub fn render_edit(&mut self, f: &mut Frame) {
        if self.menu != AppMenu::Edit {
            return;
        }
        let trade = self.state.get_trade();
        if self.state.current_slot as usize >= trade.len() {
            return;
        }
        let area = centered_rect(f.size(), 30, 60);
        let page = (self.search_row as u16) / (area.height - 2);
        let text = Text::from(
            self.closest_dupers
                .iter()
                .enumerate()
                .filter(|(index, _)| {
                    let index = *index as u16;
                    index >= page * (area.height - 2) && index <= (page + 1) * (area.height - 2)
                })
                .map(|(index, duper)| {
                    let mut line = Line::from(format!(
                        "{}\n",
                        format!(
                            "{} duped: {}",
                            duper.name.to_owned(),
                            duper.item.clone().unwrap_or("Unknown item".to_owned())
                        )
                    ));
                    if index == self.search_row {
                        line = line.on_blue();
                    }
                    return line;
                })
                .collect::<Vec<_>>(),
        );
        // let item = trade.get(self.state.current_slot as usize).unwrap();

        f.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().title(format!("Edit - {}", self.edit_og.clone()))),
            area,
        )
    }

    pub fn update(&mut self) {
        self.menu = AppMenu::Update;
        self.update_join_handle = Some(thread::spawn(move || update().unwrap()));
    }

    pub fn on_key_event(&mut self, key: KeyEvent) -> std::io::Result<()> {
        if let Some(handle) = &self.update_join_handle {
            if handle.is_finished() {
                self.state.update_trade();
                self.menu = AppMenu::None;
            }
        }

        match self.menu {
            AppMenu::None => {
                if key.code == KeyCode::Char('v') && key.kind == KeyEventKind::Press {
                    if self.state.jbtrader.get_info().as_str() == "JBTC" {
                        self.state.jbtrader = Box::new(JBTR::new());
                    } else {
                        self.state.jbtrader = Box::new(JBTC::new());
                    }
                    self.state.update_trade();
                }
                if key.code == KeyCode::Esc && key.kind == KeyEventKind::Press {
                    self.closed = true;
                }
                if key.code == KeyCode::Char('l') && self.state.get_trade().len() < MAX_ITEMS_COUNT
                {
                    self.menu = AppMenu::Search;
                }
                if key.code == KeyCode::Char('h') {
                    self.menu = AppMenu::Help;
                }
                if key.code == KeyCode::Char('e') {
                    let slot = self.state.current_slot;
                    let trade = self.state.get_trade();
                    if (slot as usize) < trade.len() {
                        self.menu = AppMenu::Edit;
                    }
                }
                // Movement start
                if (key.code == KeyCode::Char('w') || key.code == KeyCode::Char('s'))
                    && key.kind == KeyEventKind::Press
                {
                    self.state.current_trade = !self.state.current_trade;
                }

                if key.code == KeyCode::Char('d') && key.kind == KeyEventKind::Press {
                    self.state.increment();
                }

                if key.code == KeyCode::Char('a') && key.kind == KeyEventKind::Press {
                    self.state.decrement();
                }
                // Movement end

                if key.code == KeyCode::Backspace && key.kind == KeyEventKind::Press {
                    let trade = match self.state.current_trade {
                        true => &mut self.state.their_trade,
                        false => &mut self.state.your_trade,
                    };
                    if !trade.is_empty() {
                        if trade.len() >= self.state.current_slot as usize {
                            trade.remove((self.state.current_slot as usize).min(trade.len() - 1));
                        }
                        if trade.len() == 0 {
                            self.state.current_slot = 0;
                        } else {
                            self.state.current_slot =
                                self.state.current_slot.min((trade.len() - 1) as u8);
                        }
                    }
                }

                if key.code == KeyCode::Char('u') && key.kind == KeyEventKind::Press {
                    self.update();
                }
            }
            AppMenu::Search => {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Backspace {
                        self.search.pop();
                    } else if key.code == KeyCode::Up {
                        let results = self.state.jbtrader.get_item(&self.search);
                        if self.search_row != 0 {
                            self.search_row -= 1;
                        } else {
                            self.search_row = results.len() - 1;
                        }
                    } else if key.code == KeyCode::Down {
                        let results = self.state.jbtrader.get_item(&self.search);
                        self.search_row = (self.search_row + 1) % results.len();
                    } else if key.code == KeyCode::Enter {
                        let results = self.state.jbtrader.get_item(&self.search);
                        if self.search_row < results.len() {
                            let item = results[self.search_row].clone();
                            self.state.get_mut_trade().push(item);
                            self.menu = AppMenu::None;
                            self.search.clear();
                        }
                    } else {
                        self.search_row = 0;
                        match key.code {
                            KeyCode::Char(v) => self.search.push(v),
                            _ => {}
                        }
                    }
                }
            }
            AppMenu::Edit => {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Backspace => {
                            self.edit_og.pop();
                            self.search_dupers();
                        }
                        KeyCode::Up => {
                            let dupers_len = self.closest_dupers.len();
                            if self.search_row != 0 {
                                self.search_row -= 1;
                            } else {
                                self.search_row = dupers_len - 1;
                            }
                        }
                        KeyCode::Down => {
                            let dupers_len = self.closest_dupers.len();
                            self.search_row = (self.search_row + 1) % dupers_len;
                        }
                        KeyCode::Char(v) => {
                            self.edit_og.push(v);
                            self.search_dupers();
                        }
                        KeyCode::Enter => {
                            self.menu = AppMenu::None;
                            let slot = self.state.current_slot as usize;
                            let trade = self.state.get_mut_trade();
                            if slot < trade.len() {
                                if let Some(duper) = self.closest_dupers.get(self.search_row) {
                                    if duper.name.is_empty() {
                                        trade[slot].og = None;
                                    } else {
                                        trade[slot].og = Some(duper.name.clone());
                                        self.edit_og = duper.name.clone();
                                        self.search_dupers();
                                        self.search_row = 0;
                                    }
                                } else {
                                    trade[slot].og = Some(self.edit_og.clone());
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            AppMenu::Help => {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Up {
                        if self.search_row != 0 {
                            self.search_row -= 1;
                        } else {
                            self.search_row = self.closest_dupers.len() - 1;
                        }
                    } else if key.code == KeyCode::Down {
                        self.search_row = (self.search_row + 1) % self.closest_dupers.len();
                    }
                }
            }
            _ => {}
        }

        if key.code == KeyCode::Esc
            && key.kind == KeyEventKind::Press
            && self.menu != AppMenu::Update
        {
            self.menu = AppMenu::None;
        }

        Ok(())
    }

    pub fn render_trade(&self, items: &Vec<JBItem>, f: &mut Frame, area: Rect, active: bool) {
        if items.len() == 0 {
            f.render_widget(
                Paragraph::new("Press `H` to open help menu")
                    .centered()
            /*.block(Block::default().bg(ratatui::style::Color::Rgb(0, 13, 52)))*/,
                area,
            );
        }
        if active {
            f.render_widget(Block::new().bg(ratatui::style::Color::Rgb(0, 13, 52)), area);
        }
        for (i, item_ref) in items.iter().enumerate() {
            // if i == items.len() - 1 {
            let mut add = area.width / (MAX_ITEMS_COUNT as u16);
            if items.len() < MAX_ITEMS_COUNT {
                // Automatic scaling that caps at `area.height + 8`
                add = area.width / items.len() as u16;
                if add > area.height + 8 {
                    add = area.height + 8;
                }
            }
            let mut block = Block::default().borders(Borders::ALL);
            if i.min(items.len() - 1) == self.state.current_slot as usize && active {
                block = block.bg(ratatui::style::Color::Rgb(0, 25, 83));
            }
            let paragraph = Paragraph::new(format!(
                "{}\nValue: {}{}\nDemand: {:?}\nType: {:?}\n{}\n{}",
                item_ref.name,
                shorten_number(item_ref.value as f64),
                match item_ref.duped_value {
                    Some(v) => format!("/{}", shorten_number(v as f64)),
                    None => "".to_owned(),
                },
                item_ref.demand,
                item_ref.category,
                item_ref.notes.clone().unwrap_or_default(),
                item_ref.og.clone().unwrap_or_default()
            ))
            .centered()
            .wrap(Wrap { trim: true })
            .block(block);
            f.render_widget(
                paragraph,
                Rect::new(area.x + (i as u16 * add), area.y, add, area.height),
            );
        }
    }
}
