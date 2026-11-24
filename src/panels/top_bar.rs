//! Top Bar panel (classic GNOME Tweaks settings).

use crate::{panel::Panel, settings::rows};
use libadwaita as adw;
use libadwaita::prelude::*;

pub fn build() -> Panel {
    let page = adw::PreferencesPage::builder().title("Top Bar").build();

    let clock = adw::PreferencesGroup::builder().title("Clock").build();
    clock.add(&rows::combo(
        "org.gnome.desktop.interface",
        "clock-format",
        "Clock Format",
        None,
        &[("24h", "24-hour"), ("12h", "12-hour")],
    ));
    clock.add(&rows::switch(
        "org.gnome.desktop.interface",
        "clock-show-seconds",
        "Show Seconds",
        None,
    ));
    clock.add(&rows::switch(
        "org.gnome.desktop.interface",
        "clock-show-date",
        "Show Date",
        None,
    ));
    clock.add(&rows::switch(
        "org.gnome.desktop.interface",
        "clock-show-weekday",
        "Show Weekday",
        None,
    ));
    clock.add(&rows::switch(
        "org.gnome.desktop.calendar",
        "show-weekdate",
        "Show Week Numbers",
        Some("In the calendar dropdown"),
    ));

    let status = adw::PreferencesGroup::builder().title("Status Area").build();
    status.add(&rows::switch(
        "org.gnome.desktop.interface",
        "show-battery-percentage",
        "Show Battery Percentage",
        None,
    ));
    status.add(&rows::switch(
        "org.gnome.desktop.privacy",
        "show-full-name-in-top-bar",
        "Show Full Name",
        Some("Show your full name in the top bar instead of your avatar"),
    ));

    page.add(&clock);
    page.add(&status);

    Panel {
        id: "top-bar",
        title: "Top Bar",
        description: "Clock, calendar and status area",
        icon: Some("appointment-new-symbolic"),
        keywords: &[
            "top bar", "clock", "date", "weekday", "seconds", "battery", "calendar", "status",
        ],
        page,
    }
}
