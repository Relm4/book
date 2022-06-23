// ANCHOR: all
use gtk::prelude::{ButtonExt, DialogExt, GtkWindowExt, ToggleButtonExt, WidgetExt};
use relm4::*;

// ANCHOR: header_model
struct HeaderModel;
// ANCHOR_END: header_model

// ANCHOR: header_msg
enum HeaderOutput {
    View,
    Edit,
    Export,
}
// ANCHOR_END: header_msg

// ANCHOR: header

#[relm4::component]
impl SimpleComponent for HeaderModel {
    type Input = ();

    type Output = HeaderOutput;

    type InitParams = ();

    type Widgets = HeaderWidgets;

    // ANCHOR: header_widgets
    view! {
        #[root]
        gtk::HeaderBar {
            #[wrap(Some)]
            set_title_widget = &gtk::Box {
                add_css_class: "linked",
                #[name = "group"]
                gtk::ToggleButton {
                    set_label: "View",
                    set_active: true,
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.output.send(HeaderOutput::View)
                        }
                    },
                },
                gtk::ToggleButton {
                    set_label: "Edit",
                    set_group: Some(&group),
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.output.send(HeaderOutput::Edit)
                        }
                    },
                },
                gtk::ToggleButton {
                    set_label: "Export",
                    set_group: Some(&group),
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.output.send(HeaderOutput::Export)
                        }
                    },
                },
            }
        }
    }
    // ANCHOR_END: header_widgets

    fn init(
        _params: Self::InitParams,
        root: &Self::Root,
        sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = HeaderModel;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}

// ANCHOR_END: header

// ANCHOR: dialog_model
struct DialogModel {
    hidden: bool,
}
// ANCHOR_END: dialog_model

// ANCHOR: dialog_msg
enum DialogInput {
    Show,
    Accept,
    Cancel,
}

enum DialogOutput {
    Close,
}
// ANCHOR_END: dialog_msg

#[relm4::component]
impl SimpleComponent for DialogModel {
    type Input = DialogInput;

    type Output = DialogOutput;

    type InitParams = bool;

    type Widgets = DialogWidgets;

    // ANCHOR: dialog_widgets
    view! {
        gtk::MessageDialog {
            set_modal: true,
            #[watch]
            set_visible: !model.hidden,
            set_text: Some("Do you want to close before saving?"),
            set_secondary_text: Some("All unsaved changes will be lost"),
            add_button: ("Close", gtk::ResponseType::Accept),
            add_button: ("Cancel", gtk::ResponseType::Cancel),
            connect_response[sender] => move |_, resp| {
                sender.input.send(if resp == gtk::ResponseType::Accept {
                    DialogInput::Accept
                } else {
                    DialogInput::Cancel
                })
            }
        }
    }
    // ANCHOR_END: dialog_widgets

    fn init(
        params: Self::InitParams,
        root: &Self::Root,
        sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = DialogModel { hidden: params };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    // ANCHOR: dialog_update
    fn update(&mut self, msg: Self::Input, sender: &ComponentSender<Self>) {
        match msg {
            DialogInput::Show => self.hidden = false,
            DialogInput::Accept => {
                self.hidden = true;
                sender.output.send(DialogOutput::Close)
            }
            DialogInput::Cancel => self.hidden = true,
        }
    }
    // ANCHOR_END: dialog_update
}

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
    header: Controller<HeaderModel>,
    dialog: Controller<DialogModel>,
}
// ANCHOR_END: app_model

// ANCHOR: app
#[relm4::component]
impl SimpleComponent for AppModel {
    type Input = AppMsg;

    type Output = ();

    type InitParams = AppMode;

    type Widgets = AppWidgets;

    // ANCHOR: app_widgets
    view! {
        main_window = gtk::Window {
            set_default_width: 500,
            set_default_height: 250,
            set_titlebar: Some(model.header.widget()),

            gtk::Label {
                #[watch]
                set_label: &format!("Placeholder for {:?}", model.mode),
            },
            connect_close_request[sender] => move |_| {
                sender.input.send(AppMsg::CloseRequest);
                gtk::Inhibit(true)
            }
        }
    }
    // ANCHOR_END: app_widgets

    // ANCHOR: app_init
    fn init(
        params: Self::InitParams,
        root: &Self::Root,
        sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel {
            mode: params,
            header: HeaderModel::builder()
                .launch(())
                .forward(&sender.input, |msg| match msg {
                    HeaderOutput::View => AppMsg::SetMode(AppMode::View),
                    HeaderOutput::Edit => AppMsg::SetMode(AppMode::Edit),
                    HeaderOutput::Export => AppMsg::SetMode(AppMode::Export),
                }),
            dialog: DialogModel::builder()
                .launch(true)
                .forward(&sender.input, |msg| match msg {
                    DialogOutput::Close => AppMsg::Close,
                }),
        };
        model.dialog.widget().set_transient_for(Some(root));
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
    // ANCHOR_END: app_init

    // ANCHOR:app_update
    fn update(&mut self, msg: Self::Input, _sender: &ComponentSender<Self>) {
        match msg {
            AppMsg::SetMode(mode) => {
                self.mode = mode;
            }
            AppMsg::CloseRequest => {
                self.dialog.sender().send(DialogInput::Show);
            }
            AppMsg::Close => {
                // TODO: Figure out how to close app.
            }
        }
    }
    // ANCHOR_END:app_update
}
// ANCHOR_END: app

fn main() {
    let relm: RelmApp<AppModel> = RelmApp::new("ewlm4.test.components");
    relm.run(AppMode::Edit);
}
// ANCHOR_END: all
