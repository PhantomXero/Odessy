use character::Character;
use crossterm::event::{self, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Terminal;
use std::io::Stdout;

use super::terminal::{prepare_terminal, restore_terminal};

#[derive(Debug, Clone, Copy)]
pub enum ProfileAction {
    Quit,
    LevelUp,
    EditIdentity,
    EditHistory,
    EditPhysical,
    EditVitality,
    EditWarrior,
    Save,
}

pub fn profile_screen(character: &Character) -> std::io::Result<ProfileAction> {
    let mut terminal = prepare_terminal()?;
    let result = profile_loop(&mut terminal, character);
    restore_terminal(terminal)?;
    result
}

fn profile_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    character: &Character,
) -> std::io::Result<ProfileAction> {
    let mut scroll_offset: u16 = 0;
    const SCROLL_STEP: u16 = 3;
    loop {
        terminal.draw(|f| {
            let size = f.area();
            let layout = Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([Constraint::Min(5), Constraint::Length(3)].as_ref())
                .split(size);

            let block = Block::default()
                .title("Character Overview")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan));
            let body = Paragraph::new(character.profile_card())
                .wrap(Wrap { trim: true })
                .scroll((scroll_offset, 0))
                .block(block);
            f.render_widget(body, layout[0]);

            let instructions = Paragraph::new(Line::from(vec![
                Span::raw("↑/↓ scroll  "),
                Span::raw("PgUp/PgDn fast scroll  "),
                Span::raw(
                    "Hotkeys: [L] Level  [I] Identity  [H] History  [P] Physical  [V] Vitality  [W] Warrior  [S] Save  [Q] Quit",
                ),
            ]))
            .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(instructions, layout[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::PageUp {
                scroll_offset = scroll_offset.saturating_sub(SCROLL_STEP);
                continue;
            }
            if key.code == KeyCode::PageDown {
                scroll_offset = scroll_offset.saturating_add(SCROLL_STEP);
                continue;
            }
            if key.code == KeyCode::Up {
                scroll_offset = scroll_offset.saturating_sub(1);
                continue;
            }
            if key.code == KeyCode::Down {
                scroll_offset = scroll_offset.saturating_add(1);
                continue;
            }
            let action = match key.code {
                KeyCode::Char(ch) => match ch.to_ascii_lowercase() {
                    'q' => ProfileAction::Quit,
                    'l' => ProfileAction::LevelUp,
                    'i' => ProfileAction::EditIdentity,
                    'h' => ProfileAction::EditHistory,
                    'p' => ProfileAction::EditPhysical,
                    'v' => ProfileAction::EditVitality,
                    'w' => ProfileAction::EditWarrior,
                    's' => ProfileAction::Save,
                    _ => continue,
                },
                _ => continue,
            };
            return Ok(action);
        }
    }
}
