# TweaksPlus

An advanced GNOME/Ubuntu customization tool written in Rust with GTK4 and libadwaita.

TweaksPlus starts as a drop-in spiritual successor to GNOME Tweaks: the same
familiar panels and settings, but built on a modern GTK4 codebase that is easy
to extend with powerful tweaks.

## Goals

- **v1.0.0 — GNOME Tweaks parity.** Every tweak available in GNOME Tweaks
  (current GTK4 release plus the classic panels users still rely on).
- **v1.x — More features.** Deeper Ubuntu/GNOME integration, profiles,
  presets, backups, and settings that GNOME Tweaks never exposed.

See [ROADMAP.md](ROADMAP.md) for the detailed feature checklist.

## Technology

- Rust 2024 edition
- `gtk4` and `libadwaita` Rust bindings
- `gio::Settings` for live, bidirectional GSettings integration
- No Python, no GJS — a single native binary

## Building

```sh
cargo build
cargo run
```

## Installing the icon and desktop entry

After building, install the launcher and its icon so TweaksPlus appears in the
GNOME app grid with its proper icon:

```sh
sudo install -Dm644 data/com.mkhrussell.TweaksPlus.desktop \
  /usr/share/applications/com.mkhrussell.TweaksPlus.desktop
sudo install -Dm644 data/icons/hicolor/scalable/apps/com.mkhrussell.TweaksPlus.svg \
  /usr/share/icons/hicolor/scalable/apps/com.mkhrussell.TweaksPlus.svg
sudo install -Dm644 data/icons/hicolor/512x512/apps/com.mkhrussell.TweaksPlus.png \
  /usr/share/icons/hicolor/512x512/apps/com.mkhrussell.TweaksPlus.png
sudo gtk-update-icon-cache -f /usr/share/icons/hicolor
```

For a user-local install, replace `/usr/share` with `~/.local/share` and omit
`sudo`.

## Debian / Ubuntu package

The `debian/` directory contains a native Debian source package. Build a binary
`.deb` with:

```sh
sudo apt install cargo rustc pkg-config libgtk-4-dev libadwaita-1-dev \
  libglib2.0-dev libpango1.0-dev fakeroot
fakeroot debian/rules binary
sudo apt install ../tweaksplus_1.0.0_amd64.deb
```

`dpkg-buildpackage -us -uc -b` also works and produces the same package. Shared
library dependencies are generated automatically with `dpkg-shlibdeps`, so the
package uses the correct library names for the target Ubuntu release.

## Requirements

- Rust 1.92+
- GTK4 >= 4.10, libadwaita >= 1.5, GIO >= 2.70
- A GNOME session (settings still work in other sessions, but some changes
  require GNOME Shell / Mutter to be running)

## Project layout

```
src/
  main.rs          application entry point
  app/             application, main window, actions
  settings/        reusable GSettings-backed rows and theme discovery
  panels/          one module per Tweaks panel
  autostart.rs     XDG autostart management
```

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).
