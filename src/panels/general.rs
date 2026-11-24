//! General panel: animations, pointer and visual effects.

use crate::{panel::Panel, settings::rows};
use libadwaita as adw;
use libadwaita::prelude::*;

pub fn build() -> Panel {
    let page = adw::PreferencesPage::builder().title("General").build();

    let group = adw::PreferencesGroup::builder()
        .title("General")
        .description("Desktop-wide behaviour")
        .build();
    group.add(&rows::switch(
        "org.gnome.desktop.interface",
        "enable-animations",
        "Animations",
        Some("Enable window and workspace transition animations"),
    ));
    group.add(&rows::switch(
        "org.gnome.desktop.sound",
        "allow-volume-above-100-percent",
        "Over-Amplification",
        Some("Allow sound volume above 100 percent"),
    ));
    group.add(&rows::switch(
        "org.gnome.desktop.interface",
        "cursor-blink",
        "Cursor Blinking",
        None,
    ));
    group.add(&rows::switch(
        "org.gnome.desktop.interface",
        "locate-pointer",
        "Locate Pointer",
        Some("Highlight the pointer when Ctrl is pressed"),
    ));
    group.add(&rows::switch(
        "org.gnome.desktop.interface",
        "enable-hot-corners",
        "Hot Corners",
        Some("Trigger the overview when moving to the top-left corner"),
    ));

    page.add(&group);

    Panel {
        id: "general",
        title: "General",
        description: "Animations and desktop-wide behaviour",
        icon: Some("preferences-system-symbolic"),
        keywords: &["general", "animation", "volume", "cursor", "pointer", "hot corner"],
        page,
    }
}
