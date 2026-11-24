//! Desktop panel: desktop icons and the Desktop Icons NG extension.

use crate::{
    panel::Panel,
    settings::{rows, schema_available},
};
use libadwaita as adw;
use libadwaita::prelude::*;

const DING: &str = "org.gnome.shell.extensions.ding";

pub fn build() -> Panel {
    let page = adw::PreferencesPage::builder().title("Desktop").build();

    let icons = adw::PreferencesGroup::builder().title("Desktop Icons").build();
    icons.add(&rows::switch(
        "org.gnome.desktop.background",
        "show-desktop-icons",
        "Show Desktop Icons",
        Some("Requires a desktop-icons extension on Wayland"),
    ));

    if schema_available(DING) {
        icons.add(&rows::switch(
            DING,
            "show-home",
            "Show Personal Directory",
            None,
        ));
        icons.add(&rows::switch(DING, "show-trash", "Show Trash", None));
        icons.add(&rows::switch(
            DING,
            "show-network-volumes",
            "Show Network Volumes",
            None,
        ));
        icons.add(&rows::switch(
            DING,
            "show-volumes",
            "Show Mounted Volumes",
            None,
        ));
        icons.add(&rows::switch(
            DING,
            "show-drop-place",
            "Show Drop Place",
            None,
        ));
        icons.add(&rows::combo(
            DING,
            "icon-size",
            "Icon Size",
            None,
            &[
                ("tiny", "Tiny"),
                ("small", "Small"),
                ("standard", "Standard"),
                ("large", "Large"),
            ],
        ));
    }

    page.add(&icons);

    Panel {
        id: "desktop",
        title: "Desktop",
        description: "Desktop icons and the desktop-icons extension",
        icon: Some("folder-symbolic"),
        keywords: &["desktop", "icon", "trash", "volume", "home", "folder"],
        page,
    }
}
