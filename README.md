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

## Requirements

- Rust 1.92+
- GTK4 >= 4.10, libadwaita >= 1.4, GIO >= 2.70
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
