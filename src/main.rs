mod app;
mod autostart;
mod panel;
mod panels;
mod settings;

fn main() -> glib::ExitCode {
    app::run()
}
