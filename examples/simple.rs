/* ANCHOR: all */
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets};

#[derive(Default)]
struct AppModel {
    counter: u8,
}

enum AppMsg {
    Increment,
    Decrement,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            AppMsg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
        }
        true
    }
}

// ANCHOR: macro
#[relm4_macros::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        gtk::ApplicationWindow {
            set_title: Some("Simple app"),
            set_default_width: 300,
            set_default_height: 100,
            // ANCHOR: widget_assign
            set_child = Some(&gtk::Box) {
            // ANCHOR_END: widget_assign
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 5,
                set_spacing: 5,

                append = &gtk::Button {
                    set_label: "Increment",
                    // ANCHOR: connect
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::Increment);
                    },
                    // ANCHOR_END: connect
                },
                // ANCHOR: widget_assign_fn
                append = &gtk::Button::with_label("Decrement") {
                // ANCHOR_END: widget_assign_fn
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::Decrement);
                    },
                },
                append = &gtk::Label {
                    set_margin_all: 5,
                    // ANCHOR: watch
                    set_label: watch! { &format!("Counter: {}", model.counter) },
                    // ANCHOR_END: watch
                }
            },
        }
    }
}
// ANCHOR_END: macro

fn main() {
    let model = AppModel::default();
    let app = RelmApp::new(model);
    app.run();
}
/* ANCHOR_END: all */
