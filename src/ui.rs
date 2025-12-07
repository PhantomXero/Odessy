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

#[derive(Clone, Copy)]
struct ActionEntry {
    label: &'static str,
    hotkey: char,
    action: ProfileAction,
}

const PROFILE_ACTIONS: [ActionEntry; 8] = [
    ActionEntry {
        label: "Level Up",
        hotkey: 'l',
        action: ProfileAction::LevelUp,
    },
    ActionEntry {
        label: "Edit Identity",
        hotkey: 'i',
        action: ProfileAction::EditIdentity,
    },
    ActionEntry {
        label: "Edit History",
        hotkey: 'h',
        action: ProfileAction::EditHistory,
    },
    ActionEntry {
        label: "Edit Physical",
        hotkey: 'p',
        action: ProfileAction::EditPhysical,
    },
    ActionEntry {
        label: "Edit Vitality",
        hotkey: 'v',
        action: ProfileAction::EditVitality,
    },
    ActionEntry {
        label: "Edit Warrior",
        hotkey: 'w',
        action: ProfileAction::EditWarrior,
    },
    ActionEntry {
        label: "Save",
        hotkey: 's',
        action: ProfileAction::Save,
    },
    ActionEntry {
        label: "Quit",
        hotkey: 'q',
        action: ProfileAction::Quit,
    },
];

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
                            if index == 0 {
                                index = options_len - 1;
                            } else {
                                index -= 1;
                            }
                        }
                        's' | 'j' => {
                            index = (index + 1) % options_len;
                        }
                        _ => {}
                    },
                    KeyCode::Up => {
                        if index == 0 {
                            index = options_len - 1;
                        } else {
                            index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        index = (index + 1) % options_len;
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
    let mut action_index: usize = 0;
    let actions_height: u16 = PROFILE_ACTIONS.len() as u16 + 2;
    loop {
        terminal.draw(|f| {
            let size = f.area();
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Min(5),
                        Constraint::Length(actions_height),
                        Constraint::Length(2),
                    ]
                    .as_ref(),
                )
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

            let items: Vec<ListItem> = PROFILE_ACTIONS
                .iter()
                .map(|entry| {
                    let mut label = String::from("[");
                    label.push(entry.hotkey.to_ascii_uppercase());
                    label.push_str("] ");
                    label.push_str(entry.label);
                    ListItem::new(label)
                })
                .collect();
            let actions = List::new(items)
                .block(
                    Block::default()
                        .title("Actions")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Yellow)),
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("▶ ");
            let mut state = ListState::default();
            state.select(Some(action_index));
            f.render_stateful_widget(actions, layout[1], &mut state);

            let instructions = Paragraph::new(Line::from(vec![
                Span::raw("PgUp/PgDn scroll overview  "),
                Span::raw("↑/↓ or J/K move  "),
                Span::raw("Enter confirm  "),
                Span::raw("Hotkeys still apply (letters in brackets)."),
            ]))
            .alignment(Alignment::Center);
            f.render_widget(instructions, layout[2]);
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
            match key.code {
                KeyCode::Up => {
                    if action_index == 0 {
                        action_index = PROFILE_ACTIONS.len() - 1;
                    } else {
                        action_index -= 1;
                    }
                    continue;
                }
                KeyCode::Down => {
                    action_index = (action_index + 1) % PROFILE_ACTIONS.len();
                    continue;
                }
                KeyCode::Enter => {
                    return Ok(PROFILE_ACTIONS[action_index].action);
                }
                KeyCode::Char(ch) => {
                    let lower = ch.to_ascii_lowercase();
                    if lower == 'j' {
                        action_index = (action_index + 1) % PROFILE_ACTIONS.len();
                        continue;
                    }
                    if lower == 'k' {
                        if action_index == 0 {
                            action_index = PROFILE_ACTIONS.len() - 1;
                        } else {
                            action_index -= 1;
                        }
                        continue;
                    }
                    if let Some(entry) =
                        PROFILE_ACTIONS.iter().find(|entry| entry.hotkey == lower)
                    {
                        return Ok(entry.action);
                    }
                    continue;
                }
                _ => continue,
            }
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
