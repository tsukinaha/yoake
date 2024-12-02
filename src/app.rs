use adw::{
    prelude::*,
    subclass::prelude::*,
};
use gtk::glib;

mod imp {

    use super::*;

    #[derive(Debug, Default)]
    pub struct YoakeApplication;

    #[glib::object_subclass]
    impl ObjectSubclass for YoakeApplication {
        const NAME: &'static str = "YoakeApplication";
        type Type = super::YoakeApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for YoakeApplication {}

    impl ApplicationImpl for YoakeApplication {
        fn activate(&self) {
            self.parent_activate();

            crate::Window::new(&self.obj(), &crate::Layer::new(&self.obj())).present();
        }
    }

    impl GtkApplicationImpl for YoakeApplication {}

    impl AdwApplicationImpl for YoakeApplication {}
}

glib::wrapper! {
    pub struct YoakeApplication(ObjectSubclass<imp::YoakeApplication>)
        @extends gtk::gio::Application, gtk::Application, adw::Application, @implements gtk::Accessible;
}

impl Default for YoakeApplication {
    fn default() -> Self {
        Self::new()
    }
}

impl YoakeApplication {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
