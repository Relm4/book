# Efficient UI updates

Relm4 follows the Elm programming model which means that data and widgets are separated. At first glance this might cause a problem. Larger applications need to efficiently update their widgets because rebuilding the whole UI for every update is not an option. But since data and widgets are separated, how do we know which UI elements need to be updated?

Let's have a look at the following example: Imagine you have an app with 1000 counters and you only increment the first counter. The model receives the increment message for the first counter and increments it. Now the view function gets the updated model with 1000 counters and… well, has no idea what has changed! So instead of one UI update we need to do 1000 because we don't know which of our counters was modified.

There are two concepts in Relm4 to avoid unnecessary UI updates:

+ **Trackers** identify modifications of fields in `struct`s to only trigger updates to the affected UI elements.
+ **Factories** track changes in data structures similar to [`std::collections`](https://doc.rust-lang.org/std/collections/index.html) in order to perform also minimal UI updates. They are used to generate multiple similar widgets, e.g. a row of buttons, from a data collection.

Both concepts are explained in the following chapters.
