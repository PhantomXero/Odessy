use crate::storage::CharacterSummary;
use character::Character;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{ExecutableCommand, execute};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use std::io::{self, Stdout};
use std::time::Duration;

pub enum MenuSelection {
    CreateNew,
    LoadExisting(i64),
}

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

pub fn character_selector(rows: &[CharacterSummary]) -> io::Result<MenuSelection> {
    if rows.is_empty() {
        return Ok(MenuSelection::CreateNew);
    }

    let mut terminal = prepare_terminal()?;
    let result = selector_loop(&mut terminal, rows);
    restore_terminal(terminal)?;
    result
}

pub fn profile_screen(character: &Character) -> io::Result<ProfileAction> {
    let mut terminal = prepare_terminal()?;
    let result = profile_loop(&mut terminal, character);
    restore_terminal(terminal)?;
    result
}

fn selector_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    rows: &[CharacterSummary],
) -> io::Result<MenuSelection> {
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
                .map(|row| ListItem::new(format!("{} (ID {})", row.name, row.id)))
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

            let footer = Paragraph::new("Use ↑/↓ or W/S to move, Enter to select, Q to quit.")
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
                    _ => {}
                }
            }
        }
    }
}

fn profile_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    character: &Character,
) -> io::Result<ProfileAction> {
    let mut scroll_offset: u16 = 0;
    const SCROLL_STEP: u16 = 3;
    loop {
        terminal.draw(|f| {
            let size = f.area();
            let layout = Layout::default()
                .direction(Direction::Vertical)
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
                Span::raw("Hotkeys: [L] Level  [I] Identity  [H] History  [P] Physical  [V] Vitality  [W] Warrior  [S] Save  [Q] Quit"),
            ]))
            .alignment(Alignment::Center);
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

fn prepare_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    Ok(terminal)
}

fn restore_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    terminal.show_cursor()?;
    drop(terminal);
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
