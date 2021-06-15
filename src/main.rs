use std::{error::Error, io};

use termion::raw::IntoRawMode;
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout()
        .into_raw_mode()
        .expect("Failed to switch to raw mode");
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Failed to initialize terminal backend");

    terminal
        .draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(60),
                        Constraint::Percentage(20),
                    ]
                    .as_ref(),
                )
                .split(frame.size());

            let input = "This is a test message";

            let input_panel = Paragraph::new(input).block(
                Block::default().borders(Borders::ALL).title(Span::styled(
                    "Your message",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
            );

            frame.render_widget(input_panel, chunks[2]);
        })
        .expect("Failed to draw to terminal");

    Ok(())
}
