use crossterm::{event::{self, KeyCode, KeyEventKind},terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen,LeaveAlternateScreen,},ExecutableCommand,};
use ratatui::{prelude::{CrosstermBackend, Stylize, Terminal}, widgets::Paragraph,};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // Main loop
    loop {
        // The draw method on terminal is the main interaction point an app has with Ratatui.
        terminal.draw(|frame| { //The draw method accepts a closure (an anonymous method) with a single Frame parameter, and renders the entire screen.
            let area = frame.size();
            frame.render_widget(Paragraph::new("Hello Ratatui! (press 'q' to quit)").white().on_blue(),area,);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') || key.kind == KeyEventKind::Press && key.code == KeyCode::Char('Q') {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

