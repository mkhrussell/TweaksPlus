//! Sound panel: system sound theme and event sounds.

use crate::{
    panel::Panel,
    settings::{discover, rows},
};
use libadwaita as adw;
use libadwaita::prelude::*;

pub fn build() -> Panel {
    let page = adw::PreferencesPage::builder().title("Sound").build();

    let group = adw::PreferencesGroup::builder().title("System Sounds").build();
    group.add(&rows::combo_owned(
        "org.gnome.desktop.sound",
        "theme-name",
        "System Sound Theme",
        Some("Theme used for sound events"),
        discover::sound_themes(),
    ));
    group.add(&rows::switch(
        "org.gnome.desktop.sound",
        "event-sounds",
        "Event Sounds",
        Some("Play sounds for system events"),
    ));
    group.add(&rows::switch(
        "org.gnome.desktop.sound",
        "input-feedback-sounds",
        "Input Feedback Sounds",
        Some("Play sounds when typing or clicking"),
    ));
    page.add(&group);

    Panel {
        id: "sound",
        title: "Sound",
        description: "Sound themes and event sounds",
        icon: Some("audio-volume-high-symbolic"),
        keywords: &["sound", "theme", "event", "audio", "bell"],
        page,
    }
}
