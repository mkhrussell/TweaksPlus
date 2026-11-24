//! Windows panel: titlebar actions and buttons, window behaviour and focus.

use std::{cell::Cell, rc::Rc};

use crate::{panel::Panel, settings::rows};
use gio::prelude::*;
use glib::clone;
use gtk4::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;

const WM: &str = "org.gnome.desktop.wm.preferences";

pub fn build() -> Panel {
    let page = adw::PreferencesPage::builder().title("Windows").build();

    let actions = adw::PreferencesGroup::builder().title("Titlebar Actions").build();
    for (key, title) in [
        ("action-double-click-titlebar", "Double-Click"),
        ("action-middle-click-titlebar", "Middle-Click"),
        ("action-right-click-titlebar", "Secondary-Click"),
    ] {
        actions.add(&rows::combo(
            WM,
            key,
            title,
            None,
            &[
                ("none", "None"),
                ("toggle-shade", "Toggle Shade"),
                ("toggle-maximize", "Toggle Maximize"),
                ("toggle-maximize-horizontally", "Toggle Maximize Horizontally"),
                ("toggle-maximize-vertically", "Toggle Maximize Vertically"),
                ("minimize", "Minimize"),
                ("lower", "Lower"),
                ("menu", "Menu"),
            ],
        ));
    }

    let buttons = adw::PreferencesGroup::builder().title("Titlebar Buttons").build();
    let settings = gio::Settings::new(WM);
    buttons.add(&button_switch(&settings, "maximize", "Maximize"));
    buttons.add(&button_switch(&settings, "minimize", "Minimize"));
    buttons.add(&placement_row(&settings));

    let behaviour = adw::PreferencesGroup::builder().title("Click Actions").build();
    behaviour.add(&rows::switch(
        "org.gnome.mutter",
        "attach-modal-dialogs",
        "Attach Modal Dialogs",
        Some("Keep dialogs attached to their parent window"),
    ));
    behaviour.add(&rows::switch(
        "org.gnome.mutter",
        "center-new-windows",
        "Center New Windows",
        None,
    ));
    behaviour.add(&rows::combo(
        WM,
        "mouse-button-modifier",
        "Window Action Key",
        None,
        &[
            ("disabled", "Disabled"),
            ("<Alt>", "Alt"),
            ("<Super>", "Super"),
        ],
    ));
    behaviour.add(&rows::switch(
        WM,
        "resize-with-right-button",
        "Resize with Secondary-Click",
        None,
    ));

    let focus = adw::PreferencesGroup::builder()
        .title("Window Focus")
        .description("Which window becomes active when you point or click")
        .build();
    focus.add(&rows::choice_buttons(
        WM,
        "focus-mode",
        "Focus Mode",
        &[
            ("click", "Click"),
            ("sloppy", "Hover"),
            ("mouse", "Follows Mouse"),
        ],
    ));

    let raise = rows::switch(
        WM,
        "auto-raise",
        "Raise Windows When Focused",
        Some("Only available with hover or follow-mouse focus"),
    );
    let focus_settings = gio::Settings::new(WM);
    let sync_sensitivity = clone!(#[weak] raise, move |settings: &gio::Settings| {
        let mode = settings.string("focus-mode");
        raise.set_sensitive(mode.as_str() != "click");
    });
    sync_sensitivity(&focus_settings);
    focus_settings.connect_changed(Some("focus-mode"), move |settings, _| {
        sync_sensitivity(settings);
    });
    focus.add(&raise);

    page.add(&actions);
    page.add(&buttons);
    page.add(&behaviour);
    page.add(&focus);

    Panel {
        id: "windows",
        title: "Windows",
        description: "Titlebars, focus and window behaviour",
        icon: Some("window-new-symbolic"),
        keywords: &[
            "window", "titlebar", "button", "focus", "maximize", "minimize", "modal", "resize",
        ],
        page,
    }
}

