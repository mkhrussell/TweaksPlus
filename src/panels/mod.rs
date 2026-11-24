//! The Tweaks panels. Each module builds one `AdwPreferencesPage`.

mod appearance;
mod fonts;
mod keyboard;
mod mouse;
mod windows;

use crate::panel::Panel;

/// Returns every panel in sidebar order.
pub fn all() -> Vec<Panel> {
    vec![
        appearance::build(),
        fonts::build(),
        windows::build(),
        keyboard::build(),
        mouse::build(),
    ]
}
