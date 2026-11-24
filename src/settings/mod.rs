//! Reusable, bidirectional GSettings-backed preference rows.
//!
//! Every row in TweaksPlus reads its initial value from GSettings, writes
//! changes back to GSettings, and listens for external changes so the UI stays
//! in sync when a setting is changed by another application.

pub mod discover;
pub mod rows;

use gio::prelude::*;

/// Returns whether `schema` is installed on this system.
///
/// Panels use this to hide tweaks that cannot work without an optional schema
/// (for example, the User Themes shell extension schema).
pub fn schema_available(schema: &str) -> bool {
    gio::SettingsSchemaSource::default()
        .and_then(|source| source.lookup(schema, true))
        .is_some()
}

/// Resets a single key to its default value.
pub fn reset(schema: &str, key: &str) {
    let settings = gio::Settings::new(schema);
    settings.reset(key);
}

/// Every key that TweaksPlus can change.
///
/// Only these keys are reset by "Reset All Tweaks" so unrelated settings from
/// the same schemas are left untouched.
pub const TWEAK_KEYS: &[(&str, &str)] = &[
    ("org.gnome.desktop.interface", "enable-animations"),
    ("org.gnome.desktop.interface", "cursor-theme"),
    ("org.gnome.desktop.interface", "icon-theme"),
    ("org.gnome.desktop.interface", "gtk-theme"),
    ("org.gnome.desktop.interface", "gtk-key-theme"),
    ("org.gnome.desktop.interface", "font-name"),
    ("org.gnome.desktop.interface", "document-font-name"),
    ("org.gnome.desktop.interface", "monospace-font-name"),
    ("org.gnome.desktop.interface", "font-hinting"),
    ("org.gnome.desktop.interface", "font-antialiasing"),
    ("org.gnome.desktop.interface", "text-scaling-factor"),
    ("org.gnome.desktop.interface", "cursor-blink"),
    ("org.gnome.desktop.interface", "locate-pointer"),
    ("org.gnome.desktop.interface", "enable-hot-corners"),
    ("org.gnome.desktop.interface", "clock-format"),
    ("org.gnome.desktop.privacy", "show-full-name-in-top-bar"),
    ("org.gnome.desktop.interface", "clock-show-seconds"),
    ("org.gnome.desktop.interface", "clock-show-date"),
    ("org.gnome.desktop.interface", "clock-show-weekday"),
    ("org.gnome.desktop.interface", "show-battery-percentage"),
    ("org.gnome.desktop.interface", "gtk-enable-primary-paste"),
    ("org.gnome.desktop.background", "picture-uri"),
    ("org.gnome.desktop.background", "picture-uri-dark"),
    ("org.gnome.desktop.background", "picture-options"),
    ("org.gnome.desktop.background", "show-desktop-icons"),
    ("org.gnome.shell.extensions.ding", "show-home"),
    ("org.gnome.shell.extensions.ding", "show-trash"),
    ("org.gnome.shell.extensions.ding", "show-network-volumes"),
    ("org.gnome.shell.extensions.ding", "show-volumes"),
    ("org.gnome.shell.extensions.ding", "show-drop-place"),
    ("org.gnome.shell.extensions.ding", "icon-size"),
    ("org.gnome.desktop.wm.preferences", "button-layout"),
    ("org.gnome.desktop.wm.preferences", "action-double-click-titlebar"),
    ("org.gnome.desktop.wm.preferences", "action-middle-click-titlebar"),
    ("org.gnome.desktop.wm.preferences", "action-right-click-titlebar"),
    ("org.gnome.desktop.wm.preferences", "focus-mode"),
    ("org.gnome.desktop.wm.preferences", "auto-raise"),
    ("org.gnome.desktop.wm.preferences", "mouse-button-modifier"),
    ("org.gnome.desktop.wm.preferences", "resize-with-right-button"),
    ("org.gnome.desktop.wm.preferences", "num-workspaces"),
    ("org.gnome.mutter", "attach-modal-dialogs"),
    ("org.gnome.mutter", "center-new-windows"),
    ("org.gnome.mutter", "edge-tiling"),
    ("org.gnome.mutter", "dynamic-workspaces"),
    ("org.gnome.mutter", "workspaces-only-on-primary"),
    ("org.gnome.mutter", "overlay-key"),
    ("org.gnome.desktop.sound", "theme-name"),
    ("org.gnome.desktop.sound", "event-sounds"),
    ("org.gnome.desktop.sound", "input-feedback-sounds"),
    ("org.gnome.desktop.sound", "allow-volume-above-100-percent"),
    ("org.gnome.desktop.peripherals.mouse", "middle-click-emulation"),
    ("org.gnome.desktop.peripherals.mouse", "accel-profile"),
    ("org.gnome.desktop.peripherals.mouse", "natural-scroll"),
    ("org.gnome.desktop.peripherals.touchpad", "disable-while-typing"),
    ("org.gnome.desktop.peripherals.touchpad", "tap-to-click"),
    ("org.gnome.desktop.peripherals.touchpad", "two-finger-scrolling-enabled"),
    ("org.gnome.desktop.peripherals.touchpad", "natural-scroll"),
    ("org.gnome.desktop.input-sources", "xkb-options"),
    ("org.gnome.desktop.peripherals.touchpad", "accel-profile"),
    ("org.gnome.desktop.peripherals.touchpad", "click-method"),
    ("org.gnome.desktop.peripherals.pointingstick", "scroll-method"),
    ("org.gnome.desktop.input-sources", "show-all-sources"),
    ("org.gnome.desktop.calendar", "show-weekdate"),
    ("org.gnome.desktop.datetime", "automatic-timezone"),
    ("org.gnome.settings-daemon.plugins.power", "lid-close-ac-action"),
    ("org.gnome.settings-daemon.plugins.power", "lid-close-battery-action"),
    ("org.gnome.settings-daemon.plugins.power", "power-button-action"),
    ("org.gnome.settings-daemon.plugins.power", "idle-dim"),
    ("org.gnome.settings-daemon.plugins.power", "idle-brightness"),
    (
        "org.gnome.settings-daemon.plugins.power",
        "power-saver-profile-on-low-battery",
    ),
    ("org.gnome.desktop.session", "idle-delay"),
];

/// Resets every tweak managed by TweaksPlus to its default value.
pub fn reset_all() {
    for (schema, key) in TWEAK_KEYS {
        if schema_available(schema) {
            reset(schema, key);
        }
    }
}
