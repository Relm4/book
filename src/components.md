# Components

I've already mentioned components several times in the previous chapters. Now we'll finally have a look at them.

In short, components are independent parts of your application that can communicate with each other through messages. They are used in a parent-child model: The main app can have components and each component can have child components that again can have child components. This means that each component has a parent, whereas the main app is at the top of this tree structure and therefore does not have a parent. Also, each component can send and receive messages from both parent and children.

To showcase this, we will create a small application which opens a dialog when it gets closed. The headerbar and the dialog will be implemented as standalone components. The communication to the main application will be done via messages.

![App screenshot dark](img/screenshots/components-dark-1.png)

![App screenshot dark](img/screenshots/components-dark-2.png)

## When to use components

Components are mainly useful for separating parts of the UI into smaller, more manageable parts. They are not necessary but for larger applications, they can be very helpful.

# Example application

Let's write a small example app to see how components can be used in action. For this example, we write parts of an app that can edit images.

> The app we will write in this chapter is also available [here](https://github.com/AaronErhardt/relm4/blob/main/relm4-examples/examples/components.rs). Run `cargo run --example components` from the [example directory](https://github.com/AaronErhardt/relm4/tree/main/relm4-examples) if you want to see the code in action.

## The header bar

Our first component will be a header bar. There are not a lot of advantages for writing this component except for reducing the complexity in other parts of our UI.

The header bar will have three buttons for three modes that our application can have:

+ **View:** View the image.
+ **Edit:** Edit the image.
+ **Export:** Export the image in different formats.

We will not implement the actual functionality, but use placeholders instead to keep things simple.

### The model

Usually you want to store everything that only affects your component in the state of the component. In this case however, there is no state that can be stored in the component, but only state that affects the root component (app). Therefore, we leave the model empty and only send messages to the root component.

```rust,no_run,noplayground
{{#include ../examples/components.rs:header_model }}
```

The message type allows us to switch between the modes.

```rust,no_run,noplayground
{{#include ../examples/components.rs:header_msg }}
```

For components we also need to implement the `Model` trait. The `Components` type is empty here because it refers to child components. We don't have any child components for our header bar so we use a `()`.

```rust,no_run,noplayground
{{#include ../examples/components.rs:header_model_impl }}
```

The update function is rather minimal. If our header bar was more complex, storing state in this component would make sense, but because we just handle a few buttons, we can simply forward messages. For that we can use the `parent_sender`. You can see that the message type of the main application is `AppMsg` and that there's an enum `AppMode`. Both were not introduced yet, but will be explained later. For now, we just need to know that this component will send `SetMode` messages to the app.

```rust,no_run,noplayground
{{#include ../examples/components.rs:header_update }}
```

> We don't use the `_parent_model` argument of the `init_model` in this example. Yet you can use it if you need to access information from the parent model during initialization, for example for passing a resource shared with the component.

### The widgets

There's nothing special about widgets of a component. The only difference to the main app is that the root widget doesn't need to be a `gtk::ApplicationWindow`. Instead, we use a `gtk::HeaderBar` here, but theoretically the root widget doesn't even need to be a widget at all (which can be useful in special cases).

```rust,no_run,noplayground
{{#include ../examples/components.rs:header_widgets }}
```

## The close alert

Like a normal application that's used to edit files, we want to notify the user before accidentally closing the application and discarding all progress. For this &mdash; you might have guessed it already &mdash; we will use another component.

### The model

The state of the dialog only needs to store whether or not it's hidden.

```rust,no_run,noplayground
{{#include ../examples/components.rs:dialog_model }}
```

The message contains three options:

+ Show is used by the parent to display the dialog.
+ Accept is used internally to indicate that the user agreed to close the application.
+ Cancel is used internally to indicate that the user changes his mind and doesn't want to close the application.

```rust,no_run,noplayground
{{#include ../examples/components.rs:dialog_msg }}
```

The update function updates the state of the dialog and sends a close message if the user accepted.

```rust,no_run,noplayground
{{#include ../examples/components.rs:dialog_update }}
```

### The widgets

You've probably seen enough widget implementations by now to know roughly how this should look like, but because we haven't had window components let's have a look at it either way.

```rust,no_run,noplayground
{{#include ../examples/components.rs:dialog_widgets }}
```

Most notably there is the `args!` macro. It allows us to pass values to functions that take more than one argument. The macro would otherwise interpret the comma for a second argument as new property, so we need to use `args!` here.

Also, we set the `set_transient_for` property, which actually uses the main window from the parent widgets. So far `parent_widgets` was an unused argument in our implementations. However in this case, it's neat to have access to the parent widgets. The dialog should set his parent window so that GTK can handle the dialog better. The GTK docs state: "[set_transient_for] allows window managers to e.g. keep the dialog on top of the main window, or center the dialog over the main window". So we definitely want that and conveniently Relm4 gives us the widgets we need from the parents.

## The main app

Now all parts come together to form a single app. You might remember that there was a components type we always set to `()`. Now we actually make use of this type.

### The components

Because each app and each component can have any amount of child components we need to define a struct that stores all of our components.

```rust,no_run,noplayground
{{#include ../examples/components.rs:components }}
```

To do this, just implement a struct with the components wrapped into a `RelmComponent` (which is similar to `RelmApp`). The first generic type of `RelmComponent` is the model of the component and the second one the parent model.

To make this work and to initialize our components, we need to implement the `Components` trait for our struct.

```rust,no_run,noplayground
{{#include ../examples/components.rs:components_impl }}
```

We just need to pass the arguments of the `init_components` function over to the `RelmComponent::new` function and the rest will be handled by Relm4.

### The model

Now we're looking at something familiar again, the model of the main app.

```rust,no_run,noplayground
{{#include ../examples/components.rs:app_model }}
```

The `AppMode` struct stores the modes the application can be in. The `SetMode` message is used by our header bar component to update the state of the main application when someone presses a button in the header bar. The `Close` message is used by the dialog component to indicate that the window should be closed.

And now we finally use the `Components` type of the `Model` trait.

```rust,no_run,noplayground
{{#include ../examples/components.rs:app_model_impl }}
```

The update function of the model is pretty straight forward.

```rust,no_run,noplayground
{{#include ../examples/components.rs:app_update }}
```

You see we can use `components.NAME.send()` to send messages to a child component, similar to the parent_sender we used to send messages in the other direction. Also we return `false` if our dialog component sends the `Close` message to tell Relm4 to close the application.

### The widgets

We're almost done! We only need to define the widgets of the main app.

```rust,no_run,noplayground
{{#include ../examples/components.rs:app_widgets }}
```

The `component!` macro is used to interact with components. We just need to get our header bar component in place. Our dialog component does not need to be attached anywhere because the dialog lives in a separate window.

> Widgets from components are added **after** everything else. Because Relm4 initializes components after their parents we can only add components after the rest is already in place. This means that you sometimes might have to use methods like [`prepend`](https://gtk-rs.org/gtk4-rs/git/docs/gtk4/prelude/trait.BoxExt.html#tymethod.prepend) to keep the right order because with `append` the a component will always be added at the end. Yet, everything else is initialized in the right order.

## Conclusion

You now know most of the secrets that Relm4 offers. Components can be powerful and if they are implemented correctly, they are even reusable across different apps. The relm4-components crate offers several reusable components you can use in your applications. In the following chapters, we'll look at an even simpler component type called worker, how to implement reusable components yourself and how to use components with async code and multiple threads.

## The complete code

Let's review our code in one piece one more time to see how all these parts work together:

```rust,no_run,noplayground
{{#include ../examples/components.rs:all }}
```
