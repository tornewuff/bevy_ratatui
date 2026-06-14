use std::io::{Stdout, stdout};
use std::panic;
use std::sync::atomic::{AtomicBool, Ordering};

use bevy::prelude::*;

use ratatui::Terminal;
use ratatui::crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
    PushKeyboardEnhancementFlags,
};
use ratatui::crossterm::terminal::supports_keyboard_enhancement;
use ratatui::crossterm::{
    ExecutableCommand, cursor,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::backend::CrosstermBackend;

#[derive(Clone, Copy, Debug)]
pub struct CrosstermOptions {
    /// Use kitty protocol if available and enabled.
    pub enable_kitty_protocol: bool,
    /// Capture mouse if enabled.
    pub enable_mouse_capture: bool,
}

impl Default for CrosstermOptions {
    fn default() -> Self {
        Self {
            enable_kitty_protocol: true,
            enable_mouse_capture: false,
        }
    }
}

static TERMINAL_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Ratatui context that will draw to the terminal buffer using crossterm.
#[derive(Deref, DerefMut, Debug)]
pub struct CrosstermContext {
    #[deref]
    terminal: Terminal<CrosstermBackend<Stdout>>,
    options: CrosstermOptions,
}

impl CrosstermContext {
    pub fn new(mut options: CrosstermOptions) -> Result<Self> {
        if TERMINAL_INITIALIZED.swap(true, Ordering::Relaxed) {
            return Err("Only one CrosstermContext can exist at a time".into());
        }
        if options.enable_kitty_protocol && !supports_keyboard_enhancement()? {
            options.enable_kitty_protocol = false;
        }
        set_panic_hook(options);
        let mut stdout = stdout();
        enable_raw_mode()?;
        stdout.execute(EnterAlternateScreen)?;
        if options.enable_mouse_capture {
            stdout.execute(EnableMouseCapture)?;
        }
        if options.enable_kitty_protocol {
            stdout.execute(PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::all()))?;
        }
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal, options })
    }

    fn restore(options: CrosstermOptions) -> Result<()> {
        if !TERMINAL_INITIALIZED.swap(false, Ordering::Relaxed) {
            return Ok(());
        }

        let mut stdout = stdout();
        if options.enable_kitty_protocol {
            stdout.execute(PopKeyboardEnhancementFlags)?;
        }
        if options.enable_mouse_capture {
            stdout.execute(DisableMouseCapture)?;
        }
        stdout
            .execute(LeaveAlternateScreen)?
            .execute(cursor::Show)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn kitty_enabled(&self) -> bool {
        self.options.enable_kitty_protocol
    }

    pub fn mouse_enabled(&self) -> bool {
        self.options.enable_mouse_capture
    }
}

fn set_panic_hook(options: CrosstermOptions) {
    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = CrosstermContext::restore(options);
        panic_hook(panic_info);
    }));
}

impl Drop for CrosstermContext {
    fn drop(&mut self) {
        if let Err(err) = Self::restore(self.options) {
            eprintln!("Failed to restore terminal: {}", err);
        }
    }
}
