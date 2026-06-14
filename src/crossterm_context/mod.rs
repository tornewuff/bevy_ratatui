use bevy::prelude::*;

use crate::RatatuiContext;
use crate::context::CrosstermContext;
use crate::crossterm_context::context::CrosstermOptions;

pub mod context;
pub mod event;

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
        app.add_plugins(event::EventPlugin::default());

        #[cfg(feature = "keyboard")]
        if self.enable_input_forwarding {
            app.add_plugins(translation::TranslationPlugin);
        }

        let &Self {
            enable_kitty_protocol,
            enable_mouse_capture,
            ..
        } = self;

        app.add_systems(Startup, move |mut commands: Commands| -> Result {
            let context = CrosstermContext::new(CrosstermOptions {
                enable_kitty_protocol,
                enable_mouse_capture,
            })?;
            commands.insert_resource(RatatuiContext(context));
            Ok(())
        });
    }
}
