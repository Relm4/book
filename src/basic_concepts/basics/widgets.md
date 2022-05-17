# Widgets

GTK 4 offers the computer [widgets](https://docs.gtk.org/gtk4/visual_index.html) that allow it to take input and respond. Widgets are simply parts of an UI like buttons, input fields or text areas. To be able to update the widgets in our program, we can put them all into a `struct`.

```rust,no_run,noplayground
{{#include ../../../examples/simple_manual.rs:widgets }}
```

This struct will allow us to handle UI changes when we update the model, for example, in a to-do app, we could strikethrough the label when a task is marked as completed.