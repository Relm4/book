// ANCHOR: all
use gtk::prelude::*;
use relm4::prelude::*;
use relm4::Controller;

// ANCHOR: settings
/// Configuration for the alert dialog component
pub struct AlertSettings {
    /// Large text
    pub text: String,
    /// Optional secondary, smaller text
    pub secondary_text: Option<String>,
    /// Modal dialogs freeze other windows as long they are visible
    pub is_modal: bool,
    /// Sets color of the accept button to red if the theme supports it
    pub destructive_accept: bool,
    /// Text for confirm button
    pub confirm_label: String,
    /// Text for cancel button
    pub cancel_label: String,
    /// Text for third option button. If [`None`] the third button won't be created.
    pub option_label: Option<String>,
}
// ANCHOR_END: settings

// ANCHOR: child_component
// ANCHOR: model
/// Alert dialog component.
pub struct Alert {
    settings: AlertSettings,
    is_active: bool,
}
// ANCHOR_END: model

// ANCHOR: input
/// Messages that can be sent to the alert dialog component
#[derive(Debug)]
pub enum AlertMsg {
    /// Message sent by the parent to view the dialog
    Show,

    #[doc(hidden)]
    Response(gtk::ResponseType),
}
// ANCHOR_END: input

// ANCHOR: output
/// User action performed on the alert dialog.
#[derive(Debug)]
pub enum AlertResponse {
    /// User clicked confirm button.
    Confirm,

    /// User clicked cancel button.
    Cancel,

    /// User clicked user-supplied option.
    Option,
}
// ANCHOR_END: output

/// Widgets of the alert dialog component.
#[relm4::component(pub)]
impl SimpleComponent for Alert {
    // ANCHOR: types
    type Widgets = AlertWidgets;
    type Init = AlertSettings;
    type Input = AlertMsg;
    type Output = AlertResponse;
    // ANCHOR_END: types

    // ANCHOR: view
    view! {
        #[name = "dialog"]
        gtk::MessageDialog {
            set_message_type: gtk::MessageType::Question,
            #[watch]
            set_visible: model.is_active,
            connect_response[sender] => move |_, response| {
                sender.input(AlertMsg::Response(response));
            },

            // Apply configuration
            set_text: Some(&model.settings.text),
            set_secondary_text: model.settings.secondary_text.as_deref(),
            set_modal: model.settings.is_modal,
            add_button: (&model.settings.confirm_label, gtk::ResponseType::Accept),
            add_button: (&model.settings.cancel_label, gtk::ResponseType::Cancel),
        }
    }
    // ANCHOR_END: view

    // ANCHOR: init_model
    fn init(
        settings: AlertSettings,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Alert {
            settings,
            is_active: false,
        };

        let widgets = view_output!();

        if let Some(option_label) = &model.settings.option_label {
            widgets
                .dialog
                .add_button(option_label, gtk::ResponseType::Other(0));
        }

        if model.settings.destructive_accept {
            let accept_widget = widgets
                .dialog
                .widget_for_response(gtk::ResponseType::Accept)
                .expect("No button for accept response set");
            accept_widget.add_css_class("destructive-action");
        }

        ComponentParts { model, widgets }
    }
    // ANCHOR_END: init_model

    // ANCHOR: component_update
    fn update(&mut self, input: AlertMsg, sender: ComponentSender<Self>) {
        match input {
            AlertMsg::Show => {
                self.is_active = true;
            }
            AlertMsg::Response(ty) => {
                self.is_active = false;
                sender
                    .output(match ty {
                        gtk::ResponseType::Accept => AlertResponse::Confirm,
                        gtk::ResponseType::Other(_) => AlertResponse::Option,
                        _ => AlertResponse::Cancel,
                    })
                    .unwrap();
            }
        }
    }
    // ANCHOR_END: component_update
}
// ANCHOR_END: child_component

// ANCHOR: app_component
// ANCHOR: app_model
struct App {
    counter: u8,
    alert_toggle: bool,
    dialog: Controller<Alert>,
    second_dialog: Controller<Alert>,
}
// ANCHOR_END: app_model

#[derive(Debug)]
enum AppMsg {
    Increment,
    Decrement,
    CloseRequest,
    Save,
    Close,
    Ignore,
}

#[relm4::component]
impl SimpleComponent for App {
    type Input = AppMsg;
    type Output = ();
    type Init = ();

    view! {
        main_window = gtk::ApplicationWindow {
            set_title: Some("Simple app"),
            set_default_width: 300,
            set_default_height: 100,

            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::CloseRequest);
                gtk::glib::Propagation::Stop
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 5,
                set_spacing: 5,

                append = &gtk::Button {
                    set_label: "Increment",
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::Increment);
                    },
                },
                append = &gtk::Button {
                    set_label: "Decrement",
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::Decrement);
                    },
                },
                append = &gtk::Label {
                    set_margin_all: 5,
                    #[watch]
                    set_label: &format!("Counter: {}", model.counter),
                },
                append = &gtk::Button {
                    set_label: "Close",
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::CloseRequest);
                    },
                },
            },
        }
    }

    fn update(&mut self, msg: AppMsg, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            AppMsg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
            // ANCHOR: close
            AppMsg::CloseRequest => {
                if self.counter == 42 {
                    relm4::main_application().quit();
                } else {
                    self.alert_toggle = !self.alert_toggle;
                    if self.alert_toggle {
                        self.dialog.emit(AlertMsg::Show);
                    } else {
                        self.second_dialog.emit(AlertMsg::Show);
                    }
                }
            }
            // ANCHOR_END: close
            AppMsg::Save => {
                println!("* Open save dialog here *");
            }
            AppMsg::Close => {
                relm4::main_application().quit();
            }
            AppMsg::Ignore => (),
        }
    }

    // ANCHOR: app_init
    fn init(_: (), root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = App {
            counter: 0,
            alert_toggle: false,
            dialog: Alert::builder()
                .transient_for(&root)
                .launch(AlertSettings {
                    text: String::from("Do you want to quit without saving? (First alert)"),
                    secondary_text: Some(String::from("Your counter hasn't reached 42 yet")),
                    confirm_label: String::from("Close without saving"),
                    cancel_label: String::from("Cancel"),
                    option_label: Some(String::from("Save")),
                    is_modal: true,
                    destructive_accept: true,
                })
                .forward(sender.input_sender(), convert_alert_response),
            second_dialog: Alert::builder()
                .transient_for(&root)
                .launch(AlertSettings {
                    text: String::from("Do you want to quit without saving? (Second alert)"),
                    secondary_text: Some(String::from("Your counter hasn't reached 42 yet")),
                    confirm_label: String::from("Close without saving"),
                    cancel_label: String::from("Cancel"),
                    option_label: Some(String::from("Save")),
                    is_modal: true,
                    destructive_accept: true,
                })
                .forward(sender.input_sender(), convert_alert_response),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
    // ANCHOR_END: app_init
}

fn convert_alert_response(response: AlertResponse) -> AppMsg {
    match response {
        AlertResponse::Confirm => AppMsg::Close,
        AlertResponse::Cancel => AppMsg::Ignore,
        AlertResponse::Option => AppMsg::Save,
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.alert");
    app.run::<App>(());
}
// ANCHOR_END: app_component
// ANCHOR_END: all
