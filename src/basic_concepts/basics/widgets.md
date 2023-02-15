# Widgets

GTK4 provides [widgets](https://docs.gtk.org/gtk4/visual_index.html) as building blocks for your UI, like buttons, input fields or text areas.
They can visualize data and also receive user inputs.
In Relm4, user inputs are usually directly translated into input messages for our components.

It's important to understand that widgets behave similar to [`Rc`](https://doc.rust-lang.org/std/rc/index.html).
Most importantly, this means that:

+ Cloning a widget doesn't create a new instance, but just increases the reference count.
+ Widgets are kept alive automatically. Dropping widgets that are still used somewhere does not destroy them, but just decreases the reference count.
+ Widgets are not thread-safe. Widgets don't implement `Send` and can only be used on the main thread.
