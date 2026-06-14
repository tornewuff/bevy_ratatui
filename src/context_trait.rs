use std::ops::Deref;

use bevy::prelude::Result;
use ratatui::{Terminal, prelude::Backend};

/// Trait for types that implement lifecycle functions for initializing a terminal context and
/// restoring the terminal state after exiting.
pub trait TerminalContext<T: Backend + 'static>:
    Sized + Send + Sync + Deref<Target = Terminal<T>> + 'static
{
    /// Initialize the terminal context.
    fn init() -> Result<Self>;
}
