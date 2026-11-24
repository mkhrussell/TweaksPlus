//! Widgets that bind a GSettings key to a libadwaita preference row.

use std::{cell::Cell, rc::Rc};

use gio::prelude::*;
use glib::clone;
use gtk4::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;

/// A `(stored value, visible label)` pair used by choice widgets.
pub type Choice = (&'static str, &'static str);

/// A boolean switch row.
pub fn switch(schema: &str, key: &str, title: &str, subtitle: Option<&str>) -> adw::SwitchRow {
    let settings = gio::Settings::new(schema);
    let key = key.to_owned();
    let row = adw::SwitchRow::builder()
        .title(title)
        .active(settings.boolean(&key))
        .build();
    if let Some(subtitle) = subtitle {
        row.set_subtitle(subtitle);
    }

    row.connect_active_notify(clone!(#[strong] settings, #[strong] key, move |row| {
        let value = row.is_active();
        if settings.boolean(&key) != value {
            let _ = settings.set_boolean(&key, value);
        }
    }));

    settings.connect_changed(
        Some(&key),
        clone!(#[weak] row, #[strong] key, move |settings, _| {
            let value = settings.boolean(&key);
            if row.is_active() != value {
                row.set_active(value);
            }
        }),
    );

    row
}

/// A switch that maps two specific string values to on/off.
pub fn value_switch(
    schema: &str,
    key: &str,
    title: &str,
    subtitle: Option<&str>,
    on_value: &'static str,
    off_value: &'static str,
) -> adw::SwitchRow {
    let settings = gio::Settings::new(schema);
    let key = key.to_owned();
    let row = adw::SwitchRow::builder()
        .title(title)
        .active(settings.string(&key).as_str() == on_value)
        .build();
    if let Some(subtitle) = subtitle {
        row.set_subtitle(subtitle);
    }

    row.connect_active_notify(clone!(#[strong] settings, #[strong] key, move |row| {
        let value = if row.is_active() { on_value } else { off_value };
        if settings.string(&key).as_str() != value {
            let _ = settings.set_string(&key, value);
        }
    }));

    settings.connect_changed(
        Some(&key),
        clone!(#[weak] row, #[strong] key, move |settings, _| {
            let active = settings.string(&key).as_str() == on_value;
            if row.is_active() != active {
                row.set_active(active);
            }
        }),
    );

    row
}

/// A switch that is on for every value except `off_value`.
///
/// Used for pointer acceleration profiles: switching off selects the flat
/// profile, switching on restores the schema default.
pub fn enum_switch(
    schema: &str,
    key: &str,
    title: &str,
    subtitle: Option<&str>,
    off_value: &'static str,
) -> adw::SwitchRow {
    let settings = gio::Settings::new(schema);
    let key = key.to_owned();
    let row = adw::SwitchRow::builder()
        .title(title)
        .active(settings.string(&key).as_str() != off_value)
        .build();
    if let Some(subtitle) = subtitle {
        row.set_subtitle(subtitle);
    }

    row.connect_active_notify(clone!(#[strong] settings, #[strong] key, move |row| {
        if row.is_active() {
            settings.reset(&key);
        } else {
            let _ = settings.set_string(&key, off_value);
        }
    }));

    settings.connect_changed(
        Some(&key),
        clone!(#[weak] row, #[strong] key, move |settings, _| {
            let active = settings.string(&key).as_str() != off_value;
            if row.is_active() != active {
                row.set_active(active);
            }
        }),
    );

    row
}

/// A drop-down row for string-valued settings.
pub fn combo(
    schema: &str,
    key: &str,
    title: &str,
    subtitle: Option<&str>,
    choices: &[Choice],
) -> adw::ComboRow {
    combo_owned(
        schema,
        key,
        title,
        subtitle,
        choices
            .iter()
            .map(|(value, label)| (value.to_string(), label.to_string()))
            .collect(),
    )
}

/// A drop-down row whose options are discovered at runtime (themes, sounds…).
pub fn combo_owned(
    schema: &str,
    key: &str,
    title: &str,
    subtitle: Option<&str>,
    choices: Vec<(String, String)>,
) -> adw::ComboRow {
    let settings = gio::Settings::new(schema);
    let key = key.to_owned();
    let model = gtk4::StringList::new(&[]);
    for (_, label) in &choices {
        model.append(label);
    }

    let current = settings.string(&key);
    let selected = choices
        .iter()
        .position(|(value, _)| value == current.as_str())
        .unwrap_or(0) as u32;

    let row = adw::ComboRow::builder()
        .title(title)
        .model(&model)
        .selected(selected)
        .build();
    if let Some(subtitle) = subtitle {
        row.set_subtitle(subtitle);
    }

    row.connect_selected_notify(clone!(#[strong] settings, #[strong] choices, #[strong] key, move |row| {
        if let Some((value, _)) = choices.get(row.selected() as usize) {
            if settings.string(&key).as_str() != value.as_str() {
                let _ = settings.set_string(&key, value);
            }
        }
    }));

    settings.connect_changed(
        Some(&key),
        clone!(#[weak] row, #[strong] choices, #[strong] key, move |settings, _| {
            let value = settings.string(&key);
            if let Some(index) = choices.iter().position(|(stored, _)| stored == value.as_str()) {
                if row.selected() != index as u32 {
                    row.set_selected(index as u32);
                }
            }
        }),
    );

    row
}

