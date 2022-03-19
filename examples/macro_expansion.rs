// ANCHOR: all
use gtk::prelude::{BoxExt, ButtonExt, GridExt, GtkWindowExt, OrientableExt, WidgetExt};
use relm4::{
    send, AppUpdate, ComponentUpdate, Components, Model, RelmApp, RelmComponent, Sender,
    WidgetPlus, Widgets,
};

struct AppModel {
    counter: u8,
    classes: Vec<&'static str>,
    decrement: bool,
}

enum AppMsg {
    Increment,
    Decrement,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

impl AppUpdate for AppModel {
    fn update(
        &mut self,
        msg: AppMsg,
        _components: &AppComponents,
        _sender: Sender<AppMsg>,
    ) -> bool {
        match msg {
            AppMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
                self.decrement = false;
            }
            AppMsg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
                self.decrement = true;
            }
        }
        true
    }
}

enum ButtonMsg {}

struct ButtonModel {}

impl Model for ButtonModel {
    type Msg = ButtonMsg;
    type Widgets = ButtonWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for ButtonModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        ButtonModel {}
    }
    fn update(
        &mut self,
        _msg: ButtonMsg,
        _components: &(),
        _sender: Sender<ButtonMsg>,
        _parent_sender: Sender<AppMsg>,
    ) {
    }
}

#[allow(dead_code)]
struct ButtonWidgets {
    _gtk_button_0: gtk::Button,
}

impl Widgets<ButtonModel, AppModel> for ButtonWidgets {
    type Root = gtk::Button;
    /// Initialize the UI.
    fn init_view(
        model: &ButtonModel,
        parent_widgets: &<AppModel as ::relm4::Model>::Widgets,
        sender: ::gtk::glib::Sender<<ButtonModel as ::relm4::Model>::Msg>,
    ) -> Self {
        let _gtk_button_0 = gtk::Button::default();
        _gtk_button_0.set_label("ButtonComponent!");
        Self { _gtk_button_0 }
    }
    fn connect_components(&self, model: &ButtonModel, components: &<ButtonModel as ::relm4::Model>::Components) {}
    /// Return the root widget.
    fn root_widget(&self) -> Self::Root {
        self._gtk_button_0.clone()
    }
    /// Update the view to represent the updated model.
    fn view(
        &mut self,
        model: &ButtonModel,
        sender: ::gtk::glib::Sender<<ButtonModel as ::relm4::Model>::Msg>,
    ) {
    }
}

pub struct AppComponents {
    button1: RelmComponent<ButtonModel, AppModel>,
    button2: RelmComponent<ButtonModel, AppModel>,
}

impl Components<AppModel> for AppComponents {
    fn init_components(
        model: &AppModel,
        parent_widgets: &AppWidgets,
        sender: Sender<AppMsg>,
    ) -> Self {
        AppComponents {
            button1: RelmComponent::new(model, parent_widgets, sender.clone()),
            button2: RelmComponent::new(model, parent_widgets, sender),
        }
    }
}

fn new_label() -> gtk::Label {
    gtk::Label::new(Some("test"))
}

// ANCHOR: widgets_struct
#[allow(dead_code)]
struct AppWidgets {
    main_window: gtk::ApplicationWindow,
    _gtk_box_7: gtk::Box,
    inc_button: gtk::Button,
    _gtk_button_new_1: gtk::Button,
    _new_label_2: gtk::Label,
    _gtk_grid_6: gtk::Grid,
    _gtk_label_3: gtk::Label,
    _gtk_label_4: gtk::Label,
    _gtk_label_5: gtk::Label,
    test_field: u8,
}
// ANCHOR_END: widgets_struct

