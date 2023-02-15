use relm4::{Component, ComponentParts, ComponentSender};
use std::time::Duration;

#[derive(Debug)]
struct RemoteData;

struct CommandModel {
    remote_data: RemoteData,
}

#[derive(Debug)]
#[allow(unused)]
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
    tokio::time::sleep(Duration::from_secs(10)).await;
    RemoteData
}
// ANCHOR_END: fetch_data

// ANCHOR: compute_result
fn compute_result() -> RemoteData {
    std::thread::sleep(Duration::from_secs(10));
    RemoteData
}
// ANCHOR_END: compute_result

// ANCHOR: command_output_type
impl Component for CommandModel {
    type CommandOutput = CommandMsg;
    // ANCHOR_END: command_output_type
    type Init = u8;
    type Input = CommandModelMsg;
    type Output = ();
    type Root = ();
    type Widgets = ();

    fn init_root() -> Self::Root {
        todo!()
    }

    // Initialize the UI.
    fn init(_: Self::Init, _: &Self::Root, _: ComponentSender<Self>) -> ComponentParts<Self> {
        todo!()
    }

    // ANCHOR: async_update
    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, _: &Self::Root) {
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
    fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        _sender: ComponentSender<Self>,
        _: &Self::Root,
    ) {
        match message {
            CommandMsg::Data(data) => self.remote_data = data,
        }
    }
    // ANCHOR_END: update_cmd
}

struct SyncCommandModel {
    remote_data: RemoteData,
}

// ANCHOR: sync_command_output_type
impl Component for SyncCommandModel {
    type CommandOutput = CommandMsg;
    // ANCHOR_END: sync_command_output_type
    type Init = u8;
    type Input = CommandModelMsg;
    type Output = ();
    type Root = ();
    type Widgets = ();

    fn init_root() -> Self::Root {
        todo!()
    }

    // Initialize the UI.
    fn init(_: Self::Init, _: &Self::Root, _: ComponentSender<Self>) -> ComponentParts<Self> {
        todo!()
    }

    // ANCHOR: sync_update
    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, _: &Self::Root) {
        match msg {
            CommandModelMsg::FetchData => {
                sender.spawn_oneshot_command(|| {
                    // Run CPU-bound background task
                    CommandMsg::Data(compute_result())
                });
            }
        }
    }
    // ANCHOR_END: sync_update
    // ANCHOR: sync_update_cmd
    fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        _sender: ComponentSender<Self>,
        _: &Self::Root,
    ) {
        match message {
            CommandMsg::Data(data) => self.remote_data = data,
        }
    }
    // ANCHOR_END: sync_update_cmd
}

fn main() {}
