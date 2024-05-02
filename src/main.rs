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
