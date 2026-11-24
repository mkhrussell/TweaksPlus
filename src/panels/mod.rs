//! The Tweaks panels. Each module builds one `AdwPreferencesPage`.

use crate::panel::Panel;
use libadwaita as adw;
use libadwaita::prelude::*;

/// Returns every panel in sidebar order.
pub fn all() -> Vec<Panel> {
    vec![welcome()]
}

fn welcome() -> Panel {
    let page = adw::PreferencesPage::new();
    let group = adw::PreferencesGroup::builder()
        .title("Welcome to TweaksPlus")
        .description("Panel implementations are added here.")
        .build();
    page.add(&group);

    Panel {
        id: "welcome",
        title: "Welcome",
        description: "Getting started",
        icon: Some("applications-system-symbolic"),
        keywords: &["home", "start"],
        page,
    }
}
