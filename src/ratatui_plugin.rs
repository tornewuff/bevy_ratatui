use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::CrosstermPlugin;

/// A plugin group that includes all the plugins in the Ratatui crate.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_ratatui::RatatuiPlugins;
///
/// App::new().add_plugins(RatatuiPlugins::default());
/// ```
pub struct RatatuiPlugins {
    /// Use kitty protocol if available and enabled.
    pub enable_kitty_protocol: bool,
    /// Capture mouse if enabled.
    pub enable_mouse_capture: bool,
    /// Forwards terminal input events to the bevy input system if enabled.
    pub enable_input_forwarding: bool,
}

impl Default for RatatuiPlugins {
    fn default() -> Self {
        Self {
            enable_kitty_protocol: true,
            enable_mouse_capture: false,
            enable_input_forwarding: false,
        }
    }
}

impl PluginGroup for RatatuiPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();

        #[cfg(all(feature = "crossterm", not(feature = "windowed")))]
        {
            builder = builder.add(CrosstermPlugin {
                enable_kitty_protocol: self.enable_kitty_protocol,
                enable_mouse_capture: self.enable_mouse_capture,
                enable_input_forwarding: self.enable_input_forwarding,
            });
        }

        builder
    }
}
