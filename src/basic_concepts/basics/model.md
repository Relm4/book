# Model

Like a person, a computer needs a brain to be functional. It needs to process our messages and remember the results.

Relm4 uses the term model as a data type that represents the application state, the memory of your application. 

For example, to store a counter value, we can store a `u8` in our model:

```rust,no_run,noplayground
{{#include ../../../examples/simple_manual.rs:model }}
```
