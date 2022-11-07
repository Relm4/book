#![allow(unused)]

/* ANCHOR: all */
use relm4::{component::EmptyRoot, Component, ComponentParts, ComponentSender, SimpleComponent};
use std::time::Duration;

struct AppModel {
    rsa_key: (),
}

#[derive(Debug)]
enum AppMsg {
    RunHeavyCalculation,
}

// ANCHOR: rsa_key
fn generate_rsa_key() {
    std::thread::sleep(Duration::from_secs(10));
}
// ANCHOR_END: rsa_key

impl SimpleComponent for AppModel {
    type Init = u8;
    type Input = AppMsg;
    type Output = ();
    type Widgets = ();
    type Root = EmptyRoot;

    fn init_root() -> Self::Root {
        todo!()
    }

    // Initialize the UI.
    fn init(_: Self::Init, _: &Self::Root, _: ComponentSender<Self>) -> ComponentParts<Self> {
        todo!()
    }

    // ANCHOR: slow_update
    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::RunHeavyCalculation => {
                self.rsa_key = generate_rsa_key();
            }
        }
    }
    // ANCHOR_END: slow_update
}

#[derive(Debug)]
struct RemoteData;

struct CommandModel {
    remote_data: RemoteData,
}

#[derive(Debug)]
enum CommandModelMsg {
    FetchData,
}

// ANCHOR: command_msg
#[derive(Debug)]
enum CommandMsg {
    Data(RemoteData),
}
// ANCHOR_END: command_msg

// ANCHOR: fetch_data
async fn fetch_data() -> RemoteData {
    RemoteData
}
// ANCHOR_END: fetch_data

// ANCHOR: command_output_type 
impl Component for CommandModel {
    type CommandOutput = CommandMsg;
// ANCHOR_END: command_output_type 
    type Input = CommandModelMsg;
    type Output = ();
    type Init = u8;
    type Root = EmptyRoot;
    type Widgets = ();

    fn init_root() -> Self::Root {
        todo!()
    }

    // Initialize the UI.
    fn init(_: Self::Init, _: &Self::Root, _: ComponentSender<Self>) -> ComponentParts<Self> {
        todo!()
    }

    // ANCHOR: async_update
    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            CommandModelMsg::FetchData => {
                sender.oneshot_command(async {
                    // Run async background task
                    CommandMsg::Data(fetch_data().await)
                });
            }
        }
    }
    // ANCHOR_END: async_update
    // ANCHOR: update_cmd
    fn update_cmd(&mut self, message: Self::CommandOutput, _sender: ComponentSender<Self>) {
        match message {
            CommandMsg::Data(data) => self.remote_data = data,
        }
    }
    // ANCHOR_END: update_cmd
}

fn main() {}
/* ANCHOR_END: all */
