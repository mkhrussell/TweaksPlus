//! Appearance panel: styles and desktop backgrounds.

use crate::{
    panel::Panel,
    settings::{discover, rows, schema_available},
};
use libadwaita as adw;
use libadwaita::prelude::*;

const SHELL_THEME_SCHEMA: &str = "org.gnome.shell.extensions.user-theme";

pub fn build() -> Panel {
    let page = adw::PreferencesPage::builder().title("Appearance").build();

    let styles = adw::PreferencesGroup::builder()
        .title("Styles")
        .description("Themes installed on your system")
        .build();
    styles.add(&rows::combo_owned(
        "org.gnome.desktop.interface",
        "cursor-theme",
        "Cursor",
        None,
        discover::cursor_themes(),
    ));
    styles.add(&rows::combo_owned(
        "org.gnome.desktop.interface",
        "icon-theme",
        "Icons",
        None,
        discover::icon_themes(),
    ));
    styles.add(&shell_theme_row());
    styles.add(&rows::combo_owned(
        "org.gnome.desktop.interface",
        "gtk-theme",
        "Legacy Applications",
        Some("GTK 3 and GTK 4 applications that honour themes"),
        discover::gtk_themes(),
    ));

    let background = adw::PreferencesGroup::builder()
        .title("Background")
        .description("Desktop wallpaper")
        .build();
    let images = &["image/png", "image/jpeg", "application/xml"];
    background.add(&rows::file(
        "org.gnome.desktop.background",
        "picture-uri",
        "Default Image",
        images,
    ));
    background.add(&rows::file(
        "org.gnome.desktop.background",
        "picture-uri-dark",
        "Dark Style Image",
        images,
    ));
    background.add(&rows::combo(
        "org.gnome.desktop.background",
        "picture-options",
        "Adjustment",
        None,
        &[
            ("none", "None"),
            ("wallpaper", "Wallpaper"),
            ("centered", "Centered"),
            ("scaled", "Scaled"),
            ("stretched", "Stretched"),
            ("zoom", "Zoom"),
            ("spanned", "Spanned"),
        ],
    ));

    page.add(&styles);
    page.add(&background);

    Panel {
        id: "appearance",
        title: "Appearance",
        description: "Themes, icons, cursor and wallpaper",
        icon: Some("applications-graphics-symbolic"),
        keywords: &["theme", "icon", "cursor", "wallpaper", "background", "dark", "shell"],
        page,
    }
}

/// Shell themes require the User Themes extension to be installed and enabled.
fn shell_theme_row() -> gtk4::Widget {
    if schema_available(SHELL_THEME_SCHEMA) {
        rows::combo_owned(
            SHELL_THEME_SCHEMA,
            "name",
            "Shell",
            Some("Requires the User Themes extension"),
            discover::shell_themes(),
        )
        .upcast()
    } else {
        let hint = crate::settings::environment::extension_hint(
            None,
            "Install the User Themes extension to change the Shell theme",
        );
        adw::ActionRow::builder()
            .title("Shell")
            .subtitle(hint)
            .sensitive(false)
            .build()
            .upcast()
    }
}
