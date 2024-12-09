mod app;
mod layer;
mod input;
mod window;
mod runtime;

use gtk::prelude::ApplicationExtManual;

pub use app::YoakeApplication as Application;
pub use layer::YoakeLayer as Layer;
pub use window::YoakeWindow as Window;
pub use runtime::runtime as runtime;
pub use input::Interface as InputInterface;

pub fn run() -> gtk::glib::ExitCode {
    gtk::gio::resources_register_include!("yoake.gresource").expect("Failed to register resources.");
    
    gtk::init().expect("Failed to initialize GTK.");

    let provider = gtk::CssProvider::new();

    provider.load_from_string(include_str!("style.css"));
    
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    
    Application::new().run()
}
