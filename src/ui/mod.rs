mod app;
mod state;
mod util;
pub use app::App;
pub use state::AppState;

use crossterm::{
    event,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{stdout, Result, Stdout};

type Term = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> Result<Term> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    std::panic::set_hook(Box::new(|info| {
        stop().unwrap();
        println!("Restoring terminal\n{info}");
        stop().unwrap();
    }));

    Ok(terminal)
}

pub fn run(terminal: &mut Term, app: &mut App) -> Result<()> {
    while !app.closed {
        terminal.draw(|f| {
            app.draw(f);
        })?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                app.on_key_event(key)?;
            }
        }
    }
    stop()?;
    Ok(())
}

pub fn stop() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
