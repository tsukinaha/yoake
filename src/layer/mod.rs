use adw::subclass::prelude::*;
use gtk::{glib, prelude::{BoxExt, WidgetExt}};

use crate::input::INPUT_CHANNEL;

mod imp {

    use glib::subclass::InitializingObject;
    use gtk::CompositeTemplate;
    use gtk4_layer_shell::{
        Edge,
        Layer,
        LayerShell,
    };

    use super::*;

    #[derive(CompositeTemplate, Debug, Default)]
    #[template(resource = "/org/tsukinaha/yoake/ui/layer.ui")]
    pub struct YoakeLayer {
        #[template_child]
        pub layer_box: TemplateChild<gtk::Box>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for YoakeLayer {
        const NAME: &'static str = "YoakeLayer";
        type Type = super::YoakeLayer;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for YoakeLayer {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.init_layer_shell();
            obj.set_layer(Layer::Overlay);
            obj.set_margin(Edge::Left, 40);
            obj.set_margin(Edge::Right, 40);
            obj.set_margin(Edge::Top, 20);

            let anchors = [
                (Edge::Left, false),
                (Edge::Right, false),
                (Edge::Top, false),
                (Edge::Bottom, true),
            ];

            for (anchor, state) in anchors {
                obj.set_anchor(anchor, state);
            }


            obj.listen_key_events();
        }
    }

    impl WidgetImpl for YoakeLayer {}

    impl WindowImpl for YoakeLayer {}

    impl AdwWindowImpl for YoakeLayer {}

    impl ApplicationWindowImpl for YoakeLayer {}

    impl AdwApplicationWindowImpl for YoakeLayer {}
}

glib::wrapper! {
    pub struct YoakeLayer(ObjectSubclass<imp::YoakeLayer>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget, @implements gtk::Accessible;
}

impl YoakeLayer {
    pub fn new(app: &crate::Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    pub fn listen_key_events(&self) {
        glib::spawn_future_local(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            async move {
                while let Ok(key) = INPUT_CHANNEL.rx.recv_async().await {
                    obj.add_key_button(&key);
                }
            }
        ));
    }

    pub fn add_key_button(&self, key: &str) {
        let button = gtk::Button::with_label(key);
        button.add_css_class("key-button");
        let revealer = gtk::Revealer::builder()
            .transition_type(gtk::RevealerTransitionType::SlideLeft)
            .child(&button)
            .build();
        self.imp().layer_box.append(&revealer);
        revealer.set_reveal_child(true);
        glib::timeout_add_seconds_local_once(3, glib::clone!(
            #[weak]
            revealer,
            move || {
                revealer.set_visible(false);
            }
        ));
    }
}
