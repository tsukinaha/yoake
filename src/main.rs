use gtk::prelude::ApplicationExtManual;

fn main() -> gtk::glib::ExitCode {
    gtk::gio::resources_register_include!("yoake.gresource").expect("Failed to register resources.");
    
    gtk::init().unwrap();

    let provider = gtk::CssProvider::new();

    provider.load_from_string(include_str!("style.css"));
    
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    
    yoake::Application::new().run()
}
