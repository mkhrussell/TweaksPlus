# TweaksPlus

An advanced GNOME customization tool written in Rust with GTK4 and libadwaita.
The same build supports **Ubuntu 26.04** and **CachyOS GNOME** (Arch-based,
rolling release).

TweaksPlus starts as a drop-in spiritual successor to GNOME Tweaks: the same
familiar panels and settings, but built on a modern GTK4 codebase that is easy
to extend with powerful tweaks.

## Goals

- **v1.0.0 — GNOME Tweaks parity.** Every tweak available in GNOME Tweaks
  (current GTK4 release plus the classic panels users still rely on).
- **v1.1.0 — Cross-distro support.** Ubuntu 26.04 and CachyOS GNOME work from
  one binary, with adaptive GSettings handling and Arch packaging.
- **v1.x — More features.** Deeper desktop integration, profiles, presets,
  backups, and settings that GNOME Tweaks never exposed.

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

On CachyOS/Arch, install the build dependencies first:

```sh
sudo pacman -S --needed base-devel cargo rust pkgconf gtk4 libadwaita glib2 pango
```

On Debian/Ubuntu:

```sh
sudo apt install cargo rustc pkg-config libgtk-4-dev libadwaita-1-dev \
  libglib2.0-dev libpango1.0-dev
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
sudo apt install ../tweaksplus_1.1.0_amd64.deb
```

`dpkg-buildpackage -us -uc -b` also works and produces the same package. Shared
library dependencies are generated automatically with `dpkg-shlibdeps`, so the
package uses the correct library names for the target Ubuntu release.

## Arch / CachyOS package (pacman / AUR)

Two PKGBUILDs are provided:

- `packaging/arch/PKGBUILD` — stable release package, pointing at the GitHub
  source tarball for the tagged version. Replace `sha256sums=('SKIP')` with the
  release checksum before publishing to the AUR.
- `packaging/arch-git/PKGBUILD` — VCS package (`tweaksplus-git`) that tracks the
  latest source and derives `pkgver` from `Cargo.toml`.

Build and install either with:

```sh
cd packaging/arch       # or packaging/arch-git
makepkg -si
```

To package the **current working tree** (including unreleased fixes) instead of
a tagged release:

```sh
scripts/build-arch-package.sh -si
```

This creates a source archive from the checkout and runs `makepkg` for you.

Optional GNOME Shell extensions:

```sh
sudo pacman -S gnome-shell-extensions                  # User Themes (Shell theme)
sudo pacman -S gnome-shell-extension-desktop-icons-ng  # extra desktop icon options
```

TweaksPlus detects whether those schemas are installed and shows an installation
hint when an optional extension is missing, instead of failing to start.

## User-local install (no build)

The installer downloads a portable `.tar.xz` (or a `.deb` as a fallback) from a
GitHub release and extracts it into `~/.local`, so no Cargo build or system-wide
install is needed. It works on Ubuntu and on Arch/CachyOS without `dpkg`:

```sh
curl -fsSL https://github.com/mkhrussell/TweaksPlus/releases/download/v1.1.0/install-local.sh | bash
```

It installs the binary to `~/.local/bin`, the desktop launcher to
`~/.local/share/applications`, and the icons to `~/.local/share/icons/hicolor`.
Set `RELEASE_TAG` to install a specific release:

```sh
RELEASE_TAG=v1.1.0 bash scripts/install-local.sh
```

## Updating the GitHub release

Maintainers can rebuild the packages and refresh the release in one step:

```sh
scripts/update-release.sh
```

The script builds a portable `.tar.xz` and, when dpkg tooling is available, a
`.deb`. It replaces those assets, uploads `install-local.sh`, and updates the
release notes with install instructions for both distributions. It uses
`GITHUB_TOKEN` or the GitHub token in git's credential store.

## Cross-distribution compatibility

GNOME settings are not identical across releases. TweaksPlus therefore:

- checks both the **schema and the key** before touching a setting, so a key
  removed in a rolling GNOME release never aborts the application;
- falls back from the `lid-close-*-action` keys (Ubuntu 26.04 and older GNOME)
  to the equivalent `sleep-inactive-*-type` keys on CachyOS GNOME 51+, where
  lid-close keys were removed;
- discovers themes, icons, cursors and sounds from standard XDG directories,
  with no Ubuntu-specific paths;
- gates optional extension settings (User Themes, Desktop Icons NG) on schema
  availability and shows distribution-appropriate installation hints;
- keeps every tweak in place when its schema exists, so Ubuntu 26.04
  functionality is unchanged.

## Requirements

- Rust 1.92+
- GTK4 >= 4.10, libadwaita >= 1.5, GIO >= 2.70
- Ubuntu 26.04 GNOME or CachyOS GNOME (settings still work in other sessions,
  but some changes require GNOME Shell / Mutter to be running)

## Project layout

```
src/
  main.rs          application entry point
  app/             application, main window, actions
  settings/        GSettings rows, discovery and environment helpers
  panels/          one module per Tweaks panel
  autostart.rs     XDG autostart management
debian/            native Debian/Ubuntu package
packaging/arch/    PKGBUILD for Arch/CachyOS releases (+ .SRCINFO)
packaging/arch-git/ PKGBUILD for the latest Git source (+ .SRCINFO)
scripts/           build, install and release helpers
```

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).
