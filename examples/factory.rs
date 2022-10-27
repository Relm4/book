// ANCHOR: all
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::factory::{DynamicIndex, FactoryComponent, FactoryComponentSender, FactoryVecDeque};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

// ANCHOR: factory_model
#[derive(Debug)]
struct Counter {
    value: u8,
}
// ANCHOR_END: factory_model

// ANCHOR: factory_input
#[derive(Debug)]
enum CounterMsg {
    Increment,
    Decrement,
}
// ANCHOR_END: factory_input

// ANCHOR: factory_output
#[derive(Debug)]
enum CounterOutput {
    SendFront(DynamicIndex),
    MoveUp(DynamicIndex),
    MoveDown(DynamicIndex),
}
// ANCHOR_END: factory_output

// ANCHOR: factory
// ANCHOR: factory_impl_start
#[relm4::factory]
impl FactoryComponent for Counter {
    type Init = u8;
    type Input = CounterMsg;
    type Output = CounterOutput;
    type CommandOutput = ();
    type Widgets = CounterWidgets;
    type ParentInput = AppMsg;
    type ParentWidget = gtk::Box;
    // ANCHOR_END: factory_impl_start

    // ANCHOR: factory_view
    view! {
        root = gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 10,

            #[name(label)]
            gtk::Label {
                #[watch]
                set_label: &self.value.to_string(),
                set_width_chars: 3,
            },

            #[name(add_button)]
            gtk::Button {
                set_label: "+",
                connect_clicked => CounterMsg::Increment,
            },

            #[name(remove_button)]
            gtk::Button {
                set_label: "-",
                connect_clicked => CounterMsg::Decrement,
            },

            #[name(move_up_button)]
            gtk::Button {
                set_label: "Up",
                connect_clicked[sender, index] => move |_| {
                    sender.output(CounterOutput::MoveUp(index.clone()))
                }
            },

            #[name(move_down_button)]
            gtk::Button {
                set_label: "Down",
                connect_clicked[sender, index] => move |_| {
                    sender.output(CounterOutput::MoveDown(index.clone()))
                }
            },

            #[name(to_front_button)]
            gtk::Button {
                set_label: "To Start",
                connect_clicked[sender, index] => move |_| {
                    sender.output(CounterOutput::SendFront(index.clone()))
                }
            }
        }
    }
    // ANCHOR_END: factory_view

    // ANCHOR: output_to_parent
    fn output_to_parent_input(output: Self::Output) -> Option<AppMsg> {
        Some(match output {
            CounterOutput::SendFront(index) => AppMsg::SendFront(index),
            CounterOutput::MoveUp(index) => AppMsg::MoveUp(index),
            CounterOutput::MoveDown(index) => AppMsg::MoveDown(index),
        })
    }
    // ANCHOR_END: output_to_parent

    // ANCHOR: factory_init_model
    fn init_model(
        value: Self::Init,
        _index: &DynamicIndex,
        _sender: FactoryComponentSender<Self>,
    ) -> Self {
        Self { value }
    }
    // ANCHOR_END: factory_init_model

    // ANCHOR: factory_update
    fn update(&mut self, msg: Self::Input, _sender: FactoryComponentSender<Self>) {
        match msg {
            CounterMsg::Increment => {
                self.value = self.value.wrapping_add(1);
            }
            CounterMsg::Decrement => {
                self.value = self.value.wrapping_sub(1);
            }
        }
    }
    // ANCHOR_END: factory_update
}
// ANCHOR_END: factory

// ANCHOR: main_types
struct App {
    created_widgets: u8,
    counters: FactoryVecDeque<Counter>,
}

#[derive(Debug)]
enum AppMsg {
    AddCounter,
    RemoveCounter,
    SendFront(DynamicIndex),
    MoveUp(DynamicIndex),
    MoveDown(DynamicIndex),
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = u8;
    type Input = AppMsg;
    type Output = ();
    type Widgets = AppWidgets;
    // ANCHOR_END: main_types

    // ANCHOR: main_view
    view! {
        gtk::Window {
            set_title: Some("Factory example"),
            set_default_size: (300, 100),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Button {
                    set_label: "Add counter",
                    connect_clicked => AppMsg::AddCounter,
                },

                gtk::Button {
                    set_label: "Remove counter",
                    connect_clicked => AppMsg::RemoveCounter,
                },

                #[local_ref]
                counter_box -> gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                }
            }
        }
    }
    // ANCHOR_END: main_view

    // ANCHOR: main_init
    // Initialize the UI.
    fn init(
        counter: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let counters = FactoryVecDeque::new(gtk::Box::default(), sender.input_sender());
        let model = App {
            created_widgets: counter,
            counters,
        };

        let counter_box = model.counters.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
    // ANCHOR_END: main_init

    // ANCHOR: main_update
    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::AddCounter => {
                self.counters.guard().push_back(self.created_widgets);
                self.created_widgets = self.created_widgets.wrapping_add(1);
            }
            AppMsg::RemoveCounter => {
                self.counters.guard().pop_back();
            }
            AppMsg::SendFront(index) => {
                self.counters.guard().move_front(index.current_index());
            }
            AppMsg::MoveDown(index) => {
                let index = index.current_index();
                let new_index = index + 1;
                // Already at the end?
                if new_index < self.counters.len() {
                    self.counters.guard().move_to(index, new_index);
                }
            }
            AppMsg::MoveUp(index) => {
                let index = index.current_index();
                // Already at the start?
                if index != 0 {
                    self.counters.guard().move_to(index, index - 1);
                }
            }
        }
    }
    // ANCHOR_END: main_update
}

// ANCHOR: main
fn main() {
    let app = RelmApp::new("relm4.example.factory");
    app.run::<App>(0);
}
// ANCHOR_END: main
// ANCHOR_END: all
