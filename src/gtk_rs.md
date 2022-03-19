# gtk-rs overview

So far, we only discussed which features Relm4 provides. Yet, Relm4 is based on GTK, which itself has many useful features. Let’s have a look at it!

> This is just an overview. I’ve linked the relevant sections of the [gtk-rs book](https://gtk-rs.org/gtk4-rs/git/book/) but if you want to get familiar with all the features, I recommend reading the book from the start.

## GObjects

GTK is an object-oriented framework that uses the GObject library to implement objects. GObjects have some really useful features that we will discuss in the following sections.

### Subclassing

Like many other OOP frameworks or languages, GObjects can inherit from other GObjects. This is called subclassing. In the case of GTK, that’s really helpful because it allows us to create custom widgets. 

For example, you could use subclassing to create your own button widget that acts as a counter. Or you can create a custom application window that better suits your application.

**Read more about subclassing in the [gtk-rs book](https://gtk-rs.org/gtk4-rs/git/book/gobject_subclassing.html)**.

### Properties

Each GObject can have properties that work similar to the fields of a structure in Rust. You can set them and you can read (get) them. But one thing that's particularly cool is that properties can be bound to other properties.

For example, you could bind the "visible" property of a widget to the "active" property of a `gtk::ToggleButton`. This would allow you to show or hide the widget using the toggle button and the best part is, that it's done fully automatically!

**Read more about properties in the [gtk-rs book](https://gtk-rs.org/gtk4-rs/git/book/gobject_properties.html)**.

### Signals

GObjects can not only have properties but also signals. Actually, we've been using signals all the time, for example, by using the `connect_clicked` method on a button. This method simply adds an event handler function for the "click" signal.

You can create your own signals in custom widgets. You can also use [emit](https://gtk-rs.org/gtk-rs-core/git/docs/glib/object/trait.ObjectExt.html#tymethod.emit) to emit signals on you widgets manually.

**Read more about signals in the [gtk-rs book](https://gtk-rs.org/gtk4-rs/git/book/gobject_signals.html)**.

## Settings

Most applications need to store settings at some point. GTK makes that pretty simple. You can use `gtk::Settings` to store your settings and keep them stored after your app has been closed.

**Read more about settings in the [gtk-rs book](https://gtk-rs.org/gtk4-rs/git/book/settings.html)**.

## Lists

Relm4 has factories for generating widgets from collections of data. GTK has a similar mechanism that should be used for large list. Because GTK knows which widgets of a list are actually shown it can optimize the rendering and memory usage a lot better.

**Read more about lists in the [gtk-rs book](https://gtk-rs.org/gtk4-rs/git/book/lists.html)**.

## Interface builder

Relm4 leaves it up to you how to create you UI. You can do it manually like in our first app, you can do with the widget macro or you can use the interface builder from GTK.

With the interface builder, you can use a XML file to specify your widgets and properties.

**Read more about the interface builder in the [gtk-rs book](https://gtk-rs.org/gtk4-rs/git/book/interface_builder.html)**.