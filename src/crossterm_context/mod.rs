use bevy::prelude::*;

pub mod cleanup;
pub mod context;
pub mod error;
pub mod event;
pub mod kitty;
#[cfg(feature = "mouse")]
pub mod mouse;

#[cfg(feature = "keyboard")]
pub mod translation;

pub struct CrosstermPlugin {
    /// Use kitty protocol if available and enabled.
    pub enable_kitty_protocol: bool,
    /// Capture mouse if enabled.
    pub enable_mouse_capture: bool,
    /// Forwards terminal input events to the bevy input system if enabled.
    pub enable_input_forwarding: bool,
}

impl Default for CrosstermPlugin {
    fn default() -> Self {
        Self {
            enable_kitty_protocol: true,
            enable_mouse_capture: false,
            enable_input_forwarding: false,
        }
    }
}

impl Plugin for CrosstermPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            cleanup::CleanupPlugin,
            error::ErrorPlugin,
            event::EventPlugin::default(),
        ));

        if self.enable_kitty_protocol {
            app.add_plugins(kitty::KittyPlugin);
        }

        #[cfg(feature = "mouse")]
        if self.enable_mouse_capture {
            app.add_plugins(mouse::MousePlugin);
        }

        #[cfg(feature = "keyboard")]
        if self.enable_input_forwarding {
            app.add_plugins(translation::TranslationPlugin);
        }
    }
}
