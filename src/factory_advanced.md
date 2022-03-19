# Advanced factories

In this chapter we will build an even more advanced UI for modifying the available counters:

![App screenshot dark](img/screenshots/factory-advanced-dark.png)

Additionally, certain counters can now be removed or inserted above or below an existing counter.

> If you're not familiar with the `Rc` type of the standard library, have a look at [this](https://doc.rust-lang.org/std/rc/index.html).

The `FactoryVec` we used in the previous chapter is sufficient for simple applications where elements only need to be added and removed from the back. Yet a common use case would be to add elements before another one or to remove a specific element. That introduces additional complexity that needs to be taken care of but fortunately this is mostly handled by Relm4.

To show this, we'll create a similar counter app to the one of the previous chapter, but this time on **steroids**: we'll add functionality to add counters before and after a specific counter and to remove a certain counter. To get the required flexibility, we'll use the `FactoryVecDeque` type instead of a `FactoryVec`.

> The app we will write in this chapter is also available [here](https://github.com/AaronErhardt/relm4/blob/main/relm4-examples/examples/factory_advanced.rs). Run `cargo run --example factory_advanced` from the [example directory](https://github.com/AaronErhardt/relm4/tree/main/relm4-examples) if you want to see the code in action.

## Indices

The indices of a `FactoryVec` were just numbers of type `usize`. That's great unless elements can move and change their index. This tragedy starts when we, for example, add an element to the front: the new element now has index `0`, the element that had index `0` before now has index `1` and so on. Adding one element will shift the indices of all following elements. If we naively create a signal handler similar to the previous chapter were we just copied the index at start and moved it into the closure, we will quickly end up with quite wrong or even out-of-bounds indices as elements are added and removed at arbitrary positions.

One solution would be to recreate all signal handlers with the updated indices once an element's index has been changed. However, that's complicated because you need to remove the old signal handlers first and therefore you have to store all signal handler IDs.

The solution Relm4 chose was dynamic indices. These indices are updated automatically to always point at the same element.



### The message type

```rust,no_run,noplayground
{{#include ../examples/factory_advanced.rs:msg }}
```

As you can see, we use a lot of `MsgIndex` aka `Weak<DynamicIndex>`. This allows us to always hold a reference to the dynamic index value.

The reason we use weak pointers here is that we don’t want to hold references to invalid indices. We don’t know if our messages are handled immediately or queued up instead, so the data the index was pointing at could have been replaced by a new data in the meantime. Usually this happens so rarely that this can be ignored, but with the weak pointers we guarantee that the indices are not kept alive in the message queue and we will never use a stale index.

### The model

The model is very similar to the previous chapter. The only difference is that we use `FactoryVecDeque` as a data structure now.

```rust,no_run,noplayground
{{#include ../examples/factory_advanced.rs:model }}
```

### The update function

The update function now handles quite a lot of events. We want to

+ Add elements at the start
+ Remove elements from the back
+ Decrement (count) a counter at a specific index
+ Insert a new counter before another counter
+ Insert a new counter after another counter

```rust,no_run,noplayground
{{#include ../examples/factory_advanced.rs:app_update }}
```

To get the current index value from the dynamic index, we simply call `index.current_index()`.

## The factory implementation

The factory implementation is mostly the same, so we'll just have a look at what has changed.

### The widgets type

Because we have four actions per counter now, we also need an additional box to store these buttons.
To be able to provide the root widget via the `get_root` function we need to store the box in the widgets type.

```rust,no_run,noplayground
{{#include ../examples/factory_advanced.rs:factory_widgets }}
```

### The generate function

For the generate function, we need to first generate the new buttons and the box.

```rust,no_run,noplayground
{{#include ../examples/factory_advanced.rs:generate_start }}
```

Then we need to place the buttons inside of the box.

```rust,no_run,noplayground
{{#include ../examples/factory_advanced.rs:append }}
```

Now we can connect the messages. We always send a weak pointer of our dynamic index.

```rust,no_run,noplayground
{{#include ../examples/factory_advanced.rs:connect }}
```

And that's it! All the other complex operations that keep track of changes are implemented in Relm4 already, we just need to use dynamic indices to make out program work :)

## The complete code

Let's review our code in one piece one more time to see how all these parts work together:

> Unlike the example in the previous chapter, the following code does not use the widget macro from relm4-macros but implements the `Widgets` trait manually. Yet, the generated code from the macro and the manual code should be almost identical.

```rust,no_run,noplayground
{{#include ../examples/factory_advanced.rs:all }}
```
