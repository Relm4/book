use std::convert::identity;
use std::time::Duration;

use gtk::prelude::*;
use relm4::prelude::*;
use relm4::{Worker, WorkerController};

// ANCHOR:worker_impl
#[derive(Debug)]
enum AppMsg {
    Increment,
    Decrement,
}

#[derive(Debug)]
enum AsyncHandlerMsg {
    DelayedIncrement,
    DelayedDecrement,
}

struct AsyncHandler;

impl Worker for AsyncHandler {
    type Init = ();
    type Input = AsyncHandlerMsg;
    type Output = AppMsg;

    fn init(_init: Self::Init, _sender: ComponentSender<Self>) -> Self {
        Self
    }

    fn update(&mut self, msg: AsyncHandlerMsg, sender: ComponentSender<Self>) {
        // Simulating heavy CPU-bound task
        std::thread::sleep(Duration::from_secs(1));

        // Send the result of the calculation back
        match msg {
            AsyncHandlerMsg::DelayedIncrement => sender.output(AppMsg::Increment),
            AsyncHandlerMsg::DelayedDecrement => sender.output(AppMsg::Decrement),
        }
        .unwrap()
    }
}
// ANCHOR_END:worker_impl

// ANCHOR: app_model
struct AppModel {
    counter: u8,
    worker: WorkerController<AsyncHandler>,
}
// ANCHOR_END: app_model

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Worker Counter"),
            set_default_width: 300,
            set_default_height: 100,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 5,
                set_spacing: 5,

                gtk::Button {
                    set_label: "Increment",
                    connect_clicked[sender = model.worker.sender().clone()] => move |_| {
                        sender.send(AsyncHandlerMsg::DelayedIncrement).unwrap();
                    },
                },
                gtk::Button::with_label("Decrement") {
                    connect_clicked[sender = model.worker.sender().clone()] => move |_| {
                        sender.send(AsyncHandlerMsg::DelayedDecrement).unwrap();
                    },
                },
                gtk::Label {
                    set_margin_all: 5,
                    #[watch]
                    set_label: &format!("Counter: {}", model.counter),
                },
            },
        }
    }

    // ANCHOR: worker_construction
    fn init(_: (), root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = AppModel {
            counter: 0,
            worker: AsyncHandler::builder()
                .detach_worker(())
                .forward(sender.input_sender(), identity),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
    // ANCHOR_END: worker_construction

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            AppMsg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.worker");
    app.run::<AppModel>(());
}
