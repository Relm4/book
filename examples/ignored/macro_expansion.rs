// ANCHOR: all
#![feature(prelude_import)]
//! This example is a modified version of the `macro_reference` example in the [main Relm4
//! repository][Relm4 repo].
//!
//! [Relm4 repo]: https://github.com/Relm4/Relm4/blob/main/examples/macro_reference.rs
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use gtk::prelude::{
    BoxExt, ButtonExt, GridExt, GtkWindowExt, OrientableExt, ToggleButtonExt, WidgetExt,
};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, SimpleComponent, WidgetPlus};
struct AppModel {
    value: u8,
    tracker: u8,
}
impl AppModel {
    #[allow(dead_code, non_snake_case)]
    /// Get an immutable reference to this field.
    fn get_value(&self) -> &u8 {
        &self.value
    }
    #[allow(dead_code, non_snake_case)]
    /// Get a mutable reference to this field. Marks the field as changed.
    fn get_mut_value(&mut self) -> &mut u8 {
        self.tracker |= Self::value();
        &mut self.value
    }
    #[allow(dead_code, non_snake_case)]
    /// Use a closure to update this field. Marks the field as changed.
    fn update_value<F: Fn(&mut u8)>(&mut self, f: F) {
        self.tracker |= Self::value();
        f(&mut self.value);
    }
    #[allow(dead_code, non_snake_case)]
    /// Get bit mask to look for changes on this field.
    fn value() -> u8 {
        1 << 0usize
    }
    #[allow(dead_code, non_snake_case)]
    /// Setter method. Will mark field as changed.
    fn set_value(&mut self, value: u8) {
        if self.value != value {
            self.tracker |= Self::value();
        }
        self.value = value;
    }
    #[allow(dead_code)]
    /// Use this to check whether any changes made to this struct.
    fn track_all() -> u8 {
        u8::MAX
    }
    #[allow(dead_code)]
    /// Use this to mark all fields of the struct as changed.
    fn mark_all_changed(&mut self) {
        self.tracker = u8::MAX;
    }
    /// Check for changes made to this struct.
    fn changed(&self, mask: u8) -> bool {
        self.tracker & mask != 0
    }
    /// Resets the tracker of this struct.
    fn reset(&mut self) {
        self.tracker = 0;
    }
}
enum AppMsg {
    Increment,
    Decrement,
}
#[automatically_derived]
impl ::core::fmt::Debug for AppMsg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            AppMsg::Increment => ::core::fmt::Formatter::write_str(f, "Increment"),
            AppMsg::Decrement => ::core::fmt::Formatter::write_str(f, "Decrement"),
        }
    }
}
struct AppInit {
    counter: u8,
}
// ANCHOR: widgets_struct
#[allow(dead_code)]
struct AppWidgets {
    #[allow(missing_docs)]
    main_window: gtk::Window,
    #[allow(missing_docs)]
    _gtk_box_14: gtk::Box,
    #[allow(missing_docs)]
    inc_button: gtk::Button,
    #[allow(missing_docs)]
    _gtk_button_0: gtk::Button,
    #[allow(missing_docs)]
    _gtk_grid_2: gtk::Grid,
    #[allow(missing_docs)]
    _gtk_label_1: gtk::Label,
    #[allow(missing_docs)]
    _conditional_widget_3: relm4::gtk::Stack,
    #[allow(missing_docs)]
    _gtk_label_4: gtk::Label,
    #[allow(missing_docs)]
    _gtk_label_5: gtk::Label,
    #[allow(missing_docs)]
    _gtk_label_6: gtk::Label,
    #[allow(missing_docs)]
    match_stack: relm4::gtk::Stack,
    #[allow(missing_docs)]
    _gtk_label_7: gtk::Label,
    #[allow(missing_docs)]
    _gtk_label_8: gtk::Label,
    #[allow(missing_docs)]
    _gtk_label_9: gtk::Label,
    #[allow(missing_docs)]
    _gtk_label_builder_10: gtk::Label,
    #[allow(missing_docs)]
    _gtk_label_new_11: gtk::Label,
    /// Counter label
    _gtk_label_12: gtk::Label,
    #[allow(missing_docs)]
    _gtk_togglebutton_13: gtk::ToggleButton,
    #[allow(missing_docs)]
    toggle_handler: relm4::gtk::glib::signal::SignalHandlerId,
    #[allow(missing_docs)]
    local_label: gtk::Label,
    #[allow(missing_docs)]
    local_ref_label: gtk::Label,
    #[allow(missing_docs)]
    _gtk_window_15: gtk::Window,
    #[allow(missing_docs)]
    my_label_name: gtk::Label,
    test_field: u8,
}
// ANCHOR_END: widgets_struct
#[automatically_derived]
#[allow(dead_code)]
impl ::core::fmt::Debug for AppWidgets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "main_window",
            "_gtk_box_14",
            "inc_button",
            "_gtk_button_0",
            "_gtk_grid_2",
            "_gtk_label_1",
            "_conditional_widget_3",
            "_gtk_label_4",
            "_gtk_label_5",
            "_gtk_label_6",
            "match_stack",
            "_gtk_label_7",
            "_gtk_label_8",
            "_gtk_label_9",
            "_gtk_label_builder_10",
            "_gtk_label_new_11",
            "_gtk_label_12",
            "_gtk_togglebutton_13",
            "toggle_handler",
            "local_label",
            "local_ref_label",
            "_gtk_window_15",
            "my_label_name",
            "test_field",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &&self.main_window,
            &&self._gtk_box_14,
            &&self.inc_button,
            &&self._gtk_button_0,
            &&self._gtk_grid_2,
            &&self._gtk_label_1,
            &&self._conditional_widget_3,
            &&self._gtk_label_4,
            &&self._gtk_label_5,
            &&self._gtk_label_6,
            &&self.match_stack,
            &&self._gtk_label_7,
            &&self._gtk_label_8,
            &&self._gtk_label_9,
            &&self._gtk_label_builder_10,
            &&self._gtk_label_new_11,
            &&self._gtk_label_12,
            &&self._gtk_togglebutton_13,
            &&self.toggle_handler,
            &&self.local_label,
            &&self.local_ref_label,
            &&self._gtk_window_15,
            &&self.my_label_name,
            &&self.test_field,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(
            f,
            "AppWidgets",
            names,
            values,
        )
    }
}
// ANCHOR: widgets_impl
impl SimpleComponent for AppModel {
    type Init = AppInit;
    type Input = AppMsg;
    type Output = ();
    type Widgets = AppWidgets;
    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        self.reset();
        match msg {
            AppMsg::Increment => {
                self.set_value(self.value.wrapping_add(1));
            }
            AppMsg::Decrement => {
                self.set_value(self.value.wrapping_sub(1));
            }
        }
    }
    type Root = gtk::Window;
    fn init_root() -> Self::Root {
        let main_window = gtk::Window::default();
        main_window
    }
    // ANCHOR_END: widgets_impl
    // ANCHOR: update_view
    /// Update the view to represent the updated model.
    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {
        struct __DoNotReturnManually;
        let _no_manual_return: __DoNotReturnManually = (move || {
            #[allow(unused_variables)]
            let Self::Widgets {
                main_window,
                _gtk_box_14,
                inc_button,
                _gtk_button_0,
                _gtk_grid_2,
                _gtk_label_1,
                _conditional_widget_3,
                _gtk_label_4,
                _gtk_label_5,
                _gtk_label_6,
                match_stack,
                _gtk_label_7,
                _gtk_label_8,
                _gtk_label_9,
                _gtk_label_builder_10,
                _gtk_label_new_11,
                _gtk_label_12,
                _gtk_togglebutton_13,
                toggle_handler,
                local_label,
                local_ref_label,
                _gtk_window_15,
                my_label_name,
                test_field,
            } = widgets;
            #[allow(unused_variables)]
            let counter = self;
            if (counter.value % 10 == 0) {
                _gtk_label_1
                    .set_label(
                        &{
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Grid works! (", ")"],
                                    &[::core::fmt::ArgumentV1::new_display(&counter.value)],
                                ),
                            );
                            res
                        },
                    );
            }
            let __current_page = _conditional_widget_3
                .visible_child_name()
                .map_or("".to_string(), |s| s.as_str().to_string());
            // ANCHOR: track_expression
            _conditional_widget_3
                .set_visible_child_name(
                    if counter.value % 2 == 0 {
                        let __page_active: bool = (__current_page == "0");
                        "0"
                    } else if counter.value % 3 == 0 {
                        let __page_active: bool = (__current_page == "1");
                        "1"
                    } else {
                        let __page_active: bool = (__current_page == "2");
                        "2"
                    },
                );
            // ANCHOR_END: track_expression
            let __current_page = match_stack
                .visible_child_name()
                .map_or("".to_string(), |s| s.as_str().to_string());
            match_stack
                .set_visible_child_name(
                    match counter.value {
                        (0..=2) => {
                            let __page_active: bool = (__current_page == "0");
                            "0"
                        }
                        _ => {
                            let __page_active: bool = (__current_page == "1");
                            "1"
                        }
                    },
                );
            // ANCHOR: watch
            _gtk_label_12
                .set_label(
                    &{
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["Counter: "],
                                &[::core::fmt::ArgumentV1::new_display(&counter.value)],
                            ),
                        );
                        res
                    },
                );
            // ANCHOR_END: watch
            // ANCHOR: track
            if (counter.changed(Self::value())) {
                _gtk_label_12.set_margin_all(counter.value.into());
            }
            // ANCHOR_END: track
            {
                use relm4::WidgetRef;
                #[allow(clippy::needless_borrow)]
                relm4::gtk::prelude::ObjectExt::block_signal(
                    _gtk_togglebutton_13.widget_ref(),
                    &toggle_handler,
                );
            }
            _gtk_togglebutton_13.set_active(counter.value % 2 == 0);
            {
                use relm4::WidgetRef;
                #[allow(clippy::needless_borrow)]
                relm4::gtk::prelude::ObjectExt::unblock_signal(
                    _gtk_togglebutton_13.widget_ref(),
                    &toggle_handler,
                );
            }
            _gtk_window_15.set_visible(counter.value == 42);
            (move || {})();
            __DoNotReturnManually
        })();
    }
    // ANCHOR_END: update_view
    // ANCHOR: pre_init
    fn init(
        init: Self::Init,
        renamed_root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let counter = AppModel {
            value: init.counter,
            tracker: 0,
        };
        let test_field = 0;
        let icon_name = rand::random::<bool>().then(|| "go-up-symbolic");
        let local_label = gtk::Label::new(Some("local_label"));
        let local_ref_label_value = gtk::Label::new(Some("local_ref_label"));
        let local_ref_label = &local_ref_label_value;
        // ANCHOR_END: pre_init
        // ANCHOR: widget_init
        let main_window = renamed_root.clone();
        let _gtk_box_14 = gtk::Box::default();
        let inc_button = gtk::Button::default();
        let _gtk_button_0 = gtk::Button::default();
        let _gtk_grid_2 = gtk::Grid::default();
        let _gtk_label_1 = gtk::Label::default();
        let _conditional_widget_3 = relm4::gtk::Stack::default();
        _conditional_widget_3
            .set_transition_type(relm4::gtk::StackTransitionType::SlideLeft);
        let _gtk_label_4 = gtk::Label::default();
        let _gtk_label_5 = gtk::Label::default();
        let _gtk_label_6 = gtk::Label::default();
        let match_stack = relm4::gtk::Stack::default();
        match_stack.set_transition_type(relm4::gtk::StackTransitionType::SlideRight);
        let _gtk_label_7 = gtk::Label::default();
        let _gtk_label_8 = gtk::Label::default();
        let _gtk_label_9 = gtk::Label::default();
        let _gtk_label_builder_10 = gtk::Label::builder()
            .label("Builder pattern works!")
            .selectable(true)
            .build();
        let _gtk_label_new_11 = gtk::Label::new(Some("Constructors work!"));
        let _gtk_label_12 = gtk::Label::default();
        let _gtk_togglebutton_13 = gtk::ToggleButton::default();
        let _gtk_window_15 = gtk::Window::default();
        let my_label_name = gtk::Label::default();
        // ANCHOR_END: widget_init
        // ANCHOR: connect
        {
            #[allow(clippy::redundant_clone)]
            #[allow(clippy::clone_on_copy)]
            let sender = sender.clone();
            inc_button
                .connect_clicked(move |_| {
                    sender.input(AppMsg::Increment);
                });
        }
        {
            #[allow(clippy::redundant_clone)]
            #[allow(clippy::clone_on_copy)]
            let sender = sender.clone();
            _gtk_button_0
                .connect_clicked(move |_| {
                    sender.input(AppMsg::Decrement);
                });
        }
        let toggle_handler = {
            #[allow(clippy::redundant_clone)]
            #[allow(clippy::clone_on_copy)]
            let sender = sender.clone();
            _gtk_togglebutton_13
                .connect_toggled(move |_| {
                    sender.input(AppMsg::Increment);
                })
        };
        // ANCHOR_END: connect
        {}
        // ANCHOR: property_assign
        main_window.set_title(Some("Macro reference example"));
        main_window.set_default_width(300);
        main_window.set_default_height(100);
        relm4::RelmContainerExt::container_add(&main_window, &_gtk_box_14);
        _gtk_box_14.set_orientation(gtk::Orientation::Vertical);
        _gtk_box_14.set_spacing(5);
        _gtk_box_14.set_margin_all(5);
        _gtk_box_14.append(&inc_button);
        inc_button.set_label("Increment");
        if let Some(__p_assign) = icon_name {
            inc_button.set_icon_name(__p_assign);
        }
        relm4::RelmContainerExt::container_add(&_gtk_box_14, &_gtk_button_0);
        _gtk_button_0.set_label("Decrement");
        relm4::RelmContainerExt::container_add(&_gtk_box_14, &_gtk_grid_2);
        _gtk_grid_2.attach(&_gtk_label_1, 1, 1, 1, 1);
        _gtk_label_1
            .set_label(
                &{
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["Grid works! (", ")"],
                            &[::core::fmt::ArgumentV1::new_display(&counter.value)],
                        ),
                    );
                    res
                },
            );
        _gtk_box_14.append(&_conditional_widget_3);
        _conditional_widget_3.add_named(&_gtk_label_4, Some("0"));
        _gtk_label_4.set_label("Value is even");
        _conditional_widget_3.add_named(&_gtk_label_5, Some("1"));
        _gtk_label_5.set_label("Value is dividable by 3");
        _conditional_widget_3.add_named(&_gtk_label_6, Some("2"));
        _gtk_label_6.set_label("Value is odd");
        _gtk_box_14.append(&match_stack);
        match_stack.add_named(&_gtk_label_7, Some("0"));
        _gtk_label_7.set_label("Value is smaller than 3");
        match_stack.add_named(&_gtk_label_8, Some("1"));
        _gtk_label_8.set_label("Value is higher than 2");
        _gtk_box_14.append(&_gtk_label_9);
        // ANCHOR_END: property_assign
        relm4::RelmContainerExt::container_add(&_gtk_box_14, &_gtk_label_builder_10);
        relm4::RelmContainerExt::container_add(&_gtk_box_14, &_gtk_label_new_11);
        relm4::RelmContainerExt::container_add(&_gtk_box_14, &_gtk_label_12);
        _gtk_label_12
            .set_label(
                &{
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["Counter: "],
                            &[::core::fmt::ArgumentV1::new_display(&counter.value)],
                        ),
                    );
                    res
                },
            );
        _gtk_label_12.set_margin_all(counter.value.into());
        relm4::RelmContainerExt::container_add(&_gtk_box_14, &_gtk_togglebutton_13);
        _gtk_togglebutton_13.set_label("Counter is even");
        {
            use relm4::WidgetRef;
            #[allow(clippy::needless_borrow)]
            relm4::gtk::prelude::ObjectExt::block_signal(
                _gtk_togglebutton_13.widget_ref(),
                &toggle_handler,
            );
        }
        _gtk_togglebutton_13.set_active(counter.value % 2 == 0);
        {
            use relm4::WidgetRef;
            #[allow(clippy::needless_borrow)]
            relm4::gtk::prelude::ObjectExt::unblock_signal(
                _gtk_togglebutton_13.widget_ref(),
                &toggle_handler,
            );
        }
        relm4::RelmContainerExt::container_add(&_gtk_box_14, &local_label);
        local_label.set_opacity(0.7);
        relm4::RelmContainerExt::container_add(&_gtk_box_14, &local_ref_label);
        local_ref_label.set_opacity(0.7);
        local_ref_label.set_size_request(40, 40);
        let __current_page = "";
        _conditional_widget_3
            .set_visible_child_name(
                if counter.value % 2 == 0 {
                    let __page_active: bool = (__current_page == "0");
                    "0"
                } else if counter.value % 3 == 0 {
                    let __page_active: bool = (__current_page == "1");
                    "1"
                } else {
                    let __page_active: bool = (__current_page == "2");
                    "2"
                },
            );
        let __current_page = "";
        match_stack
            .set_visible_child_name(
                match counter.value {
                    (0..=2) => "0",
                    _ => "1",
                },
            );
        _gtk_window_15.set_title(Some("Another window"));
        _gtk_window_15.set_default_width(300);
        _gtk_window_15.set_default_height(100);
        _gtk_window_15.set_transient_for(Some(&main_window));
        _gtk_window_15.hide();
        _gtk_window_15.set_visible(counter.value == 42);
        relm4::RelmContainerExt::container_add(&_gtk_window_15, &my_label_name);
        my_label_name.set_label("You made it to 42!");
        // ANCHOR: post_init
        let widgets = Self::Widgets {
            main_window,
            _gtk_box_14,
            inc_button,
            _gtk_button_0,
            _gtk_grid_2,
            _gtk_label_1,
            _conditional_widget_3,
            _gtk_label_4,
            _gtk_label_5,
            _gtk_label_6,
            match_stack,
            _gtk_label_7,
            _gtk_label_8,
            _gtk_label_9,
            _gtk_label_builder_10,
            _gtk_label_new_11,
            _gtk_label_12,
            _gtk_togglebutton_13,
            toggle_handler,
            local_label,
            local_ref_label: local_ref_label.clone(),
            _gtk_window_15,
            my_label_name,
            test_field,
        };
        ComponentParts {
            model: counter,
            widgets,
        }
        // ANCHOR_END: post_init
    }
    // ANCHOR_END: init
}
// ANCHOR_END: all
fn main() {
    let app = RelmApp::new("relm4.example.macro_reference");
    app.run::<AppModel>(AppInit { counter: 0 });
}
