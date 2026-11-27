//! Keyboard panel: input sources, Emacs input and XKB layout options.

use crate::{panel::Panel, settings::rows};
use gio::prelude::*;
use glib::clone;
use gtk4::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;

const INPUT_SOURCES: &str = "org.gnome.desktop.input-sources";

/// `(group, XKB option, label)`. Only one option per group can be active.
const XKB_OPTIONS: &[(&str, &str, &str)] = &[
    ("caps", "caps:swapescape", "Caps Lock: Swap with Escape"),
    ("caps", "caps:escape", "Caps Lock: Escape"),
    ("caps", "caps:none", "Caps Lock: Disabled"),
    ("compose", "compose:ralt", "Compose Key: Right Alt"),
    ("compose", "compose:rctrl", "Compose Key: Right Ctrl"),
    ("compose", "compose:menu", "Compose Key: Menu"),
    ("ctrl", "ctrl:swap_lalt_lctrl", "Ctrl: Swap with Left Alt"),
    ("terminate", "terminate:ctrl_alt_bksp", "Ctrl+Alt+Backspace"),
];

pub fn build() -> Panel {
    let page = adw::PreferencesPage::builder().title("Keyboard").build();

    let general = adw::PreferencesGroup::builder().title("General").build();
    general.add(&rows::switch(
        INPUT_SOURCES,
        "show-all-sources",
        "Show Extended Input Sources",
        Some("Increases the choice of input sources in Settings"),
    ));
    general.add(&rows::value_switch(
        "org.gnome.desktop.interface",
        "gtk-key-theme",
        "Emacs Input",
        Some("Use Emacs keybindings in GTK applications"),
        "Emacs",
        "Default",
    ));

    let layout = adw::PreferencesGroup::builder().title("Layout").build();
    layout.add(&rows::choice_buttons(
        "org.gnome.mutter",
        "overlay-key",
        "Overview Shortcut",
        &[("Super_L", "Left Super"), ("Super_R", "Right Super")],
    ));

    let button = gtk4::Button::builder()
        .label("Additional Layout Options")
        .halign(gtk4::Align::End)
        .valign(gtk4::Align::Center)
        .build();
    button.connect_clicked(|button| {
        show_layout_options(button.root().and_downcast::<gtk4::Window>().as_ref());
    });
    let button_row = adw::ActionRow::builder().title("Layout Options").build();
    button_row.add_suffix(&button);
    layout.add(&button_row);

    page.add(&general);
    page.add(&layout);

    Panel {
        id: "keyboard",
        title: "Keyboard",
        description: "Input sources and layout options",
        icon: Some("input-keyboard-symbolic"),
        keywords: &["keyboard", "input", "layout", "emacs", "xkb", "caps", "compose", "super"],
        page,
    }
}

fn show_layout_options(parent: Option<&gtk4::Window>) {
    let settings = crate::settings::settings_for(INPUT_SOURCES, "xkb-options");
    let group = adw::PreferencesGroup::builder()
        .title("Layout Options")
        .description("Common XKB options for keyboard layouts")
        .build();
    if let Some(settings) = &settings {
        for (xkb_group, value, label) in XKB_OPTIONS {
            group.add(&xkb_option_row(settings, xkb_group, value, label));
        }
    } else {
        group.add(
            &adw::ActionRow::builder()
                .title("Layout options are not available in this GNOME version")
                .build(),
        );
    }

    let dialog = adw::Dialog::builder()
        .title("Additional Layout Options")
        .child(&group)
        .build();
    let close = gtk4::Button::builder()
        .icon_name("window-close-symbolic")
        .tooltip_text("Close")
        .css_classes(["flat"])
        .valign(gtk4::Align::Center)
        .build();
    close.connect_clicked(glib::clone!(#[weak] dialog, move |_| {
        dialog.close();
    }));
    group.set_header_suffix(Some(&close));
    dialog.present(parent);
}

fn xkb_option_row(
    settings: &gio::Settings,
    group: &'static str,
    value: &'static str,
    title: &str,
) -> adw::SwitchRow {
    let is_set = move |settings: &gio::Settings| {
        settings
            .strv("xkb-options")
            .iter()
            .any(|option| option.as_str() == value)
    };
    let row = adw::SwitchRow::builder()
        .title(title)
        .active(is_set(settings))
        .build();

    row.connect_active_notify(clone!(#[strong] settings, move |row| {
        let enabled = row.is_active();
        let mut options: Vec<String> = settings
            .strv("xkb-options")
            .iter()
            .map(|option| option.to_string())
            .collect();

        // Options in the same group are mutually exclusive.
        options.retain(|option| {
            let in_group = XKB_OPTIONS
                .iter()
                .any(|(candidate_group, candidate, _)| {
                    *candidate_group == group && *candidate == option.as_str()
                });
            !in_group || option.as_str() == value
        });

        if enabled {
            if !options.iter().any(|option| option == value) {
                options.push(value.to_owned());
            }
        } else {
            options.retain(|option| option != value);
        }

        let values: Vec<&str> = options.iter().map(String::as_str).collect();
        let _ = settings.set_strv("xkb-options", values.as_slice());
    }));

    settings.connect_changed(
        Some("xkb-options"),
        clone!(#[weak] row, move |settings, _| {
            let active = is_set(settings);
            if row.is_active() != active {
                row.set_active(active);
            }
        }),
    );

    row
}
