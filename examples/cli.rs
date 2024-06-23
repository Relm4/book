/* ANCHOR: all */
use clap::Parser;
use gtk::prelude::GtkWindowExt;
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, SimpleComponent};

struct AppModel {}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Hello world with CLI"),
            set_default_width: 300,
            set_default_height: 100,

            gtk::Label {
                set_label: "Hello world!",
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel {};
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

/* ANCHOR: args_struct */
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// some argument to test
    #[arg(long)]
    non_gtk_arg: bool,

    /// Unknown arguments or everything after -- gets passed through to GTK.
    #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
    gtk_options: Vec<String>,
}
/* ANCHOR_END: args_struct */

fn main() {
    let args = Args::parse();
    dbg!(&args);

    /* ANCHOR: main */
    let program_invocation = std::env::args().next().unwrap();
    let mut gtk_args = vec![program_invocation];
    gtk_args.extend(args.gtk_options.clone());

    let app = RelmApp::new("relm4.test.helloworld_cli");
    app.with_args(gtk_args).run::<AppModel>(());
    /* ANCHOR_END: main */
}
/* ANCHOR_END: all */
