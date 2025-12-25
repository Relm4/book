//! This example is a modified version of the `macro_reference` example in the [main Relm4
//! repository][Relm4 repo].
//!
//! [Relm4 repo]: https://github.com/Relm4/Relm4/blob/main/examples/macro_reference.rs

use gtk::prelude::{
    BoxExt, ButtonExt, GridExt, GtkWindowExt, OrientableExt, ToggleButtonExt, WidgetExt,
};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

// ANCHOR: model
#[tracker::track]
struct AppModel {
    value: u8,
}
// ANCHOR_END: model

// ANCHOR: msg
#[derive(Debug)]
enum AppMsg {
    Increment,
    Decrement,
}
// ANCHOR_END: msg

struct AppInit {
    counter: u8,
}

// ANCHOR: component
#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = AppInit;
    type Input = AppMsg;
    type Output = ();

    view! {
        #[root]
        #[name(main_window)]
        gtk::Window {
            set_title: Some("Macro reference example"),
            set_default_width: 300,
            set_default_height: 100,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                // ANCHOR: connect
                append: inc_button = &gtk::Button {
                    set_label: "Increment",
                    // Only set this if `icon_name` is Some
                    set_icon_name?: icon_name,
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::Increment);
                    }
                },

                gtk::Button {
                    set_label: "Decrement",
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::Decrement);
                    }
                },
                // ANCHOR_END: connect

                gtk::Grid {
                    attach[1, 1, 1, 1] = &gtk::Label {
                        // Alternative: #[track = "counter.value.is_multiple_of(10)"]
                        #[track(counter.value.is_multiple_of(10))]
                        set_label: &format!("Grid works! ({})", counter.value),
                    }
                },

                // A conditional widget
                // Alternative: #[transition = "SlideLeft"]
                #[transition(SlideLeft)]
                append = if counter.value.is_multiple_of(2) {
                    gtk::Label {
                        set_label: "Value is even",
                    }
                } else if counter.value.is_multiple_of(3) {
                    gtk::Label {
                        set_label: "Value is dividable by 3",
                    }
                } else {
                    gtk::Label {
                        set_label: "Value is odd",
                    }
                },

                #[transition = "SlideRight"]
                append: match_stack = match counter.value {
                    (0..=2) => {
                        gtk::Label {
                            set_label: "Value is smaller than 3",
                        }
                    },
                    _ => {
                        gtk::Label {
                            set_label: "Value is higher than 2",
                        }
                    }
                },

                append = &gtk::Label,

                gtk::Label::builder()
                    .label("Builder pattern works!")
                    .selectable(true)
                    .build(),

                gtk::Label::new(Some("Constructors work!")),

                /// Counter label
                gtk::Label {
                    #[watch]
                    set_label: &format!("Counter: {}", counter.value),
                    #[track]
                    set_margin_all: counter.value.into(),
                },

                // ANCHOR: block-signal
                gtk::ToggleButton {
                    set_label: "Counter is even",
                    #[watch]
                    #[block_signal(toggle_handler)]
                    set_active: counter.value.is_multiple_of(2),

                    connect_toggled[sender] => move |_| {
                        sender.input(AppMsg::Increment);
                    } @toggle_handler,
                },
                // ANCHOR_END: block-signal

                #[local]
                local_label -> gtk::Label {
                    set_opacity: 0.7,
                },

                #[local_ref]
                local_ref_label -> gtk::Label {
                    set_opacity: 0.7,
                    set_size_request: (40, 40),
                },
            }
        },
        gtk::Window {
            set_title: Some("Another window"),
            set_default_width: 300,
            set_default_height: 100,
            set_transient_for: Some(&main_window),
            // Empty args
            hide: (),

            #[watch]
            set_visible: counter.value == 42,

            #[name = "my_label_name"]
            gtk::Label {
                set_label: "You made it to 42!",
            }
        }
    }

    additional_fields! {
        test_field: u8,
    }

    // Initialize the UI.
    fn init(
        init: Self::Init,
        renamed_root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let counter = AppModel {
            value: init.counter,
            tracker: 0,
        };

        let test_field = 0;

        // Set icon name randomly to Some("go-up-symbolic") or None
        let icon_name = rand::random::<bool>().then_some("go-up-symbolic");

        let local_label = gtk::Label::new(Some("local_label"));
        let local_ref_label_value = gtk::Label::new(Some("local_ref_label"));
        let local_ref_label = &local_ref_label_value;
        // Insert the macro code generation here
        let widgets = view_output!();

        ComponentParts {
            model: counter,
            widgets,
        }
    }

    // ANCHOR: app_update
    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        self.reset();
        match msg {
            AppMsg::Increment => {
                self.set_value(self.value.wrapping_add(1));
            }
            AppMsg::Decrement => {
                self.set_value(self.value.wrapping_sub(1));
            }
        }
    }
    // ANCHOR_END: app_update
}
// ANCHOR_END: component

fn main() {
    let app = RelmApp::new("relm4.example.macro_reference");
    app.run::<AppModel>(AppInit { counter: 0 });
}