fn split_layout(layout: &str) -> (Vec<String>, Vec<String>) {
    let (left, right) = layout.split_once(':').unwrap_or((layout, ""));
    let parse = |side: &str| -> Vec<String> {
        side.split(',')
            .filter(|part| !part.is_empty())
            .map(str::to_owned)
            .collect()
    };
    (parse(left), parse(right))
}

fn button_present(layout: &str, button: &str) -> bool {
    let (left, right) = split_layout(layout);
    left.iter().chain(&right).any(|value| value == button)
}

fn set_button_present(settings: &gio::Settings, button: &str, present: bool) {
    let (mut left, mut right) = split_layout(&settings.string("button-layout"));
    let target = if right.iter().any(|value| value == "close") {
        &mut right
    } else {
        &mut left
    };
    if present {
        if !target.iter().any(|value| value == button) {
            target.push(button.to_owned());
        }
    } else {
        target.retain(|value| value != button);
    }
    let _ = settings.set_string(
        "button-layout",
        &format!("{}:{}", left.join(","), right.join(",")),
    );
}

fn button_switch(
    settings: &gio::Settings,
    button: &'static str,
    title: &str,
) -> adw::SwitchRow {
    let row = adw::SwitchRow::builder()
        .title(title)
        .active(button_present(&settings.string("button-layout"), button))
        .build();

    row.connect_active_notify(clone!(#[strong] settings, move |row| {
        set_button_present(&settings, button, row.is_active());
    }));

    settings.connect_changed(
        Some("button-layout"),
        clone!(#[weak] row, move |settings, _| {
            let active = button_present(&settings.string("button-layout"), button);
            if row.is_active() != active {
                row.set_active(active);
            }
        }),
    );

    row
}

fn placement_row(settings: &gio::Settings) -> adw::ActionRow {
    let left = gtk4::ToggleButton::builder().label("Left").build();
    let right = gtk4::ToggleButton::builder().label("Right").build();
    right.set_group(Some(&left));

    let updating = Rc::new(Cell::new(false));
    let sync = clone!(#[weak] left, #[weak] right, #[strong] updating, move |settings: &gio::Settings| {
        let (left_buttons, right_buttons) = split_layout(&settings.string("button-layout"));
        let on_right = right_buttons.iter().any(|value| value == "close")
            || (!left_buttons.is_empty() && right_buttons.is_empty());
        if on_right != right.is_active() {
            updating.set(true);
            right.set_active(on_right);
            left.set_active(!on_right);
            updating.set(false);
        }
    });
    sync(settings);
    settings.connect_changed(Some("button-layout"), move |settings, _| sync(settings));

    let move_buttons = clone!(#[strong] settings, move |to_right: bool| {
        let (left_buttons, right_buttons) = split_layout(&settings.string("button-layout"));
        if to_right && !left_buttons.is_empty() {
            let mut all = left_buttons;
            all.extend(right_buttons);
            let _ = settings.set_string("button-layout", &format!(":{}", all.join(",")));
        } else if !to_right && !right_buttons.is_empty() {
            let mut all = right_buttons;
            all.extend(left_buttons);
            let _ = settings.set_string("button-layout", &format!("{}:", all.join(",")));
        }
    });

    left.connect_toggled(clone!(#[strong] move_buttons, #[strong] updating, move |button| {
        if button.is_active() && !updating.get() {
            move_buttons(false);
        }
    }));
    right.connect_toggled(clone!(#[strong] move_buttons, #[strong] updating, move |button| {
        if button.is_active() && !updating.get() {
            move_buttons(true);
        }
    }));

    let container = gtk4::Box::builder()
        .spacing(0)
        .valign(gtk4::Align::Center)
        .css_classes(["linked"])
        .build();
    container.append(&left);
    container.append(&right);

    let row = adw::ActionRow::builder().title("Placement").build();
    row.add_suffix(&container);
    row
}
