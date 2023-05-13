# Messages 

To help the computer understand what we want to tell it, we first translate user interactions into messages.

In Relm4, a message can be any data type, but most often, an `enum` is used. 

```rust,no_run,noplayground
{{#include ../../examples/simple_manual.rs:msg }}
```

Computers are capable of both sending and receiving messages and similarly, components in Relm4 can send and receive messages.

This is accomplished by having two types of messages: `Input` and `Output`.
