# Component template

```rust,no_run,noplayground
# extern crate relm4;
use gtk::prelude::*;
use relm4::{gtk, SimpleComponent, ComponentSender, ComponentParts};

struct ComponentModel {

}

#[derive(Debug)]
enum ComponentMsg {

}

#[relm4::component]
impl SimpleComponent for ComponentModel {
    type Init = ();
    type Input = ComponentMsg;
    type Output = ();
    type Widgets = ComponentWidgets;

    view! {
        gtk::Box {

        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ComponentModel {

        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, input: Self::Input, sender: ComponentSender<Self>) {
        match input {

        }
    }
}
```
