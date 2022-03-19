// ANCHOR: all
use gtk::prelude::{BoxExt, ButtonExt, DialogExt, GtkWindowExt, ToggleButtonExt, WidgetExt};
use relm4::Sender;
use relm4::*;

// ANCHOR: header_msg
enum HeaderMsg {
    View,
    Edit,
    Export,
}
// ANCHOR_END: header_msg

// ANCHOR: header_model
struct HeaderModel {}
// ANCHOR_END: header_model

// ANCHOR: header_model_impl
impl Model for HeaderModel {
    type Msg = HeaderMsg;
    type Widgets = HeaderWidgets;
    type Components = ();
}
// ANCHOR_END: header_model_impl

// ANCHOR: header_update
impl ComponentUpdate<AppModel> for HeaderModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        HeaderModel {}
    }

    fn update(
        &mut self,
        msg: HeaderMsg,
        _components: &(),
        _sender: Sender<HeaderMsg>,
        parent_sender: Sender<AppMsg>,
    ) {
        match msg {
            HeaderMsg::View => {
                send!(parent_sender, AppMsg::SetMode(AppMode::View));
            }
            HeaderMsg::Edit => {
                send!(parent_sender, AppMsg::SetMode(AppMode::Edit));
            }
            HeaderMsg::Export => {
                send!(parent_sender, AppMsg::SetMode(AppMode::Export));
            }
        }
    }
}
// ANCHOR_END: header_update

// ANCHOR: header_widgets
#[relm4_macros::widget]
impl Widgets<HeaderModel, AppModel> for HeaderWidgets {
    view! {
        gtk::HeaderBar {
            set_title_widget = Some(&gtk::Box) {
                add_css_class: "linked",
                append: group = &gtk::ToggleButton {
                    set_label: "View",
                    set_active: true,
                    connect_toggled(sender) => move |btn| {
                        if btn.is_active() {
                            send!(sender, HeaderMsg::View);
                        }
                    },
                },
                append = &gtk::ToggleButton {
                    set_label: "Edit",
                    set_group: Some(&group),
                    connect_toggled(sender) => move |btn| {
                        if btn.is_active() {
                            send!(sender, HeaderMsg::Edit);
                        }
                    },
                },
                append = &gtk::ToggleButton {
                    set_label: "Export",
                    set_group: Some(&group),
                    connect_toggled(sender) => move |btn| {
                        if btn.is_active() {
                            send!(sender, HeaderMsg::Export);
                        }
                    },
                },
            }
        }
    }
}
// ANCHOR_END: header_widgets

// ANCHOR: dialog_model
struct DialogModel {
    hidden: bool,
}
// ANCHOR_END: dialog_model

// ANCHOR: dialog_msg
enum DialogMsg {
    Show,
    Accept,
    Cancel,
}
// ANCHOR_END: dialog_msg

// ANCHOR: dialog_model_impl
impl Model for DialogModel {
    type Msg = DialogMsg;
    type Widgets = DialogWidgets;
    type Components = ();
}
// ANCHOR_END: dialog_model_impl

// ANCHOR: dialog_update
impl ComponentUpdate<AppModel> for DialogModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        DialogModel { hidden: true }
    }

    fn update(
        &mut self,
        msg: DialogMsg,
        _components: &(),
        _sender: Sender<DialogMsg>,
        parent_sender: Sender<AppMsg>,
    ) {
        match msg {
            DialogMsg::Show => self.hidden = false,
            DialogMsg::Accept => {
                self.hidden = true;
                send!(parent_sender, AppMsg::Close);
            }
            DialogMsg::Cancel => self.hidden = true,
        }
    }
}
// ANCHOR_END: dialog_update

// ANCHOR: dialog_widgets
#[relm4_macros::widget]
impl Widgets<DialogModel, AppModel> for DialogWidgets {
    view! {
        gtk::MessageDialog {
            set_transient_for: Some(&parent_widgets.main_window),
            set_modal: true,
            set_visible: watch!(!model.hidden),
            set_text: Some("Do you want to close before saving?"),
            set_secondary_text: Some("All unsaved changes will be lost"),
            add_button: args!("Close", gtk::ResponseType::Accept),
            add_button: args!("Cancel", gtk::ResponseType::Cancel),
            connect_response(sender) => move |_, resp| {
                send!(sender, if resp == gtk::ResponseType::Accept {
                    DialogMsg::Accept
                } else {
                    DialogMsg::Cancel
                });
            }
        }
    }
}
// ANCHOR_END: dialog_widgets

// ANCHOR: components
struct AppComponents {
    header: RelmComponent<HeaderModel, AppModel>,
    dialog: RelmComponent<DialogModel, AppModel>,
}
// ANCHOR_END: components

// ANCHOR: components_impl
impl Components<AppModel> for AppComponents {
    fn init_components(
        parent_model: &AppModel,
        parent_widgets: &AppWidgets,
        parent_sender: Sender<AppMsg>,
    ) -> Self {
        AppComponents {
            header: RelmComponent::new(parent_model, parent_widgets, parent_sender.clone()),
            dialog: RelmComponent::new(parent_model, parent_widgets, parent_sender),
        }
    }
}
// ANCHOR_END: components_impl

// ANCHOR: app_model
#[derive(Debug)]
enum AppMode {
    View,
    Edit,
    Export,
}

enum AppMsg {
    SetMode(AppMode),
    CloseRequest,
    Close,
}

struct AppModel {
    mode: AppMode,
}
// ANCHOR_END: app_model

// ANCHOR: app_model_impl
impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}
// ANCHOR_END: app_model_impl

// ANCHOR: app_widgets
#[relm4_macros::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        main_window = gtk::ApplicationWindow {
            set_default_width: 500,
            set_default_height: 250,
            set_titlebar: component!(Some(components.header.root_widget())),
            set_child = Some(&gtk::Label) {
                set_label: watch!(&format!("Placeholder for {:?}", model.mode)),
            },
            connect_close_request(sender) => move |_| {
                send!(sender, AppMsg::CloseRequest);
                gtk::Inhibit(true)
            }
        }
    }
}
// ANCHOR_END: app_widgets

// ANCHOR: app_update
impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, components: &AppComponents, _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::SetMode(mode) => {
                self.mode = mode;
            }
            AppMsg::CloseRequest => {
                components.dialog.send(DialogMsg::Show).unwrap();
            }
            AppMsg::Close => {
                return false;
            }
        }
        true
    }
}
// ANCHOR_END: app_update

fn main() {
    let model = AppModel {
        mode: AppMode::View,
    };
    let relm = RelmApp::new(model);
    relm.run();
}
// ANCHOR_END: all
