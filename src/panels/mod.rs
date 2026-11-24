//! The Tweaks panels. Each module builds one `AdwPreferencesPage`.

mod appearance;
mod fonts;

use crate::panel::Panel;

/// Returns every panel in sidebar order.
pub fn all() -> Vec<Panel> {
    vec![appearance::build(), fonts::build()]
}