/// A numeric row for floating-point settings.
#[allow(clippy::too_many_arguments)]
pub fn spin(
    schema: &str,
    key: &str,
    title: &str,
    subtitle: Option<&str>,
    min: f64,
    max: f64,
    step: f64,
    digits: u32,
) -> adw::SpinRow {
    let settings = gio::Settings::new(schema);
    let key = key.to_owned();
    let adjustment = gtk4::Adjustment::new(settings.double(&key), min, max, step, step, 0.0);
    let row = adw::SpinRow::builder()
        .title(title)
        .adjustment(&adjustment)
        .digits(digits)
        .build();
    if let Some(subtitle) = subtitle {
        row.set_subtitle(subtitle);
    }

    row.connect_value_notify(clone!(#[strong] settings, #[strong] key, move |row| {
        let value = row.value();
        if (settings.double(&key) - value).abs() > f64::EPSILON {
            let _ = settings.set_double(&key, value);
        }
    }));

    settings.connect_changed(
        Some(&key),
        clone!(#[weak] row, #[strong] key, move |settings, _| {
            let value = settings.double(&key);
            if (row.value() - value).abs() > f64::EPSILON {
                row.set_value(value);
            }
        }),
    );

    row
}

/// A numeric row for integer-valued settings.
pub fn spin_int(
    schema: &str,
    key: &str,
    title: &str,
    subtitle: Option<&str>,
    min: i32,
    max: i32,
    step: i32,
) -> adw::SpinRow {
    let settings = gio::Settings::new(schema);
    let key = key.to_owned();
    let adjustment = gtk4::Adjustment::new(
        settings.int(&key) as f64,
        min as f64,
        max as f64,
        step as f64,
        step as f64,
        0.0,
    );
    let row = adw::SpinRow::builder()
        .title(title)
        .adjustment(&adjustment)
        .digits(0)
        .build();
    if let Some(subtitle) = subtitle {
        row.set_subtitle(subtitle);
    }

    row.connect_value_notify(clone!(#[strong] settings, #[strong] key, move |row| {
        let value = row.value() as i32;
        if settings.int(&key) != value {
            let _ = settings.set_int(&key, value);
        }
    }));

    settings.connect_changed(
        Some(&key),
        clone!(#[weak] row, #[strong] key, move |settings, _| {
            let value = settings.int(&key) as f64;
            if (row.value() - value).abs() > f64::EPSILON {
                row.set_value(value);
            }
        }),
    );

    row
}

/// A numeric row for unsigned integer settings (for example, idle delays).
pub fn spin_uint(
    schema: &str,
    key: &str,
    title: &str,
    subtitle: Option<&str>,
    min: u32,
    max: u32,
    step: u32,
) -> adw::SpinRow {
    let settings = gio::Settings::new(schema);
    let key = key.to_owned();
    let adjustment = gtk4::Adjustment::new(
        settings.uint(&key) as f64,
        min as f64,
        max as f64,
        step as f64,
        step as f64,
        0.0,
    );
    let row = adw::SpinRow::builder()
        .title(title)
        .adjustment(&adjustment)
        .digits(0)
        .build();
    if let Some(subtitle) = subtitle {
        row.set_subtitle(subtitle);
    }

    row.connect_value_notify(clone!(#[strong] settings, #[strong] key, move |row| {
        let value = row.value() as u32;
        if settings.uint(&key) != value {
            let _ = settings.set_uint(&key, value);
        }
    }));

    settings.connect_changed(
        Some(&key),
        clone!(#[weak] row, #[strong] key, move |settings, _| {
            let value = settings.uint(&key) as f64;
            if (row.value() - value).abs() > f64::EPSILON {
                row.set_value(value);
            }
        }),
    );

    row
}

