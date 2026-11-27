//! Small environment helpers for distribution-specific guidance.
//!
//! TweaksPlus uses the same GSettings keys everywhere they exist. The only
//! distribution-specific text is help for optional GNOME Shell extensions,
//! which are installed from different packages on Ubuntu and on Arch-based
//! systems such as CachyOS.

use std::fs;

fn os_release_field(field: &str) -> Option<String> {
    let contents = fs::read_to_string("/etc/os-release").ok()?;
    contents.lines().find_map(|line| {
        let (name, value) = line.split_once('=')?;
        (name == field).then(|| value.trim().trim_matches('"').to_owned())
    })
}

/// True for Arch, CachyOS, Manjaro and other Arch-derived distributions.
pub fn is_arch_based() -> bool {
    let id = os_release_field("ID").unwrap_or_default();
    let id_like = os_release_field("ID_LIKE").unwrap_or_default();
    id == "arch" || id == "cachyos" || id == "manjaro" || id_like.split_whitespace().any(|part| part == "arch")
}

/// Installation hint for an optional extension that is packaged on Arch.
///
/// `arch_package` is the package name in the Arch/CachyOS repositories;
/// `other_hint` is shown on Debian/Ubuntu and everywhere else.
pub fn extension_hint(arch_package: Option<&str>, other_hint: &str) -> String {
    if is_arch_based() {
        match arch_package {
            Some(package) => format!("Install and enable the extension: sudo pacman -S {package}"),
            None => "Install and enable the extension from GNOME Extensions".to_owned(),
        }
    } else {
        other_hint.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hints_are_not_empty() {
        assert!(!extension_hint(Some("some-package"), "Install the extension").is_empty());
    }
}
