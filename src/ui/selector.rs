use crate::persistence::CharacterSummary;
use crossterm::event::{self, Event, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Alignment;
use std::io::Stdout;
use std::time::Duration;

use super::terminal::{prepare_terminal, restore_terminal};

pub enum MenuSelection {
    CreateNew,
    LoadExisting(i64),
    DeleteProfile(i64),
}

pub fn character_selector(rows: &[CharacterSummary]) -> std::io::Result<MenuSelection> {
    if rows.is_empty() {
        return Ok(MenuSelection::CreateNew);
    }

    let mut terminal = prepare_terminal()?;
    let result = selector_loop(&mut terminal, rows);
    restore_terminal(terminal)?;
    result
}

fn selector_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    rows: &[CharacterSummary],
) -> std::io::Result<MenuSelection> {
    let mut index: usize = 0;
    let options_len = rows.len() + 1;
    let last_index = options_len.saturating_sub(1);
    loop {
        terminal.draw(|f| {
            let size = f.area();
            let block = Block::default()
                .title("Choose a Character")
                .borders(Borders::ALL);
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
                .split(size);

            let mut items: Vec<ListItem> = rows
                .iter()
                .map(|row| ListItem::new(row.menu_label()))
                .collect();
            items.push(ListItem::new("Create New Character"));

            let list = List::new(items)
                .block(block)
                .highlight_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("▶ ");
            let mut state = ListState::default();
            state.select(Some(index));
            f.render_stateful_widget(list, chunks[0], &mut state);

            let footer = Paragraph::new(
                "↑/↓ move  Enter select  Q quit  D/Delete remove highlighted profile",
            )
                .alignment(Alignment::Center);
            f.render_widget(footer, chunks[1]);
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(ch) => match ch.to_ascii_lowercase() {
                        'q' => return Ok(MenuSelection::CreateNew),
                        'w' | 'k' => {
                            if index > 0 {
                                index -= 1;
                            }
                        }
                        's' | 'j' => {
                            if index < last_index {
                                index += 1;
                            }
                        }
                        'd' => {
                            if index < rows.len() {
                                return Ok(MenuSelection::DeleteProfile(rows[index].id));
                            }
                        }
                        _ => {}
                    },
                    KeyCode::Up => {
                        if index > 0 {
                            index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if index < last_index {
                            index += 1;
                        }
                    }
                    KeyCode::Enter => {
                        if index == rows.len() {
                            return Ok(MenuSelection::CreateNew);
                        } else {
                            return Ok(MenuSelection::LoadExisting(rows[index].id));
                        }
                    }
                    KeyCode::Delete => {
                        if index < rows.len() {
                            return Ok(MenuSelection::DeleteProfile(rows[index].id));
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
