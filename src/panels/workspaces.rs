//! Workspaces panel (classic GNOME Tweaks settings).

use crate::{panel::Panel, settings::rows};
use gio::prelude::*;
use glib::clone;
use libadwaita as adw;
use libadwaita::prelude::*;

pub fn build() -> Panel {
    let page = adw::PreferencesPage::builder().title("Workspaces").build();

    let group = adw::PreferencesGroup::builder()
        .title("Workspaces")
        .description("How the Shell creates and displays workspaces")
        .build();

    let count = rows::spin_int(
        "org.gnome.desktop.wm.preferences",
        "num-workspaces",
        "Number of Workspaces",
        Some("Only used when dynamic workspaces are disabled"),
        1,
        36,
        1,
    );

    let dynamic = rows::switch(
        "org.gnome.mutter",
        "dynamic-workspaces",
        "Dynamic Workspaces",
        Some("Automatically create and remove workspaces as needed"),
    );
    let mutter = gio::Settings::new("org.gnome.mutter");
    let sync = clone!(#[weak] count, move |settings: &gio::Settings| {
        count.set_sensitive(!settings.boolean("dynamic-workspaces"));
    });
    sync(&mutter);
    mutter.connect_changed(Some("dynamic-workspaces"), move |settings, _| {
        sync(settings);
    });

    group.add(&dynamic);
    group.add(&count);
    group.add(&rows::switch(
        "org.gnome.mutter",
        "workspaces-only-on-primary",
        "Workspaces on Primary Monitor Only",
        None,
    ));

    page.add(&group);

    Panel {
        id: "workspaces",
        title: "Workspaces",
        description: "Dynamic and static workspace settings",
        icon: Some("view-grid-symbolic"),
        keywords: &["workspace", "dynamic", "static", "monitor", "overview"],
        page,
    }
}
