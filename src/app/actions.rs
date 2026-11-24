//! Application-wide menu actions.

use crate::app::APP_ID;
use gio::prelude::*;
use gtk4::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;

pub fn install(app: &adw::Application) {
    app.add_action_entries([
        gio::ActionEntry::builder("reset-all")
            .activate(|app: &adw::Application, _, _| {
                crate::settings::reset_all();
                if let Some(window) = app.active_window() {
                    let dialog = adw::MessageDialog::builder()
                        .transient_for(&window)
                        .heading("Settings reset")
                        .body("All TweaksPlus settings have been reset to their defaults.")
                        .build();
                    dialog.add_response("ok", "OK");
                    dialog.present();
                }
            })
            .build(),
        gio::ActionEntry::builder("about")
            .activate(|app: &adw::Application, _, _| {
                let dialog = gtk4::AboutDialog::builder()
                    .logo_icon_name(APP_ID)
                    .program_name("TweaksPlus")
                    .version(env!("CARGO_PKG_VERSION"))
                    .comments("Advanced GNOME and Ubuntu customization")
                    .website("https://github.com/kamrul/TweaksPlus")
                    .license_type(gtk4::License::Gpl30)
                    .authors(["Kamrul"])
                    .build();
                if let Some(window) = app.active_window() {
                    dialog.set_transient_for(Some(&window));
                    dialog.set_modal(true);
                }
                dialog.present();
            })
            .build(),
        gio::ActionEntry::builder("quit")
            .activate(|app: &adw::Application, _, _| app.quit())
            .build(),
    ]);

    let menu = gio::Menu::new();
    menu.append(Some("Reset All Tweaks"), Some("app.reset-all"));
    menu.append(Some("About TweaksPlus"), Some("app.about"));
    menu.append(Some("Quit"), Some("app.quit"));
    // GtkApplication requires the application to be registered before a
    // menubar can be attached.
    app.connect_startup(move |app| app.set_menubar(Some(&menu)));
}
