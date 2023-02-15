# Async components and factories

Asynchronous components and factories are almost identical compared to regular components and factories.
The only major difference is that they have asynchronous `init`, `update` and `update_cmd` methods.
This allows you to `await` almost everywhere from within the component.

> The app we will write in this chapter is also available [here](https://github.com/Relm4/Relm4/blob/main/examples/simple_async.rs). Run `cargo run --example simple_async` from the [example directory](https://github.com/Relm4/Relm4/tree/macro-0.5/examples) if you want to see the code in action.

Because Rust doesn't support async traits yet, we need macros to add support for this feature.
To tell the `component` macro that we're using an async trait, we pass the `async` parameter to it.
The `component` macro will then utilize the `async_trait` crate behind the scenes to make everything work.
Also, we need to use `AsyncComponent` instead of `Component` as trait.
Apart from that, the first section is identical.

> Similarly, the `factory` macro needs the `async` parameter for async factories and the trait changes from `FactoryComponent` to `AsyncFactoryComponent`.

```rust,no_run,noplayground
{{#include ../examples/async.rs:async_component_start }}
```

Most functions of async component and factory traits are asynchronous, which allows us to `await` on futures within those functions.
Apart from that, only a couple of types need to be adjusted for the async versions of the traits, for example `AsyncComponentSender` and `AsyncComponentParts`.

```rust,no_run,noplayground
{{#include ../examples/async.rs:init }}
```

Awaiting in the init function allows us to perform a late initialization.
Depending on how you implement the init function, it might take a long time to complete.
Not showing anything in this case can look very odd.
Therefore, Relm4 allows you to specify widgets that will be displayed while your async component is initialized.

> If your init function doesn't await or completes quickly, you don't need to implement `init_loading_widgets`.

```rust,no_run,noplayground
{{#include ../examples/async.rs:init_loading_widgets }}
```

In this case, we do some basic initialization of our root widget upfront and also add a `Spinner` for a nice loading animation.
As soon as the `init` function returns, the temporary spinner will be removed automatically and the widgets from the `view!` macro will be inserted instead.

Finally, the `update` function completes the trait implementation.
Notably, awaiting slow futures will block the processing of further messages.
In other words, the update function can only process one message afters the other.
Because we use async however, this only affects each async component individually and all other components won't be affected.
If you want to process multiple messages at the same time, you should consider using commands.

```rust,no_run,noplayground
{{#include ../examples/async.rs:update }}
```

## The complete code

```rust,no_run,noplayground
{{#include ../examples/async.rs:all }}
```
