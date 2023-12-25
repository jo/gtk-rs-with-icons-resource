mod window;

use gtk::prelude::*;
use gtk::{gio, glib, gdk, Application};
use window::Window;

const APP_ID: &str = "com.github.jo.gtk-rs-with-icons-resource";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("gtk-rs-with-icons-resource.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.connect_startup(register_icons_resource);
    app.run()
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}

fn register_icons_resource(_app: &Application) {
    let display = gdk::Display::default().unwrap();
    let theme = gtk::IconTheme::for_display(&display);
    theme.add_resource_path("/com/github/jo/gtk-rs-with-icons-resource/icons/");  
}
