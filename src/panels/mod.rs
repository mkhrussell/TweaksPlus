//! The Tweaks panels. Each module builds one `AdwPreferencesPage`.

mod appearance;
mod desktop;
mod fonts;
mod general;
mod keyboard;
mod mouse;
mod power;
mod sound;
mod startup;
mod top_bar;
mod windows;
mod workspaces;

use crate::panel::Panel;

/// Returns every panel in sidebar order.
pub fn all() -> Vec<Panel> {
    vec![
        general::build(),
        appearance::build(),
        fonts::build(),
        top_bar::build(),
        desktop::build(),
        workspaces::build(),
        windows::build(),
        keyboard::build(),
        mouse::build(),
        sound::build(),
        power::build(),
        startup::build(),
    ]
}
