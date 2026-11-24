//! XDG autostart management (`~/.config/autostart/*.desktop`).

use std::{fs, io, path::PathBuf};

use gio::prelude::*;

pub struct StartupApp {
    pub info: gio_unix::DesktopAppInfo,
    pub path: PathBuf,
}

pub fn autostart_dir() -> PathBuf {
    glib::user_config_dir().join("autostart")
}

/// All user autostart entries.
pub fn list() -> Vec<StartupApp> {
    let Ok(entries) = fs::read_dir(autostart_dir()) else {
        return Vec::new();
    };

    let mut apps: Vec<StartupApp> = entries
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("desktop") {
                return None;
            }
            let info = gio_unix::DesktopAppInfo::from_filename(&path)?;
            Some(StartupApp { info, path })
        })
        .collect();

    apps.sort_by_key(|app| app.info.name().to_lowercase());
    apps
}

/// Copies an installed application's desktop file into the autostart directory.
pub fn add(app: &gio::AppInfo) -> io::Result<()> {
    let id = app
        .id()
        .ok_or_else(|| io::Error::other("application has no desktop id"))?;
    let source = gio_unix::DesktopAppInfo::new(&id)
        .and_then(|info| info.filename())
        .ok_or_else(|| io::Error::other("application has no desktop file"))?;

    fs::create_dir_all(autostart_dir())?;
    let filename = if id.ends_with(".desktop") {
        id.to_string()
    } else {
        format!("{id}.desktop")
    };
    if source != autostart_dir().join(&filename) {
        fs::copy(source, autostart_dir().join(&filename))?;
    }
    set_enabled(&autostart_dir().join(&filename), true)
}

pub fn remove(app: &StartupApp) -> io::Result<()> {
    fs::remove_file(&app.path)
}

fn key_value(path: &PathBuf, key: &str) -> Option<String> {
    let contents = fs::read_to_string(path).ok()?;
    contents.lines().find_map(|line| {
        let (name, value) = line.split_once('=').unwrap_or((line, ""));
        (name.trim() == key).then(|| value.trim().to_owned())
    })
}

pub fn is_enabled(app: &StartupApp) -> bool {
    let hidden = key_value(&app.path, "Hidden")
        .is_some_and(|value| value.eq_ignore_ascii_case("true"));
    let disabled = key_value(&app.path, "X-GNOME-Autostart-enabled")
        .is_some_and(|value| value.eq_ignore_ascii_case("false"));
    !hidden && !disabled
}

pub fn set_enabled(path: &PathBuf, enabled: bool) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;
    let mut lines: Vec<String> = contents.lines().map(str::to_owned).collect();

    for (key, value) in [
        ("Hidden", if enabled { "false" } else { "true" }),
        (
            "X-GNOME-Autostart-enabled",
            if enabled { "true" } else { "false" },
        ),
    ] {
        if let Some(line) = lines.iter_mut().find(|line| {
            line.split_once('=')
                .map(|(name, _)| name.trim() == key)
                .unwrap_or(false)
        }) {
            *line = format!("{key}={value}");
        } else if let Some(index) = lines
            .iter()
            .position(|line| line.trim() == "[Desktop Entry]")
        {
            lines.insert(index + 1, format!("{key}={value}"));
        } else {
            lines.push(format!("{key}={value}"));
        }
    }

    fs::write(path, format!("{}\n", lines.join("\n")))
}
