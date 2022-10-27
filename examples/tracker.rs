// ANCHOR: all
use gtk::prelude::{BoxExt, ButtonExt, OrientableExt};
use rand::prelude::IteratorRandom;
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

// ANCHOR: icons
const ICON_LIST: &[&str] = &[
    "bookmark-new-symbolic",
    "edit-copy-symbolic",
    "edit-cut-symbolic",
    "edit-find-symbolic",
    "starred-symbolic",
    "system-run-symbolic",
    "emoji-objects-symbolic",
    "emoji-nature-symbolic",
    "display-brightness-symbolic",
];

fn random_icon_name() -> &'static str {
    ICON_LIST
        .iter()
        .choose(&mut rand::thread_rng())
        .expect("Could not choose a random icon")
}
// ANCHOR_END: icons

// The track proc macro allows to easily track changes to different
// fields of the model
// ANCHOR: model
#[tracker::track]
struct AppModel {
    first_icon: &'static str,
    second_icon: &'static str,
    identical: bool,
}
// ANCHOR_END: model

// ANCHOR: msg
#[derive(Debug)]
enum AppInput {
    UpdateFirst,
    UpdateSecond,
}
// ANCHOR_END: msg

#[relm4::component]
impl SimpleComponent for AppModel {
    type Widgets = AppWidgets;

    type Init = ();

    type Input = AppInput;
    type Output = ();

    // ANCHOR: view
    view! {
        #[root]
        gtk::ApplicationWindow {
            // ANCHOR: track1
            #[track = "model.changed(AppModel::identical())"]
            set_class_active: ("identical", model.identical),
            // ANCHOR_END: track1
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
                set_margin_all: 10,
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    gtk::Image {
                        set_pixel_size: 50,
                        // ANCHOR: track2
                        #[track = "model.changed(AppModel::first_icon())"]
                        set_icon_name: Some(model.first_icon),
                        // ANCHOR_END: track2
                    },
                    gtk::Button {
                        set_label: "New random image",
                        connect_clicked[sender] => move |_| {
                            sender.input(AppInput::UpdateFirst)
                        }
                    }
                },
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    gtk::Image {
                        set_pixel_size: 50,
                        #[track = "model.changed(AppModel::second_icon())"]
                        set_icon_name: Some(model.second_icon),
                    },
                    gtk::Button {
                        set_label: "New random image",
                        connect_clicked[sender] => move |_| {
                            sender.input(AppInput::UpdateSecond)
                        }
                    }
                },
            }
        }
    }
    // ANCHOR_END: view

    // Initialize the UI.
    fn init(
        _params: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // ANCHOR: model_init
        let model = AppModel {
            first_icon: random_icon_name(),
            second_icon: random_icon_name(),
            identical: false,
            tracker: 0,
        };
        // ANCHOR_END: model_init

        // ANCHOR: post_init
        relm4::set_global_css(b".identical { background: #00ad5c; }");

        // Insert the macro code generation here
        let widgets = view_output!();
        // ANCHOR_END: post_init

        ComponentParts { model, widgets }
    }

    // ANCHOR: update
    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        // reset tracker value of the model
        self.reset();

        match message {
            AppInput::UpdateFirst => {
                self.set_first_icon(random_icon_name());
            }
            AppInput::UpdateSecond => {
                self.set_second_icon(random_icon_name());
            }
        }
        self.set_identical(self.first_icon == self.second_icon);
    }
    // ANCHOR_END: update
}

fn main() {
    let app: RelmApp = RelmApp::new("relm4.test.simple");
    app.run::<AppModel>(());
}
// ANCHOR_END: all
