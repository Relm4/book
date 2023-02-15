#![allow(unused)]

/* ANCHOR: all */
use relm4::{Component, ComponentParts, ComponentSender, SimpleComponent};
use std::time::Duration;

struct AppModel {
    rsa_key: (),
}

#[derive(Debug)]
enum AppMsg {
    GenerateKey,
}

// ANCHOR: fetch_rsa_key
async fn fetch_rsa_key() {
    tokio::time::sleep(Duration::from_secs(10)).await;
}
// ANCHOR_END: fetch_rsa_key

// ANCHOR: rsa_key
fn generate_rsa_key() {
    std::thread::sleep(Duration::from_secs(10));
}
// ANCHOR_END: rsa_key

impl SimpleComponent for AppModel {
    type Init = u8;
    type Input = AppMsg;
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

    // ANCHOR: slow_update
    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::GenerateKey => {
                self.rsa_key = generate_rsa_key();
            }
        }
    }
    // ANCHOR_END: slow_update
}

fn main() {}
/* ANCHOR_END: all */
