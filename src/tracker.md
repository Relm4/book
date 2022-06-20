# Tracker

A tracker in this context just means a data type that's able to track changes to itself. For example, if we increment the counter of the model we used for our first app, the model could tell us later that the counter changed during the last update function.

Relm4 does not promote any implementation of a tracker. You're free to use any implementation you like, you can even implement a tracker yourself. In this example however, we'll use the [`tracker` crate](https://docs.rs/tracker/latest/tracker/) that provides a simple macro that implements a tracker for us automatically.

Using this technique, we will implement a small program which displays two randomly picked icons that are controlled by two buttons:

![App screenshot](img/screenshots/tracker-dark-1.png)

When pressing a button, the icon above it will change. The background of the application will become green when the two icons are identical:

![App screenshot with with equal icons](img/screenshots/tracker-dark-2.png)


## The tracker crate

The [`tracker::track`](https://docs.rs/tracker/latest/tracker/attr.track.html) macro implements the following methods for your struct fields:

+ `get_#field_name()`  
  Get an immutable reference to your field.

+ `get_mut_#field_name()`  
  Get a mutable reference to your field. Assumes the field will be modified and marks it as changed.

+ `set_#field_name(value)`  
  Get a mutable reference to your field. Marks the field as changed only if the new value isn't equal with the previous value.

+ `update_#field_name(fn)`  
  Update your mutable field with a function or a closure. Assumes the field will be modified and marks it as changed.

To check for changes you can call `var_name.changed(StructName::field_name())` and it will return a bool indication whether the field was updated.

To reset all previous changes, you can call `var_name.reset()`.

## Example

First we have to add the tracker library to `Cargo.toml`:
```toml
tracker = "0.1"
```

Now let's have a look at a small example.

```rust,no_run,noplayground
#[tracker::track]
struct Test {
    x: u8,
    y: u64,
}

fn main() {
    let mut t = Test {
        x: 0,
        y: 0,
        // the macro generates a new variable called
        // "tracker" that stores the changes
        tracker: 0,
    };

    t.set_x(42);
    // let's check whether the change was detected
    assert!(t.changed(Test::x()));

    // reset t so we don't track old changes
    t.reset();

    t.set_x(42);
    // same value, so no change
    assert!(!t.changed(Test::x()));
}
```

So in short, the `tracker::track` macro provides different getters and setters that will mark struct fields as changed. You also get a method that checks for changes and a method to reset the changes.

# Using trackers in Relm4 apps

Let's build a simple app that shows two random icons and allows the user to set each of them to a new random icon. As a bonus, we want to show a fancy background color if both icons are the same.

> The app we will write in this chapter is also available [here](https://github.com/AaronErhardt/relm4/blob/main/relm4-examples/examples/tracker.rs). Run `cargo run --example tracker` from the [example directory](https://github.com/AaronErhardt/relm4/tree/main/relm4-examples) if you want to see the code in action.

## The icons

Before we can select random icons, we need to quickly implement a function that will return us random image names that are available in the default GTK icon theme.

```rust,no_run,noplayground
{{#include ../examples/tracker.rs:icons }}
```

## The model

For our model we only need to store the two icon names and whether both of them are identical.

```rust,no_run,noplayground
{{#include ../examples/tracker.rs:model }}
```

The message type is also pretty simple: we just want to update one of the icons.

```rust,no_run,noplayground
{{#include ../examples/tracker.rs:msg }}
```

There are a few notable things for the `Component`'s `update` implementation.
First, we call `self.reset()` at the top of the function body. This ensures that the tracker will be reset so we don't track old changes.

Also, we use setters instead of assignments because we want to track these changes. Yet, you could still use the assignment operator if you want to apply changes without notifying the tracker.

```rust,no_run,noplayground
{{#include ../examples/tracker.rs:update }}
```

## The view

Now we reached the interesting part of the code where we can actually make use of the tracker. Let's have a look at the complete `view!` macro invocation:

```rust,no_run,noplayground
{{#include ../examples/tracker.rs:view }}
```

The overall UI is pretty simple: A window that contains a box. This box has two boxes itself for showing the two icons and the two buttons to update those icons.

We also added some additional code in `init` that runs before the view is constructed. In our case, we want to add [custom CSS](https://docs.gtk.org/gtk4/css-properties.html) that sets the background color for elements with class name "identical".

```rust,no_run,noplayground
{{#include ../examples/tracker.rs:post_init }}
```


### The `#[track]` attribute

The `#[track]` attribute is applied to method invocations in our view code. It allows us to add a condition to the update: if the condition is true, the method will be called, otherwise, it will be skipped. The attribute syntax looks like this:

```rust,no_run,noplayground
#[track = "<boolean expression>"]
```

Let's have a look at its first appearance:

```rust,no_run,noplayground
{{#include ../examples/tracker.rs:track1 }}
```

The [`set_class_active`](https://aaronerhardt.github.io/docs/relm4/relm4/util/widget_plus/trait.WidgetPlus.html#tymethod.set_class_active) method is used to either activate or disable a CSS class. It takes two parameters, the first is the class itself and the second is a boolean which specifies if the class should be added (`true`) or removed (`false`).

The value of the `#[track]` attribute is parsed as a boolean expression. This expression will be used  as a condition to check whether something has changed. If this condition is `true`, the `set_class_active` method will be called with the parameters it guards.

The macro expansion for method calls annotated with the `#[track]` attribute look roughly like this:

```rust,no_run,noplayground
if model.changed(AppModel::identical()) {
    self.main_window.set_class_active("identical", model.identical);
}
```

That's all. It's pretty simple, actually. We just use a condition that allows us to update our widgets only when needed.

The second `#[track]` attribute works similarly:

```rust,no_run,noplayground
{{#include ../examples/tracker.rs:track2 }}
```

> Since the `#[track]` attribute parses expressions, you can use the following syntax to debug your trackers:
>
> `#[track = "{ println!("Update widget"); argument }"]`

## Initializing the model

There's one last thing to point out. When initializing our model, we need to initialize the `tracker` field as well. The initial value doesn't really matter because we call `reset()` in the update function anyway, but usually `0` is used.

```rust,no_run,noplayground
{{#include ../examples/tracker.rs:model_init}}
```

## The complete code

Let's review our code in one piece one more time to see how all these parts work together:

```rust,no_run,noplayground
{{#include ../examples/tracker.rs:all }}
```
