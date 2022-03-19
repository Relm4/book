# Macro expansion

To better understand the widget macro, we will have a look at how the different parts of the widget macro are translated into real Rust code (aka the macro expansion). Therefore, we will write a small app that uses as many widget macro features as possible.

> The app we will write in this chapter is also available [here](https://github.com/AaronErhardt/relm4/blob/main/relm4-examples/examples/macro_test.rs). Run `cargo run --example macro_test` from the [example directory](https://github.com/AaronErhardt/relm4/tree/main/relm4-examples) if you want to see the code in action.

## The boilerplate

First, let's have a look at the parts of the code that are later used by the macro.

### The model

The model stores a counter, several class names and a decrement field that will indicate if the counter was last decremented or not. This will be used later in a tracker that only updates when the user decrements the counter.

```rust,no_run,noplayground
{{#include ../examples/macro_test.rs:model }}
```

### The message type

The message type is the same as in our first app.

```rust,no_run,noplayground
{{#include ../examples/macro_test.rs:msg }}
```

### The update function

The update function is very simple, too. The only difference is that we set the decrement field to `true` if the `Decrement` message was sent.

```rust,no_run,noplayground
{{#include ../examples/macro_test.rs:app_update }}
```

### The component

We will use a minimal button component that just has a button as widget to showcase the `component!` macro later.

```rust,no_run,noplayground
{{#include ../examples/macro_test.rs:button_comp }}
```

### A custom widget function

Also, we add a small function that simply returns a `gtk::Label`.

```rust,no_run,noplayground
{{#include ../examples/macro_test.rs:new_label }}
```

## The macro

Let's have a look at the whole macro before we will break it down into smaller parts. If you're unfamiliar with the macro syntax, have a look at the previous chapter.

```rust,no_run,noplayground
{{#include ../examples/macro_test.rs:widgets }}
```

## The expansion

The macro expansion is not supposed to be readable, so the code might look a bit ugly.

### The widgets struct

The fields of the widgets struct cover all widgets we created, plus the additional fields we added manually. Names fields like `main_window` and `inc_button` keep their names. Unnamed fields will get automatically generated names with an unique ID. You should never refer to unnamed fields in your code because their names might change. At the end, we can find the additional field called `test_field` that we added manually.

```rust,no_run,noplayground
{{#include ../examples/macro_expansion.rs:widgets_struct }}
```

### The `Widgets` trait implementation

The next thing the macro does is generating the `Widgets` trait implementation block.

The start of the implementation block is very similar to the implementation block we use in the macro. Most notably, the `Root` type is automatically inserted. All attributes and comments you add to the widget macro before the `impl` block should be kept as well.

```rust,no_run,noplayground
{{#include ../examples/macro_expansion.rs:widgets_impl }}
```

#### Pre-initialization

At the start of the view initialization, we find &mdash; to no surprise &mdash; the code of the `pre_init()` function.

```rust,no_run,noplayground
{{#include ../examples/macro_expansion.rs:pre_init }}
```

It's exactly the the code of the `pre_init()` function.

```rust,no_run,noplayground
{{#include ../examples/macro_test.rs:pre_init }}
```

#### Widget initialization

The macro now initializes all widgets. Widgets that were defined by their type are initialized with the [`relm4::util::default_widgets::DefaultWidget`](https://aaronerhardt.github.io/docs/relm4/relm4/util/default_widgets/trait.DefaultWidget.html) trait that basically calls `Widget::builder().build()` to initialize a widget with default configuration. Obviously, that only works for widgets that support this builder pattern.

We also see `gtk::Button::new()` and `new_label()` used to initialize widgets. These widgets were explicitly initialized with a [function](https://aaronerhardt.github.io/relm4-book/book/widget_macro_reference.html#functions).

```rust,no_run,noplayground
{{#include ../examples/macro_expansion.rs:widget_init }}
```

#### Assigning properties

Assigning properties looks pretty normal as well.

```rust,no_run,noplayground
{{#include ../examples/macro_expansion.rs:property_assign }}
```

At the start, we find the code for the assignment from the macro that uses a trait function.

```rust,no_run,noplayground
{{#include ../examples/macro_test.rs:trait_fn_assign }}
```

In the middle we have the optional assign, that uses an `if let` statement to only assign properties that match `Some(data)`. In the macro we marked this line with a `?`.

```rust,no_run,noplayground
{{#include ../examples/macro_test.rs:optional_assign }}
```

At the end we have our iterator from the macro. 

```rust,no_run,noplayground
{{#include ../examples/macro_test.rs:iterative_assign }}
```

> There are some properties missing here because I only showed the relevant section for the purpose of this book.

#### Events

Now the macro generates the code for connecting events.

```rust,no_run,noplayground
{{#include ../examples/macro_expansion.rs:connect }}
```

The code looks very similar to what we wrote in the macro.

```rust,no_run,noplayground
{{#include ../examples/macro_test.rs:connect }}
```

Most notably, the sender we put in the parenthesis is cloned as we requested.

#### Post-initialization

At the end, we find the code of our `post_init()` function.

```rust,no_run,noplayground
{{#include ../examples/macro_expansion.rs:post_init }}
```

Again, the code is exactly the same.

```rust,no_run,noplayground
{{#include ../examples/macro_test.rs:post_init }}
```

#### Return

At the end, we return the widgets struct with all initialized widgets.

```rust,no_run,noplayground
{{#include ../examples/macro_expansion.rs:return }}
```

#### Assigning widgets and components

To keep every widget in order, all widgets are assigned in `connect_components` function. In the first stable version of Relm4 (0.1.0), regular widgets were already assigned in the `init_view` function. This caused problems with the ordering of elements because components were added after all other widgets were already in place. For Relm4 0.2 this behavior was changed so that all widgets are now added at the same place so that components keep their correct order.

```rust,no_run,noplayground
{{#include ../examples/macro_expansion.rs:connect_components }}
```

At the beginning, we find the code for the `set_child` property we used in the macro.

```rust,no_run,noplayground
{{#include ../examples/macro_test.rs:set_child_widget }}
```

In the macro we used the nested `component!` macro to add a component to our UI. This component can now be found in the last line of the `connect_components` function.

```rust,no_run,noplayground
{{#include ../examples/macro_test.rs:component }}
```

#### Root widget

The macro also implements the `root_widget` function that returns the outermost widget that is also the first we use in the `view!` macro.

```rust,no_run,noplayground
{{#include ../examples/macro_expansion.rs:root_widget }}
```

#### Manual UI updates

The last step of the macro is to generate the update logic with the `view` function. At the start of this function, we can find the code from the `manual_view()` function of the macro.

```rust,no_run,noplayground
{{#include ../examples/macro_expansion.rs:manual_view }}
```

Just like with `pre_init()` and `post_init()` the code is exactly the same, too.

```rust,no_run,noplayground
{{#include ../examples/macro_test.rs:manual_view }}
```

#### Generated UI updates

After the manually defined update logic, the macro generates its own code.

```rust,no_run,noplayground
{{#include ../examples/macro_expansion.rs:macro_view }}
```

The first update comes from the nested `watch!` macro and is unconditional.

```rust,no_run,noplayground
{{#include ../examples/macro_test.rs:watch }}
```

The second update rule sits behind an `if` statement because it comes from the nested `track!` macro. In this case, the condition for the tracker is simply the `model.decrement` field.

```rust,no_run,noplayground
{{#include ../examples/macro_test.rs:track }}
```

## Conclusion

Congrats for making it this far 🎉! You're now a real expert of Relm4!

As you have seen, the macro is nothing magical. It simply works with the information you give to it.

## The whole macro expansion

If you want to look at the whole macro expansion at once, here it is.

```rust,no_run,noplayground
{{#include ../examples/macro_expansion.rs:all }}
```