// ANCHOR: widgets_impl
impl Widgets<AppModel, ()> for AppWidgets {
    type Root = gtk::ApplicationWindow;
    // ANCHOR_END: widgets_impl
    // ANCHOR: pre_init
    /// Initialize the UI.
    fn init_view(
        model: &AppModel,
        parent_widgets: &(),
        sender: ::gtk::glib::Sender<AppMsg>,
    ) -> Self {
        let mut test_field = 0;
        println!("Pre init! test_field: {}", test_field);
        // ANCHOR_END: pre_init
        // ANCHOR: widget_init
        let main_window = gtk::ApplicationWindow::default();
        let _gtk_box_7 = gtk::Box::default();
        let inc_button = gtk::Button::default();
        let _gtk_button_new_1 = gtk::Button::new();
        let _new_label_2 = new_label();
        // ANCHOR_END: widget_init
        let _gtk_grid_6 = gtk::Grid::default();
        let _gtk_label_3 = gtk::Label::default();
        let _gtk_label_4 = gtk::Label::default();
        let _gtk_label_5 = gtk::Label::default();
        // ANCHOR: property_assign
        gtk::prelude::GtkWindowExt::set_title(&main_window, Some("Simple app"));
        main_window.set_default_width(300);
        main_window.set_default_height(100);
        _gtk_box_7.set_orientation(gtk::Orientation::Vertical);
        if let Some(__p_assign) = Some(5) {
            _gtk_box_7.set_margin_all(__p_assign);
        }
        _gtk_box_7.set_spacing(5);
        inc_button.set_label("Increment");
        for __elem in &model.classes {
            inc_button.add_css_class(__elem);
        }
        // ANCHOR_END: property_assign
        _gtk_button_new_1.set_label(&format!("Last decrement at {}", model.counter));
        _new_label_2.set_margin_all(5);
        _new_label_2.set_label(&format!("Counter: {}", model.counter));
        _gtk_grid_6.set_vexpand(true);
        _gtk_grid_6.set_hexpand(true);
        _gtk_grid_6.set_row_spacing(10);
        _gtk_grid_6.set_column_spacing(10);
        _gtk_grid_6.set_column_homogeneous(true);
        _gtk_label_3.set_label("grid test 1");
        _gtk_label_4.set_label("grid test 2");
        _gtk_label_5.set_label("grid test 3");
        // ANCHOR: connect
        {
            #[allow(clippy::redundant_clone)]
            let sender = sender.clone();
            inc_button.connect_clicked(move |_| {
                send!(sender, AppMsg::Increment);
            });
        }
        {
            #[allow(clippy::redundant_clone)]
            let sender = sender.clone();
            _gtk_button_new_1.connect_clicked(move |_| {
                send!(sender, AppMsg::Decrement);
            });
        }
        // ANCHOR_END: connect
        // ANCHOR: post_init
        relm4::set_global_css(b".first { color: green; } .second { border: 1px solid orange; }");
        test_field = 42;
        println!("Post init! test_field: {}", test_field);
        // ANCHOR_END: post_init
        // ANCHOR: return
        Self {
            main_window,
            _gtk_box_7,
            inc_button,
            _gtk_button_new_1,
            _new_label_2,
            _gtk_grid_6,
            _gtk_label_3,
            _gtk_label_4,
            _gtk_label_5,
            test_field,
        }
    }
    // ANCHOR_END: return

    // ANCHOR: connect_components
    fn connect_components(&self, model: &AppModel, components: &<AppModel as ::relm4::Model>::Components) {
        self.main_window.set_child(Some(&self._gtk_box_7));
        self._gtk_box_7.append(components.button1.root_widget());
        self._gtk_box_7.append(&self.inc_button);
        self._gtk_box_7.append(&self._gtk_button_new_1);
        self._gtk_box_7.append(&self._new_label_2);
        self._gtk_box_7.append(&self._gtk_grid_6);
        self._gtk_grid_6.attach(&self._gtk_label_3, 1, 1, 1, 1);
        self._gtk_grid_6.attach(&self._gtk_label_4, 1, 2, 1, 1);
        self._gtk_grid_6.attach(&self._gtk_label_5, 2, 1, 1, 1);
        self._gtk_grid_6
            .attach(components.button2.root_widget(), 2, 2, 1, 1);
    }
    // ANCHOR_END: connect_components

    // ANCHOR: root_widget
    /// Return the root widget.
    fn root_widget(&self) -> Self::Root {
        self.main_window.clone()
    }
    // ANCHOR_END: root_widget

    // ANCHOR: manual_view
    /// Update the view to represent the updated model.
    fn view(
        &mut self,
        model: &AppModel,
        sender: ::gtk::glib::Sender<<AppModel as ::relm4::Model>::Msg>,
    ) {
        self.test_field += 1;
        println!("Manual view! test_field: {}", self.test_field);
        // ANCHOR_END: manual_view

        // ANCHOR: macro_view
        self._new_label_2
            .set_label(&format!("Counter: {}", model.counter));

        if model.decrement {
            self._gtk_button_new_1
                .set_label(&format!("Last decrement at {}", model.counter));
        }
        // ANCHOR_END: macro_view
    }
}

fn main() {
    let model = AppModel {
        counter: 0,
        classes: vec!["first", "second"],
        decrement: false,
    };
    let app = RelmApp::new(model);
    app.run();
}
// ANCHOR_END: all
