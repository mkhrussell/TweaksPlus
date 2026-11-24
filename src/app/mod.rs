use gtk4::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;

pub const APP_ID: &str = "io.github.kamrul.TweaksPlus";

pub fn run() -> glib::ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("TweaksPlus")
            .default_width(980)
            .default_height(680)
            .build();

        window.set_content(Some(&gtk4::Label::new(Some(
            "TweaksPlus — skeleton bootstrap",
        ))));
        window.present();
    });

    app.run()
}
