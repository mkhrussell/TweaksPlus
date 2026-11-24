//! Discovery of themes installed on the system.
//!
//! GNOME Tweaks only offers themes that actually exist, so TweaksPlus scans
//! the same XDG locations before populating its drop-downs.

use std::{
    collections::HashSet,
    env,
    fs,
    path::{Path, PathBuf},
};

fn system_data_dirs() -> Vec<PathBuf> {
    match env::var("XDG_DATA_DIRS") {
        Ok(value) if !value.trim().is_empty() => value
            .split(':')
            .filter(|part| !part.is_empty())
            .map(PathBuf::from)
            .collect(),
        _ => vec![PathBuf::from("/usr/local/share"), PathBuf::from("/usr/share")],
    }
}

/// Theme roots: `~/.local/share/<subdir>`, a legacy `~/<legacy>` location and
/// every XDG data directory.
fn resource_dirs(subdir: &str, legacy: &str) -> Vec<PathBuf> {
    let mut dirs = vec![
        glib::user_data_dir().join(subdir),
        glib::home_dir().join(legacy),
    ];
    dirs.extend(system_data_dirs().into_iter().map(|dir| dir.join(subdir)));
    dirs
}

fn collect(roots: &[PathBuf], is_valid: impl Fn(&Path) -> bool) -> Vec<(String, String)> {
    let mut seen = HashSet::new();
    let mut themes = Vec::new();

    for root in roots {
        let Ok(entries) = fs::read_dir(root) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() || !is_valid(&path) {
                continue;
            }
            let name = entry.file_name().to_string_lossy().into_owned();
            if seen.insert(name.clone()) {
                themes.push((name.clone(), name));
            }
        }
    }

    themes.sort_by(|a, b| a.1.to_lowercase().cmp(&b.1.to_lowercase()));
    themes
}

/// GTK themes that contain a GTK 3 or GTK 4 stylesheet.
pub fn gtk_themes() -> Vec<(String, String)> {
    let mut themes = vec![
        ("Adwaita".to_string(), "Adwaita".to_string()),
        ("Adwaita-dark".to_string(), "Adwaita-dark".to_string()),
        ("HighContrast".to_string(), "HighContrast".to_string()),
        (
            "HighContrastInverse".to_string(),
            "HighContrastInverse".to_string(),
        ),
    ];
    let mut known: HashSet<String> = themes.iter().map(|(value, _)| value.clone()).collect();

    for (value, label) in collect(&resource_dirs("themes", ".themes"), |path| {
        path.join("gtk-3.0/gtk.css").is_file()
            || path.join("gtk-4.0/gtk.css").is_file()
            || fs::read_dir(path.join("gtk-3.0"))
                .map(|entries| {
                    entries.flatten().any(|entry| {
                        entry
                            .file_name()
                            .to_string_lossy()
                            .starts_with("gtk-3.")
                    })
                })
                .unwrap_or(false)
    }) {
        if known.insert(value.clone()) {
            themes.push((value, label));
        }
    }

    themes.sort_by(|a, b| a.1.to_lowercase().cmp(&b.1.to_lowercase()));
    themes
}

/// Icon themes (directories containing an `index.theme` file).
pub fn icon_themes() -> Vec<(String, String)> {
    collect(&resource_dirs("icons", ".icons"), |path| {
        path.join("index.theme").is_file()
    })
}

/// Cursor themes (directories containing a `cursors` directory).
pub fn cursor_themes() -> Vec<(String, String)> {
    collect(&resource_dirs("icons", ".icons"), |path| {
        path.join("cursors").is_dir()
    })
}

/// GNOME Shell themes, including the default (empty) theme.
pub fn shell_themes() -> Vec<(String, String)> {
    let mut themes = vec![(String::new(), "Adwaita (default)".to_string())];
    let mut roots = resource_dirs("themes", ".themes");
    roots.extend(system_data_dirs().into_iter().map(|dir| dir.join("gnome-shell")));

    let mut known = HashSet::new();
    for (value, label) in collect(&roots, |path| {
        path.join("gnome-shell.css").is_file()
            || path.join("gnome-shell/gnome-shell.css").is_file()
            || path.join("gnome-shell/theme.json").is_file()
    }) {
        if known.insert(value.clone()) {
            themes.push((value, label));
        }
    }
    themes
}

fn sound_theme_label(index: &Path) -> Option<String> {
    let contents = fs::read_to_string(index).ok()?;
    let mut in_section = false;
    for line in contents.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_section = line.eq_ignore_ascii_case("[Sound Theme]");
        } else if in_section && line.len() > 5 && line[..5].eq_ignore_ascii_case("name=") {
            return Some(line[5..].trim().to_string());
        }
    }
    None
}

/// Sound themes, keyed by directory name and labelled from `index.theme`.
pub fn sound_themes() -> Vec<(String, String)> {
    let mut seen = HashSet::new();
    let mut themes = Vec::new();
    for root in resource_dirs("sounds", ".local/share/sounds") {
        let Ok(entries) = fs::read_dir(root) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let Some(label) = sound_theme_label(&path.join("index.theme")) else {
                continue;
            };
            if seen.insert(label.clone()) {
                themes.push((entry.file_name().to_string_lossy().into_owned(), label));
            }
        }
    }
    themes.sort_by(|a, b| a.1.to_lowercase().cmp(&b.1.to_lowercase()));
    themes
}
