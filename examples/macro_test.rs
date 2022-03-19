// ANCHOR: all
use gtk::prelude::{BoxExt, ButtonExt, GridExt, GtkWindowExt, OrientableExt, WidgetExt};
use relm4::{
    send, AppUpdate, ComponentUpdate, Components, Model, RelmApp, RelmComponent, Sender,
    WidgetPlus, Widgets,
};

// ANCHOR: model
struct AppModel {
    counter: u8,
    classes: Vec<&'static str>,
    decrement: bool,
}
// ANCHOR_END: model

// ANCHOR: msg
enum AppMsg {
    Increment,
    Decrement,
}
// ANCHOR_END: msg

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

// ANCHOR: app_update
impl AppUpdate for AppModel {
    fn update(
        &mut self,
        msg: AppMsg,
        _components: &AppComponents,
        _sender: Sender<AppMsg>,
    ) -> bool {
        match msg {
            AppMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
                self.decrement = false;
            }
            AppMsg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
                self.decrement = true;
            }
        }
        true
    }
}
// ANCHOR_END: app_update

// ANCHOR: button_comp
enum ButtonMsg {}

struct ButtonModel {}

impl Model for ButtonModel {
    type Msg = ButtonMsg;
    type Widgets = ButtonWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for ButtonModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        ButtonModel {}
    }

    fn update(
        &mut self,
        _msg: ButtonMsg,
        _components: &(),
        _sender: Sender<ButtonMsg>,
        _parent_sender: Sender<AppMsg>,
    ) {
    }
}

#[relm4_macros::widget]
impl Widgets<ButtonModel, AppModel> for ButtonWidgets {
    view! {
        gtk::Button {
            set_label: "ButtonComponent!",
        }
    }
}

pub struct AppComponents {
    button1: RelmComponent<ButtonModel, AppModel>,
    button2: RelmComponent<ButtonModel, AppModel>,
}
// ANCHOR_END: button_comp

impl Components<AppModel> for AppComponents {
    fn init_components(
        model: &AppModel,
        parent_widgets: &AppWidgets,
        sender: Sender<AppMsg>,
    ) -> Self {
        AppComponents {
            button1: RelmComponent::new(model, parent_widgets, sender.clone()),
            button2: RelmComponent::new(model, parent_widgets, sender),
        }
    }
}

// ANCHOR: new_label
fn new_label() -> gtk::Label {
    gtk::Label::new(Some("test"))
}
// ANCHOR_END: new_label

// ANCHOR: widgets
#[relm4_macros::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
            main_window = gtk::ApplicationWindow {
    // ANCHOR: trait_fn_assign
                gtk::prelude::GtkWindowExt::set_title: Some("Simple app"),
    // ANCHOR_END: trait_fn_assign
                set_default_width: 300,
                set_default_height: 100,
    // ANCHOR: set_child_widget
                set_child = Some(&gtk::Box) {
    // ANCHOR_END: set_child_widget
                    set_orientation: gtk::Orientation::Vertical,
    // ANCHOR: optional_assign
                    set_margin_all?: Some(5),
    // ANCHOR_END: optional_assign
                    set_spacing: 5,

                    append: component!(components.button1.root_widget()),
                    append: inc_button = &gtk::Button {
                        set_label: "Increment",
                        connect_clicked(sender) => move |_| {
                            send!(sender, AppMsg::Increment);
                        },
    // ANCHOR: iterative_assign
                        add_css_class: iterate!(&model.classes),
    // ANCHOR_END: iterative_assign
                    },
                    append = &gtk::Button::new() {
    // ANCHOR: track
                        set_label: track!(model.decrement, &format!("Last decrement at {}", model.counter)),
    // ANCHOR_END: track
    // ANCHOR: connect
                        connect_clicked(sender) => move |_| {
                            send!(sender, AppMsg::Decrement);
                        },
    // ANCHOR_END: connect
                    },
                    append = &new_label() -> gtk::Label {
                        set_margin_all: 5,
    // ANCHOR: watch
                        set_label: watch! { &format!("Counter: {}", model.counter) },
    // ANCHOR_END: watch
                    },
                    append = &gtk::Grid {
                        set_vexpand: true,
                        set_hexpand: true,
                        set_row_spacing: 10,
                        set_column_spacing: 10,
                        set_column_homogeneous: true,
                        attach(1, 1, 1, 1) = &gtk::Label {
                            set_label: "grid test 1",
                        },
                        attach(1, 2, 1, 1) = &gtk::Label {
                            set_label: "grid test 2",
                        },
                        attach(2, 1, 1, 1) = &gtk::Label {
                            set_label: "grid test 3",
                        },
    // ANCHOR: component
                        attach(2, 2, 1, 1): component!(components.button2.root_widget())
    // ANCHOR_END: component
                    }
                },
            }
        }

    additional_fields! {
        test_field: u8,
    }

    // ANCHOR: pre_init
    fn pre_init() {
        let mut test_field = 0;
        println!("Pre init! test_field: {}", test_field);
    }
    // ANCHOR_END: pre_init

    // ANCHOR: post_init
    fn post_init() {
        relm4::set_global_css(b".first { color: green; } .second { border: 1px solid orange; }");
        test_field = 42;
        println!("Post init! test_field: {}", test_field);
    }
    // ANCHOR_END: post_init

    // ANCHOR: manual_view
    fn manual_view() {
        self.test_field += 1;
        println!("Manual view! test_field: {}", self.test_field);
    }
    // ANCHOR_END: manual_view
}
// ANCHOR_END: widgets

fn main() {
    let model = AppModel {
        counter: 0,
        classes: vec!["first", "second"],
        decrement: false,
    };
    let app = RelmApp::new(model);
    app.run();
}
// ANCHOR_END: all
