use std::io::{Result, stdout};
use std::time::Duration;

use crossterm::{event, ExecutableCommand};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::prelude::CrosstermBackend;
use ratatui::style::Stylize;
use ratatui::Terminal;
use ratatui::widgets::Paragraph;

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            let mut area2 = frame.size();
            let reduce_size = area.height / 2;
            area2.y = reduce_size;
            area2.height = reduce_size;
            frame.render_widget(
                Paragraph::new("Hello Ratatui! (press 'q' to quit)").on_black(),
                area,
            );
            frame.render_widget(
                Paragraph::new("Hello Ratatui! (press 'q' to quit)").black().on_white(),
                area2,
            );
        })?;
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    println!("quitting!");
                    break;
                };
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
