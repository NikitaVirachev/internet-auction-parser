use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Label, Spinner, TextView};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/example/window.ui")]
pub struct Window {
    #[template_child]
    pub update_btn: TemplateChild<Button>,
    #[template_child]
    pub logs: TemplateChild<TextView>,
    #[template_child]
    pub date_label: TemplateChild<Label>,
    #[template_child]
    pub spinner: TemplateChild<Spinner>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        self.update_btn.set_cursor_from_name(Some("pointer"));

        self.obj().setup_actions();
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}
