use std::io;
use termion::clear;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, Row, Table};
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    println!("{}", clear::All);

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    loop {
        let algorithms = vec![vec!["Bubble Sort"], vec!["Merge Sort"], vec!["Quick Sort"]];

        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(4)
                .constraints([Constraint::Percentage(80)].as_ref())
                .split(f.size());
            let selected_style = Style::default()
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD);
            let default_style = Style::default().fg(Color::White);
            let header = ["Select a sorting algorithm"];
            let rows = algorithms.iter().map(|item| {
                let style = if item[0] == "Bubble Sort" {
                    selected_style
                } else {
                    default_style
                };
                Row::StyledData(item.into_iter(), style)
            });
            let table = Table::new(header.iter(), rows)
                .block(Block::default().title("Sorting Hat").borders(Borders::ALL))
                .header_style(Style::default().fg(Color::Green))
                .widths(&[Constraint::Length(60)]);
            f.render_widget(table, chunks[0])
        });
    }
}
