//! Mouse and Touchpad panel.

use crate::{panel::Panel, settings::rows};
use libadwaita as adw;
use libadwaita::prelude::*;

const TOUCHPAD: &str = "org.gnome.desktop.peripherals.touchpad";
const POINTING_STICK: &str = "org.gnome.desktop.peripherals.pointingstick";

pub fn build() -> Panel {
    let page = adw::PreferencesPage::builder()
        .title("Mouse and Touchpad")
        .build();

    let mouse = adw::PreferencesGroup::builder().title("Mouse").build();
    mouse.add(&rows::switch(
        "org.gnome.desktop.interface",
        "gtk-enable-primary-paste",
        "Middle Click Paste",
        Some("Paste the clipboard with a middle click"),
    ));
    mouse.add(&rows::enum_switch(
        "org.gnome.desktop.peripherals.mouse",
        "accel-profile",
        "Mouse Acceleration",
        Some("Turn off for consistent, precise movement"),
        "flat",
    ));
    mouse.add(&rows::switch(
        "org.gnome.desktop.peripherals.mouse",
        "natural-scroll",
        "Natural Scrolling",
        None,
    ));

    let touchpad = adw::PreferencesGroup::builder().title("Touchpad").build();
    touchpad.add(&rows::enum_switch(
        TOUCHPAD,
        "accel-profile",
        "Touchpad Acceleration",
        Some("Turn off for faster and more precise movements"),
        "flat",
    ));
    touchpad.add(&rows::value_switch(
        TOUCHPAD,
        "click-method",
        "Disable Secondary Click",
        Some("For touchpads without a physical secondary button"),
        "none",
        "default",
    ));
    touchpad.add(&rows::switch(
        TOUCHPAD,
        "tap-to-click",
        "Tap to Click",
        None,
    ));
    touchpad.add(&rows::switch(
        TOUCHPAD,
        "two-finger-scrolling-enabled",
        "Two-Finger Scrolling",
        None,
    ));
    touchpad.add(&rows::switch(
        TOUCHPAD,
        "natural-scroll",
        "Natural Scrolling",
        None,
    ));

    let stick = adw::PreferencesGroup::builder()
        .title("Pointing Stick")
        .description("Only affects laptops with a pointing stick")
        .build();
    stick.add(&rows::enum_switch(
        POINTING_STICK,
        "accel-profile",
        "Pointing Stick Acceleration",
        None,
        "flat",
    ));
    stick.add(&rows::combo(
        POINTING_STICK,
        "scroll-method",
        "Scrolling Method",
        None,
        &[
            ("default", "Default"),
            ("none", "Disabled"),
            ("on-button-down", "On Button Down"),
        ],
    ));

    page.add(&mouse);
    page.add(&touchpad);
    page.add(&stick);

    Panel {
        id: "mouse",
        title: "Mouse and Touchpad",
        description: "Pointer, scrolling and touchpad behaviour",
        icon: Some("input-mouse-symbolic"),
        keywords: &[
            "mouse", "touchpad", "pointer", "click", "scroll", "acceleration", "pointing stick",
        ],
        page,
    }
}
