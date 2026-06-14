use std::io::{Stdout, stdout};
use std::sync::atomic::{AtomicBool, Ordering};

use bevy::prelude::*;

use ratatui::Terminal;
use ratatui::crossterm::{
    ExecutableCommand, cursor,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::backend::CrosstermBackend;

use crate::context::TerminalContext;

static TERMINAL_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Ratatui context that will draw to the terminal buffer using crossterm.
#[derive(Deref, DerefMut, Debug)]
pub struct CrosstermContext(Terminal<CrosstermBackend<Stdout>>);

impl TerminalContext<CrosstermBackend<Stdout>> for CrosstermContext {
    fn init() -> Result<Self> {
        if TERMINAL_INITIALIZED.swap(true, Ordering::Relaxed) {
            return Err("Only one CrosstermContext can exist at a time".into());
        }

        let mut stdout = stdout();
        stdout.execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self(terminal))
    }
}

impl CrosstermContext {
    pub(super) fn restore() -> Result<()> {
        if TERMINAL_INITIALIZED.swap(false, Ordering::Relaxed) {
            let mut stdout = stdout();
            stdout
                .execute(LeaveAlternateScreen)?
                .execute(cursor::Show)?;
            disable_raw_mode()?;
        }
        Ok(())
    }
}

impl Drop for CrosstermContext {
    fn drop(&mut self) {
        if let Err(err) = Self::restore() {
            eprintln!("Failed to restore terminal: {}", err);
        }
    }
}
