# Efficient UI updates

Relm4 follows the Elm programming model which means that data and widgets are separated. At first glance this might cause a problem. Larger applications need to efficiently update their widgets because rebuilding the whole UI for every update is not an option. But since data and widgets are separated, how do we know which UI elements need to be updated?

Let's have a look at an imaginary example to visualize this problem. Imagine you have an app with 1000 counters and you only increment the first counter. The model receives the increment message for the first counter and increments it. Now the view function gets the updated model with 1000 counters and... well, has no idea what changed! So instead of one UI update we need to do 1000 because we don't know which of our counters was modified.

There are two concepts in Relm4 to avoid unnecessary UI updates

+ Trackers: keep track of which struct fields were modified and only update the UI if they were modified.
+ Factories: store data in a special data structures similar to the data structures in [`std::collections`](https://doc.rust-lang.org/std/collections/index.html) that will keep track of changes and will only apply minimal UI updates.

Both concepts are explained in the following chapters.
