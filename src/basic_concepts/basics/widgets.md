# Widgets

GTK 4 allows [widgets](https://docs.gtk.org/gtk4/visual_index.html) to receive input and send output messages. Widgets are simply parts of an UI like buttons, input fields or text areas. To be able to update the widgets in our program, we can put them all into a `struct`.

```rust,no_run,noplayground
{{#include ../../../examples/simple_manual.rs:widgets }}
```

This struct will allow us to handle UI changes when we update the model. For example, in a to-do app, we could strikethrough the label when a task is marked as completed.