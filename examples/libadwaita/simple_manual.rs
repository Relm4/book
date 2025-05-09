/* ANCHOR: all */
use gtk::glib::clone;
use gtk::prelude::{BoxExt, ButtonExt};
use relm4::adw::prelude::AdwWindowExt;
use relm4::{adw, gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

// ANCHOR: model
struct AppModel {
    counter: u8,
}
// ANCHOR_END: model

// ANCHOR: msg
#[derive(Debug)]
enum AppMsg {
    Increment,
    Decrement,
}
// ANCHOR_END: msg

// ANCHOR: widgets
struct AppWidgets {
    label: gtk::Label,
}
// ANCHOR_END: widgets

// ANCHOR: simple_component
impl SimpleComponent for AppModel {
    type Input = AppMsg;
    type Output = ();

    type Init = u8;

    type Root = adw::Window;
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        adw::Window::builder().title("Simple app").build()
    }

    fn init(
        counter: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel { counter };

        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();

        let header = adw::HeaderBar::builder()
            .title_widget(&gtk::Label::new(Some("Simple app")))
            .build();

        let inc_button = gtk::Button::with_label("Increment");
        let dec_button = gtk::Button::with_label("Decrement");

        let label = gtk::Label::new(Some(&format!("Counter: {}", model.counter)));
        label.set_margin_all(5);

        window.set_content(Some(&vbox));
        content.set_margin_all(5);
        content.append(&inc_button);
        content.append(&dec_button);
        content.append(&label);
        vbox.append(&header);
        vbox.append(&content);

        inc_button.connect_clicked(clone!(
            #[strong]
            sender,
            move |_| {
                sender.input(AppMsg::Increment);
            }
        ));

        dec_button.connect_clicked(clone!(
            #[strong]
            sender,
            move |_| {
                sender.input(AppMsg::Decrement);
            }
        ));

        let widgets = AppWidgets { label };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            AppMsg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
        }
    }

    /// Update the view to represent the updated model.
    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        widgets
            .label
            .set_label(&format!("Counter: {}", self.counter));
    }
}
// ANCHOR_END: app_update

// ANCHOR: main
fn main() {
    let app = RelmApp::new("relm4.adw_test.simple_manual");
    app.run::<AppModel>(0);
}
// ANCHOR_END: main
/* ANCHOR_END: all */
