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
