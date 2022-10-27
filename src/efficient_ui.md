# Efficient UI updates

Relm4 follows the Elm programming model which keeps the business logic and its data separated to the widgets. At the first glance you might think this causes problems, but larger applications need this decoupling as it enables them to efficiently modify their UI and business logic with keeping the needed changes local. Rebuilding the whole application with every change of the UI or logic is way too much effort. But since data and UI are separated, how do we know which UI elements need to be updated?

Let's have a look at the following example: Imagine you have an app with 1000 counters and you only increment the first counter. The model receives the increment message for the first counter and increments it. Now the view function gets the updated model with 1000 counters and… well, has no idea what has changed! So instead of one UI update we need to do 1000 because we don't know which of our counters was modified.

There are two concepts in Relm4 to avoid unnecessary UI updates:

+ **Trackers** identify modifications of fields in `struct`s to only trigger the update the relevant UI elements.
+ **Factories** track changes in [`std::collections`](https://doc.rust-lang.org/std/collections/index.html) like data structures to perform also minimal UI updates.

Both concepts are explained in the following chapters.
