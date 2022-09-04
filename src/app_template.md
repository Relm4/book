# App template

```rust,no_run,noplayground
# extern crate relm4;
use gtk::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, SimpleComponent};

#[derive(Debug)]
enum AppMsg {

}

struct App {

}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();
    type Widgets = AppWidgets;

    view! {
        gtk::ApplicationWindow {

        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App {

        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, input: Self::Input, sender: ComponentSender<Self>) {
        match input {

        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.templates.app");
    app.run::<App>(());
}

```
