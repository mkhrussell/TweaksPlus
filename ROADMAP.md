# TweaksPlus Roadmap

## v1.0.0 — GNOME Tweaks feature parity ✅

Every panel and tweak available in current GNOME Tweaks (GTK4 release) plus the
classic panels that users still rely on. All settings are live, bidirectional
GSettings bindings: changes made in TweaksPlus are reflected immediately by the
Shell and changes made elsewhere update the UI.

### Panels

- [x] **General** — animations, over-amplification, pointer location, hot corners
- [x] **Appearance** — legacy GTK theme, Shell theme (User Themes), icon theme,
      cursor theme, light/dark wallpapers, wallpaper adjustment
- [x] **Fonts** — interface/document/monospace fonts, hinting, antialiasing, text scaling
- [x] **Top Bar** — clock format/seconds/date/weekday, week numbers, battery
      percentage, full name in top bar
- [x] **Desktop** — desktop icons, Desktop Icons NG extension options
- [x] **Workspaces** — dynamic/static workspaces, workspace count, primary-monitor mode
- [x] **Windows** — titlebar click actions, minimize/maximize buttons, button
      placement, modal dialog attachment, window centering, action modifier key,
      right-button resize, focus mode, auto-raise
- [x] **Keyboard** — extended input sources, Emacs bindings, overview shortcut,
      common XKB layout options
- [x] **Mouse & Touchpad** — middle-click paste, pointer acceleration, natural
      scrolling, touchpad click/scroll/disable-while-typing, pointing stick
- [x] **Sound** — system sound theme, event sounds, input feedback sounds
- [x] **Power** — lid-close actions (AC/battery), power button action, screen
      blank delay, idle dimming, low-battery power saver
- [x] **Startup Applications** — list, enable/disable and add/remove XDG autostart entries

### Shell / application

- [x] Adwaita navigation split view with searchable sidebar
- [x] `Ctrl+F` search across panel titles, descriptions and keywords
- [x] Application menu: About, Reset All Tweaks, Quit
- [x] Reset All only touches keys TweaksPlus manages
- [x] Themes are discovered from the same XDG paths GNOME Tweaks scans

## v1.x — Beyond GNOME Tweaks (planned)

- Settings profiles / presets with one-click switching
- Import and export of tweak configurations
- Ubuntu-specific integrations ( dock, PPAs, live wallpapers)
- A registry so third-party plugins can contribute panels
- Flatpak packaging and CI releases