/// A font picker row.
pub fn font(schema: &str, key: &str, title: &str) -> adw::ActionRow {
    let settings = gio::Settings::new(schema);
    let key = key.to_owned();
    let current = settings.string(&key);
    let description = pango::FontDescription::from_string(&current);
    let button = gtk4::FontDialogButton::builder()
        .font_desc(&description)
        .use_size(true)
        .valign(gtk4::Align::Center)
        .build();

    button.connect_font_desc_notify(clone!(#[strong] settings, #[strong] key, move |button| {
        if let Some(description) = button.font_desc() {
            let value = description.to_string();
            if settings.string(&key).as_str() != value.as_str() {
                let _ = settings.set_string(&key, &value);
            }
        }
    }));

    settings.connect_changed(
        Some(&key),
        clone!(#[weak] button, #[strong] key, move |settings, _| {
            let description = pango::FontDescription::from_string(&settings.string(&key));
            if button.font_desc().as_ref() != Some(&description) {
                button.set_font_desc(&description);
            }
        }),
    );

    let row = adw::ActionRow::builder().title(title).build();
    row.add_suffix(&button);
    row
}

/// A file picker row for URI-valued settings (background images, for example).
pub fn file(
    schema: &str,
    key: &str,
    title: &str,
    mime_types: &'static [&'static str],
) -> adw::ActionRow {
    let settings = gio::Settings::new(schema);
    let key = key.to_owned();
    let button = gtk4::Button::builder()
        .icon_name("document-open-symbolic")
        .tooltip_text("Choose a file")
        .valign(gtk4::Align::Center)
        .css_classes(["flat"])
        .build();

    button.connect_clicked(clone!(#[strong] settings, #[strong] mime_types, #[strong] key, move |button| {
        let dialog = gtk4::FileDialog::builder().title("Choose a file").build();
        let filters = gio::ListStore::new::<gtk4::FileFilter>();
        let filter = gtk4::FileFilter::new();
        for mime in mime_types {
            filter.add_mime_type(mime);
        }
        filters.append(&filter);
        dialog.set_filters(Some(&filters));

        let current = settings.string(&key);
        let initial = if current.is_empty() {
            None
        } else {
            gio::File::for_uri(&current).parent()
        };
        if let Some(folder) = initial {
            dialog.set_initial_folder(Some(&folder));
        }

        let parent = button.root().and_downcast::<gtk4::Window>();
        dialog.open(
            parent.as_ref(),
            None::<&gio::Cancellable>,
            clone!(#[strong] settings, #[strong] key, move |result| {
                if let Ok(file) = result {
                    let _ = settings.set_string(&key, &file.uri());
                }
            }),
        );
    }));

    let row = adw::ActionRow::builder().title(title).build();
    row.add_suffix(&button);
    row.set_subtitle(&settings.string(&key));
    settings.connect_changed(
        Some(&key),
        clone!(#[weak] row, #[strong] key, move |settings, _| {
            row.set_subtitle(&settings.string(&key));
        }),
    );
    row
}

/// A row containing a group of mutually exclusive toggle buttons.
///
/// This mirrors the radio groups used by GNOME Tweaks for hinting,
/// antialiasing and window focus.
pub fn choice_buttons(schema: &str, key: &str, title: &str, choices: &[Choice]) -> adw::ActionRow {
    let settings = gio::Settings::new(schema);
    let key = key.to_owned();
    let choices: Vec<Choice> = choices.to_vec();
    let container = gtk4::Box::builder()
        .spacing(0)
        .valign(gtk4::Align::Center)
        .css_classes(["linked"])
        .build();

    // Guards against writing a value back while we are merely reflecting an
    // external GSettings change into the buttons.
    let updating = Rc::new(Cell::new(false));
    let mut first: Option<gtk4::ToggleButton> = None;

    for (value, label) in &choices {
        let value = *value;
        let button = gtk4::ToggleButton::builder().label(*label).build();
        if let Some(group) = &first {
            button.set_group(Some(group));
        } else {
            first = Some(button.clone());
        }
        button.set_active(settings.string(&key).as_str() == value);

        button.connect_toggled(clone!(#[strong] settings, #[strong] updating, #[strong] key, move |button| {
            if updating.get() || !button.is_active() {
                return;
            }
            if settings.string(&key).as_str() != value {
                let _ = settings.set_string(&key, value);
            }
        }));

        settings.connect_changed(
            Some(&key),
            clone!(#[weak] button, #[strong] updating, #[strong] key, move |settings, _| {
                if settings.string(&key).as_str() == value {
                    updating.set(true);
                    if !button.is_active() {
                        button.set_active(true);
                    }
                    updating.set(false);
                }
            }),
        );

        container.append(&button);
    }

    let row = adw::ActionRow::builder().title(title).build();
    row.add_suffix(&container);
    row
}
