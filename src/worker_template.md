# Worker template

```rust,no_run,noplayground
# extern crate relm4;
use gtk::prelude::*;
use relm4::{gtk, ComponentSender, Worker};

struct WorkerModel {

}

#[derive(Debug)]
enum WorkerMsg {

}

impl Worker for WorkerModel {
    type Init = ();
    type Input = WorkerMsg;
    type Output = ();

    fn init(init: Self::Init, sender: ComponentSender<Self>) -> Self {
        todo!();
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {

        }
    }
}
