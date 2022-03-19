# App template

```rust,no_run,noplayground
use gtk::prelude::{WidgetExt};
use relm4::*;

struct AppComponents {
    component: RelmComponent<ComponentModel, AppModel>,
}

impl Components<AppModel> for AppComponents {
    fn init_components(
        parent_model: &AppModel,
        parent_widgets: &AppWidgets,
        parent_sender: Sender<AppMsg>,
    ) -> Self {
        AppComponents {
            component: RelmComponent::new(parent_model, parent_widgets, parent_sender.clone()),
        }
    }
}

enum AppMsg {
	
}

struct AppModel {

}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, components: &AppComponents, sender: Sender<AppMsg>) -> bool {
        match msg {

        }
        true
    }
}

#[relm4_macros::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        main_window = gtk::ApplicationWindow {
            
        }
    }
}

fn main() {
    let model = AppModel {
        
    };
    let relm = RelmApp::new(model);
    relm.run();
}

```