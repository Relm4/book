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

// Returns a random icon different from the excluded one (avoids repeats).
fn gen_unique_icon(exclude: &'static str) -> &'static str {
    let mut rnd = random_icon_name();
    while rnd == exclude {
        rnd = random_icon_name()
    }
    rnd
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
        root: Self::Root,
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

        // Insert the macro code generation here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    // ANCHOR: update
    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        // reset tracker value of the model
        self.reset();

        match message {
            AppInput::UpdateFirst => {
                self.set_first_icon(gen_unique_icon(self.first_icon));
            }
            AppInput::UpdateSecond => {
                self.set_second_icon(gen_unique_icon(self.second_icon));
            }
        }
        self.set_identical(self.first_icon == self.second_icon);
    }
    // ANCHOR_END: update
}

// ANCHOR: main
fn main() {
    let app = RelmApp::new("relm4.test.simple");
    relm4::set_global_css(".identical { background: #00ad5c; }");
    app.run::<AppModel>(());
}
// ANCHOR_END: main
// ANCHOR_END: all
