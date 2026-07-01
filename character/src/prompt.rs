use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use std::io::{self, stdout, Stdout, Write};

#[derive(Clone)]
pub struct MenuItem {
    pub label: String,
    pub info: Option<String>,
}

impl MenuItem {
    #[allow(dead_code)]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            info: None,
        }
    }

    pub fn with_info(label: impl Into<String>, info: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            info: Some(info.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MenuSelection {
    pub index: usize,
    pub label: String,
}

pub fn select_from_menu(title: &str, guide: Option<&str>, options: &[MenuItem]) -> MenuSelection {
    assert!(
        !options.is_empty(),
        "Interactive menu requires at least one option"
    );
    let mut frame = ScreenFrame::activate().expect("failed to enter interactive menu");
    let mut highlighted = 0usize;
    let mut show_info = false;

    loop {
        {
            let stdout = frame.stdout();
            execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).expect("clear failure");
            writeln!(stdout, "{title}").expect("write failure");
            if let Some(details) = guide {
                writeln!(stdout, "{details}").expect("write failure");
            }
            writeln!(
                stdout,
                "Use ↑/↓ to move, Enter to select, 'i' to toggle info, Esc to cancel."
            )
            .expect("write failure");
            for (idx, item) in options.iter().enumerate() {
                if idx == highlighted {
                    writeln!(stdout, "> {}", item.label).expect("write failure");
                } else {
                    writeln!(stdout, "  {}", item.label).expect("write failure");
                }
            }
            writeln!(stdout).expect("write failure");
            if show_info {
                if let Some(info) = &options[highlighted].info {
                    writeln!(stdout, "--- Info ---").expect("write failure");
                    writeln!(stdout, "{}", info).expect("write failure");
                } else {
                    writeln!(stdout, "No additional info for this option.").expect("write failure");
                }
            } else {
                writeln!(
                    stdout,
                    "(Press 'i' to view descriptive info for the highlighted option.)"
                )
                .expect("write failure");
            }
            stdout.flush().expect("flush failure");
        }

        if let Event::Key(key_event) = event::read().expect("event failure") {
            if !matches!(key_event.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                continue;
            }
            match key_event.code {
                KeyCode::Up => {
                    if highlighted == 0 {
                        highlighted = options.len() - 1;
                    } else {
                        highlighted -= 1;
                    }
                }
                KeyCode::Down => {
                    highlighted = (highlighted + 1) % options.len();
                }
                KeyCode::Enter => {
                    break;
                }
                KeyCode::Esc => {
                    break;
                }
                KeyCode::Char('i') | KeyCode::Char('I') => {
                    show_info = !show_info;
                }
                _ => {}
            }
        }
    }

    let label = options[highlighted].label.clone();
    MenuSelection {
        index: highlighted,
        label,
    }
}

struct ScreenFrame {
    stdout: Stdout,
}

impl ScreenFrame {
    fn activate() -> io::Result<Self> {
        let mut stdout = stdout();
        terminal::enable_raw_mode()
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        execute!(stdout, EnterAlternateScreen, Hide)?;
        Ok(Self { stdout })
    }

    fn stdout(&mut self) -> &mut Stdout {
        &mut self.stdout
    }
}

impl Drop for ScreenFrame {
    fn drop(&mut self) {
        let _ = execute!(&mut self.stdout, LeaveAlternateScreen, Show);
        let _ = terminal::disable_raw_mode();
    }
}
