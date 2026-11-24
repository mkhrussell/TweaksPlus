mod actions;
mod window;

use gio::prelude::*;
use libadwaita as adw;

pub const APP_ID: &str = "io.github.kamrul.TweaksPlus";

pub fn run() -> glib::ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();

    actions::install(&app);
    app.connect_activate(|app| window::MainWindow::new(app).present());

    app.run()
}
