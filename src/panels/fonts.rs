//! Fonts panel: preferred fonts, rendering and size.

use crate::{panel::Panel, settings::rows};
use libadwaita as adw;
use libadwaita::prelude::*;

pub fn build() -> Panel {
    let page = adw::PreferencesPage::builder().title("Fonts").build();

    let preferred = adw::PreferencesGroup::builder().title("Preferred Fonts").build();
    preferred.add(&rows::font(
        "org.gnome.desktop.interface",
        "font-name",
        "Interface Text",
    ));
    preferred.add(&rows::font(
        "org.gnome.desktop.interface",
        "document-font-name",
        "Document Text",
    ));
    preferred.add(&rows::font(
        "org.gnome.desktop.interface",
        "monospace-font-name",
        "Monospace Text",
    ));

    let rendering = adw::PreferencesGroup::builder().title("Rendering").build();
    rendering.add(&rows::choice_buttons(
        "org.gnome.desktop.interface",
        "font-hinting",
        "Hinting",
        &[
            ("full", "Full"),
            ("medium", "Medium"),
            ("slight", "Slight"),
            ("none", "None"),
        ],
    ));
    rendering.add(&rows::choice_buttons(
        "org.gnome.desktop.interface",
        "font-antialiasing",
        "Antialiasing",
        &[
            ("rgba", "Subpixel"),
            ("grayscale", "Grayscale"),
            ("none", "None"),
        ],
    ));

    let size = adw::PreferencesGroup::builder().title("Size").build();
    size.add(&rows::spin(
        "org.gnome.desktop.interface",
        "text-scaling-factor",
        "Scaling Factor",
        None,
        0.5,
        3.0,
        0.01,
        2,
    ));

    page.add(&preferred);
    page.add(&rendering);
    page.add(&size);

    Panel {
        id: "fonts",
        title: "Fonts",
        description: "Font families, hinting and text size",
        icon: Some("font-x-generic-symbolic"),
        keywords: &["font", "text", "hinting", "antialiasing", "scaling", "size"],
        page,
    }
}
