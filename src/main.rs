use ui::AppState;

use crate::jailbreak::traits::BaseJBTrader;

mod jailbreak;
mod ui;

fn main() -> std::io::Result<()> {
    // Wow thats a lot of code!
    let mut terminal = ui::init()?;
    let mut app = ui::App::default();
    let content = std::fs::read_to_string("data/help.txt").unwrap();
    app.help_text = content
        .lines()
        .collect::<Vec<&str>>()
        .iter_mut()
        .map(|v| v.to_string())
        .collect();
    app.search_dupers();
    ui::run(&mut terminal, &mut app)?;
    Ok(())
}

#[test]
fn thrade_update() {
    let mut state = AppState::default();
    state
        .your_trade
        .push(state.jbtrader.get_item("Blue level V")[0].clone());

    println!("{:?}", state.your_trade);
    state.update_trade();
    println!("{:?}", state.your_trade);
}
