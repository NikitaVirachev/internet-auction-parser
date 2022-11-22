use gdk::Display;
use gtk::prelude::*;
use gtk::{gdk, gio, Application, CssProvider, StyleContext};
use window::Window;
const APP_ID: &str = "org.gtk_rs.HelloWorld3";

mod avito_parser;
pub use avito_parser::Storage;

mod book;
pub use book::Book;

mod lot;
pub use lot::Lot;

mod db;
pub use db::DB;

mod window;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Register and include resources
    gio::resources_register_include!("composite_templates_1.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // Run the application
    app.run();

    Ok(())
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}
