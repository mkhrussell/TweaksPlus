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
    } else {
        // Ubuntu ships Desktop Icons NG by default; CachyOS/Arch users can
        // install it from the repositories if they want these extra options.
        let hint = crate::settings::environment::extension_hint(
            Some("gnome-shell-extension-desktop-icons-ng"),
            "Install the Desktop Icons NG extension for more icon options",
        );
        icons.add(
            &adw::ActionRow::builder()
                .title("Additional Icon Options")
                .subtitle(hint)
                .sensitive(false)
                .build(),
        );
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
