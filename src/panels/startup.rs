//! Startup Applications panel (XDG autostart entries).

use std::{cell::RefCell, rc::Rc};

use crate::{autostart, panel::Panel};
use gio::prelude::*;
use glib::clone;
use gtk4::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;

type Rows = Rc<RefCell<Vec<adw::ActionRow>>>;

pub fn build() -> Panel {
    let page = adw::PreferencesPage::builder()
        .title("Startup Applications")
        .build();

    let group = adw::PreferencesGroup::builder()
        .title("Startup Applications")
        .description("Applications started automatically when you log in")
        .build();

    let add_button = gtk4::Button::builder()
        .icon_name("list-add-symbolic")
        .tooltip_text("Add a startup application")
        .css_classes(["flat"])
        .valign(gtk4::Align::Center)
        .build();
    group.set_header_suffix(Some(&add_button));

    let rows: Rows = Rc::new(RefCell::new(Vec::new()));
    refresh_group(&group, &rows);

    add_button.connect_clicked(clone!(#[weak] group, #[weak] add_button, #[strong] rows, move |_| {
        let parent = add_button.root().and_downcast::<gtk4::Window>();
        show_app_chooser(parent.as_ref(), &group, &rows);
    }));

    page.add(&group);

    Panel {
        id: "startup",
        title: "Startup Applications",
        description: "Programs launched when you log in",
        icon: Some("list-add-symbolic"),
        keywords: &["startup", "autostart", "login", "application", "program"],
        page,
    }
}

fn refresh_group(group: &adw::PreferencesGroup, rows: &Rows) {
    for row in rows.borrow_mut().drain(..) {
        group.remove(&row);
    }

    let apps = autostart::list();
    if apps.is_empty() {
        let row = adw::ActionRow::builder()
            .title("No startup applications")
            .subtitle("Use the + button to add one")
            .sensitive(false)
            .build();
        group.add(&row);
        rows.borrow_mut().push(row);
        return;
    }

    for app in apps {
        let row = app_row(app, group, rows);
        group.add(&row);
        rows.borrow_mut().push(row);
    }
}

fn app_row(app: autostart::StartupApp, group: &adw::PreferencesGroup, rows: &Rows) -> adw::ActionRow {
    let row = adw::ActionRow::builder()
        .title(app.info.name())
        .subtitle(app.info.description().unwrap_or_default())
        .build();

    if let Some(icon) = app.info.icon() {
        let image = gtk4::Image::builder()
            .gicon(&icon)
            .icon_size(gtk4::IconSize::Large)
            .build();
        row.add_prefix(&image);
    }

    let switch = gtk4::Switch::builder()
        .active(autostart::is_enabled(&app))
        .valign(gtk4::Align::Center)
        .build();
    let path = app.path.clone();
    switch.connect_active_notify(move |switch| {
        let _ = autostart::set_enabled(&path, switch.is_active());
    });
    row.add_suffix(&switch);

    let delete = gtk4::Button::builder()
        .icon_name("edit-delete-symbolic")
        .tooltip_text("Remove")
        .css_classes(["flat"])
        .valign(gtk4::Align::Center)
        .build();
    delete.connect_clicked(clone!(#[weak] group, #[strong] rows, move |_| {
        let _ = autostart::remove(&app);
        refresh_group(&group, &rows);
    }));
    row.add_suffix(&delete);

    row
}

fn show_app_chooser(parent: Option<&gtk4::Window>, group: &adw::PreferencesGroup, rows: &Rows) {
    let installed: Vec<gio::AppInfo> = gio::AppInfo::all()
        .into_iter()
        .filter(|app| app.should_show() && app.id().is_some())
        .collect();
    let startup_ids: Vec<String> = autostart::list()
        .iter()
        .filter_map(|app| app.info.id().map(|id| id.to_string()))
        .collect();

    let list = gtk4::ListBox::builder()
        .selection_mode(gtk4::SelectionMode::Single)
        .build();
    list.add_css_class("navigation-sidebar");

    let candidates: Rc<RefCell<Vec<(String, gio::AppInfo)>>> =
        Rc::new(RefCell::new(Vec::new()));
    for app in installed {
        if startup_ids.iter().any(|id| app.id().as_deref() == Some(id.as_str())) {
            continue;
        }
        let id = app.id().unwrap().to_string();
        let row = adw::ActionRow::builder()
            .title(app.display_name())
            .subtitle(app.description().unwrap_or_default())
            .build();
        if let Some(icon) = app.icon() {
            row.add_prefix(&gtk4::Image::builder().gicon(&icon).build());
        }
        let list_row = gtk4::ListBoxRow::builder().child(&row).build();
        list_row.set_widget_name(&format!("{} {}", app.display_name(), id).to_lowercase());
        list.append(&list_row);
        candidates.borrow_mut().push((id, app));
    }

    let search = gtk4::SearchEntry::builder()
        .placeholder_text("Search Applications…")
        .build();
    list.set_filter_func(clone!(#[weak] search, #[upgrade_or] false, move |row| {
        let query = search.text().trim().to_lowercase();
        query.is_empty() || row.widget_name().contains(&query)
    }));
    search.connect_search_changed(clone!(#[weak] list, move |_| list.invalidate_filter()));

    let scroll = gtk4::ScrolledWindow::builder()
        .child(&list)
        .vexpand(true)
        .min_content_height(320)
        .build();

    let content = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    content.append(&search);
    content.append(&scroll);

    let dialog = adw::Dialog::builder()
        .title("Select Application")
        .child(&content)
        .build();

    let buttons = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .halign(gtk4::Align::End)
        .spacing(6)
        .build();
    let cancel = gtk4::Button::builder().label("Cancel").build();
    let add = gtk4::Button::builder()
        .label("Add")
        .css_classes(["suggested-action"])
        .build();
    cancel.connect_clicked(glib::clone!(#[weak] dialog, move |_| {
        dialog.close();
    }));
    add.connect_clicked(clone!(
        #[weak] dialog,
        #[weak] list,
        #[weak] group,
        #[strong] rows,
        #[strong] candidates,
        move |_| {
            let Some(selected) = list.selected_row() else { return };
            let name = selected.widget_name();
            let app = candidates
                .borrow()
                .iter()
                .find(|(id, _)| name.contains(id.as_str()))
                .map(|(_, app)| app.clone());
            if let Some(app) = app {
                if autostart::add(&app).is_ok() {
                    refresh_group(&group, &rows);
                    dialog.close();
                }
            }
        }
    ));
    buttons.append(&cancel);
    buttons.append(&add);
    content.append(&buttons);

    dialog.present(parent);
}
