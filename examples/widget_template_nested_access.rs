// ANCHOR: all
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{gtk, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent, WidgetTemplate};

// ANCHOR: home_page_template
#[relm4::widget_template]
impl WidgetTemplate for HomePage {
    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 3,

            #[name = "btn_go_settings"]
            gtk::Button {
                #[wrap(Some)]
                set_child = &gtk::Image {
                    set_icon_name: Some("emblem-system-symbolic"),
                },
            },
        }
    }
}
// ANCHOR_END: home_page_template

// ANCHOR: settings_page_template
#[relm4::widget_template]
impl WidgetTemplate for SettingsPage {
    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 3,

            #[name = "btn_dark_mode"]
            gtk::Button {
                #[wrap(Some)]
                set_child = &gtk::Image {
                    set_icon_name: Some("night-light-symbolic"),
                },
            },

            #[name = "btn_go_homepage"]
            gtk::Button {
                #[wrap(Some)]
                set_child = &gtk::Image {
                    set_icon_name: Some("user-home-symbolic"),
                },
            },
        }
    }
}
// ANCHOR_END: settings_page_template

// ANCHOR: main_window_template
#[relm4::widget_template]
impl WidgetTemplate for MainWindow {
    view! {
        gtk::Window {
            set_title: Some("Nested Widget template"),
            set_default_width: 300,
            set_default_height: 100,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                #[name(stk_pages)]
                gtk::Stack {
                    set_margin_all: 7,

                    #[template]
                    #[name = "home_page"]
                    add_child = &HomePage {} -> {
                        set_name: "main",
                    },

                    #[template]
                    #[name = "settings_page"]
                    add_child = &SettingsPage {} -> {
                        set_name: "settings",
                    },
                },

            },
        }
    }
}
// ANCHOR_END: main_window_template

// ANCHOR: component_start
#[derive(Default)]
struct AppModel {
    current_page: &'static str,
}

#[derive(Debug)]
enum Message {
    PageHome,
    PageSettings,
    DarkMode,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = Message;
    type Output = ();

    view! {

        #[template]
        MainWindow {

            #[template_child]
            settings_page.btn_dark_mode {
                connect_clicked => Message::DarkMode
            },

            #[template_child]
            settings_page.btn_go_homepage {
                connect_clicked => Message::PageHome
            },

            #[template_child]
            home_page.btn_go_settings {
                connect_clicked => Message::PageSettings
            },

            #[template_child]
            stk_pages {
                #[watch]
                set_visible_child_name: model.current_page,
            }
        },
    }
    // ANCHOR_END: component_start

    fn init(
        _init_param: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            current_page: "main",
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Message, _sender: ComponentSender<Self>) {
        match msg {
            Message::DarkMode => {
                println!("Mode changed");
            }
            Message::PageHome => {
                self.current_page = "main";
            }
            Message::PageSettings => {
                self.current_page = "settings";
            }
        }
    }
}

fn main() {}
// ANCHOR_END: all
