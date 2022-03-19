# Component template

```rust,no_run,noplayground
use gtk::prelude::{WidgetExt};
use relm4::*;

struct ComponentModel {

}

enum ComponentMsg {

}

impl Model for ComponentModel {
    type Msg = ComponentMsg;
    type Widgets = ComponentWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for ComponentModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        ComponentModel {

        }
    }

    fn update(
        &mut self,
        msg: ComponentMsg,
        _components: &(),
        sender: Sender<ComponentMsg>,
        parent_sender: Sender<AppMsg>,
    ) {
        match msg {

        }
    }
}

#[relm4_macros::widget]
impl Widgets<ComponentModel, AppModel> for ComponentWidgets {
    view! {
        
    }
}
```