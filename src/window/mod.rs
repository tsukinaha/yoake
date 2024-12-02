use adw::subclass::prelude::*;
use adw::prelude::*;
use gtk::{
    glib,
    prelude::GtkWindowExt,
    template_callbacks, Adjustment,
};
use gtk4_layer_shell::{Edge, LayerShell};

mod imp {

    use std::sync::{atomic::AtomicBool, Arc};

    use glib::{
        subclass::InitializingObject,
        WeakRef,
    };
    use gtk::CompositeTemplate;

    use crate::Layer;

    use super::*;

    #[derive(CompositeTemplate, Debug, Default, glib::Properties)]
    #[template(resource = "/org/tsukinaha/yoake/ui/window.ui")]
    #[properties(wrapper_type = super::YoakeWindow)]
    pub struct YoakeWindow {
        #[property(get, set, construct_only)]
        layer: WeakRef<Layer>,
        input_handle: Arc<AtomicBool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for YoakeWindow {
        const NAME: &'static str = "YoakeWindow";
        type Type = super::YoakeWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for YoakeWindow {
        fn constructed(&self) {
            self.parent_constructed();

            let controller = self.input_handle.clone();
            controller.store(true, std::sync::atomic::Ordering::Relaxed);
            std::thread::spawn(move || {
                crate::InputInterface::run_event_loop(controller)
            });
        }

        fn dispose(&self) {
            if let Some(layer) = self.layer.upgrade() {
                layer.destroy();
            }
            self.input_handle.store(false, std::sync::atomic::Ordering::Relaxed);
        }
    }

    impl WidgetImpl for YoakeWindow {}

    impl WindowImpl for YoakeWindow {}

    impl AdwWindowImpl for YoakeWindow {}

    impl ApplicationWindowImpl for YoakeWindow {}

    impl AdwApplicationWindowImpl for YoakeWindow {}
}

glib::wrapper! {
    pub struct YoakeWindow(ObjectSubclass<imp::YoakeWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget, @implements gtk::Accessible;
}

#[template_callbacks]
impl YoakeWindow {
    pub fn new(app: &crate::Application, layer: &crate::Layer) -> Self {
        glib::Object::builder()
            .property("application", app)
            .property("layer", layer)
            .build()
    }

    #[template_callback]
    fn on_layer_switch_notify(&self) {
        if let Some(layer) = self.layer() {
            layer.set_visible(!layer.is_visible());
        }
    }

    #[template_callback]
    fn on_margin_bottom_value_changed(&self, adj: Adjustment) {
        if let Some(layer) = self.layer() {
            layer.set_margin(Edge::Bottom, adj.value() as i32);
        }
    }
}
