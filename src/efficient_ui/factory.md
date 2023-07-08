# Factory

Factories define how to generate widgets from data collections. 
GTK also has factories, yet Relm4 uses its own factory implementation which is much easier to use in regular Rust code.

![App screenshot dark](../img/screenshots/factory-dark.png)

This app will have a dynamic number of counters.
Also, the counters can be moved up and down by the user.

## Factories in Relm4

Factories allow you to visualize data in a natural way.
If you wanted to store a set of counter values in regular Rust code, you'd probably use `Vec<u8>`.
However, you can't simply generate widgets from a `Vec`.

This is where factories are really useful.
Custom collection types like `FactoryVecDeque` allow you to work with collections of data almost as comfortable as if they were stored in a `Vec`.
At the same time, factories allow you to automatically visualize the data with widgets.
Additionally, factories are very efficient by reducing the amount of UI updates to a minimum.

> The app we will write in this chapter is also available [here](https://github.com/Relm4/Relm4/blob/main/examples/factory.rs).
> Run `cargo run --example factory` from the [example directory](https://github.com/Relm4/Relm4/tree/main/examples) if you want to see the code in action.

### The model

First, we define the struct `Counter` that just stores the value of a single counter.
Later, we will use a `FactoryVecDeque` to store our counters.


```rust,no_run,noplayground
{{#include ../../examples/factory.rs:factory_model }}
```

### The input message type

Each counter should be able to increment and decrement.

```rust,no_run,noplayground
{{#include ../../examples/factory.rs:factory_input }}
```

### The output message type

A neat feature of factories is that each element can easily forward their output messages to the input of their parent component.
For example, this is necessary for modifications that require access to the whole `FactoryVecDeque`, like moving an element to a new position.
Therefore, these actions are covered by the output type.

The actions we want to perform "from outside" are

+ Move a counter up
+ Move a counter down
+ Move a counter to the first position

Accordingly, our message type looks like this:

```rust,no_run,noplayground
{{#include ../../examples/factory.rs:factory_output }}
```

You might wonder why `DynamicIndex` is used here.
First, the parent component needs to know which element should be moved, which is defined by the index.
Further, elements can move in the `FactoryVecDeque`.
If we used `usize` as index instead, it could happen that the index points to another element by the time it is processed.

### The factory implementation

Factories use the `FactoryComponent` trait which is very similar to regular components with some minor adjustments.
For example, `FactoryComponent` needs the `#[relm4::factory]` attribute macro and a few more associated types in the trait implementation.

```rust,no_run,noplayground
{{#include ../../examples/factory.rs:factory_impl_start }}
```

Let's look at the associated types one by one:

+ **Init**: The data required to initialize `Counter`, in this case the initial counter value.
+ **Input**: The input message type.
+ **Output**: The output message type.
+ **CommandOutput**: The command output message type, we don't need it here.
+ **Widgets**: The name of the struct that stores out widgets, it will be created by the macro.
+ **ParentInput**: The input message type of the parent component.
+ **ParentWidget**: The container widget used to store the widgets of the factory, for example `gtk::Box`.

### Creating the widget

The widget creation works as usual with our trusty `view` macro.
The only difference is that we use `self` to refer to the model due to differences in the `FactoryComponent` trait.

```rust,no_run,noplayground
{{#include ../../examples/factory.rs:factory_view }}
```

### Initializing the model

`FactoryComponent` has separate functions for initializing the model and the widgets. 
This means, that we are a bit less flexible, but don't need `view_output!()` here.
Also, we just need to implement the `init_model` function because `init_widgets` is already implemented by the macro.

```rust,no_run,noplayground
{{#include ../../examples/factory.rs:factory_init_model }}
```

### Forwarding messages

Factories can implement the `forward_to_parent` method to send messages to their parent component.

If `Some` is returned, a message is forwarded.
If `None` is returned, nothing happens.

```rust,no_run,noplayground
{{#include ../../examples/factory.rs:output_to_parent }}
```

## The main component

Now, we have implemented the `FactoryComponent` type for the elements in our factory.
The only thing left to do is to write our main component to complete our app.

### The component types

For the main component we implement the familiar `SimpleComponent` trait.
First we define the model and the input message type and then start the trait implementation.

```rust,no_run,noplayground
{{#include ../../examples/factory.rs:main_types }}
```

### Initializing the factory

We skip the `view` macro for a moment and look at the `init` method.
You see that we are initializing the `FactoryVecDeque` with a container widget.
This widget will store all the widgets created by the factory.

We also pass an input sender so the `forward_to_parent` method we defined earlier can send input messages to our main component.

The last trick we have up our sleeves is to define a local variable `counter_box` that is a reference to the container widget of our factory.
We'll use it in the `view` macro in the next section.

```rust,no_run,noplayground
{{#include ../../examples/factory.rs:main_init }}
```

### Initializing the widgets

The familiar `view` macro comes into play again.
Most things should look familiar, but this time we use a `#[local_ref]` attribute for the last widget to use the local variable we defined in the previous section.
This trick allows us to initialize the model with its `FactoryVecDeque` before the widgets, which is more convenient in most cases.

```rust,no_run,noplayground
{{#include ../../examples/factory.rs:main_view }}
```

### The main update function

This time the main update function has actually quite a bit to do.
The code should be quite readable if you worked with `Vec` or `VecDeque` before.

One thing stands out though: We see a lot of calls to `guard()`.
In fact, all mutating methods of `FactoryVecDeque` need an RAII-guard.
This is similar to a `MutexGuard` you get from locking a mutex.

The reason for this is simple.
As long as the guard is alive, we can perform multiple operations.
Once we're done, we just drop the guard (or rather leave the current scope) and this will cause the factory to update its widgets automatically.
The neat thing: You can never forget to render changes, and the update algorithm can optimize widget updates for efficiency.

```rust,no_run,noplayground
{{#include ../../examples/factory.rs:main_update }}
```

### The main function

Awesome, we almost made it!

We only need to define the main function to run our application.

```rust,no_run,noplayground
{{#include ../../examples/factory.rs:main }}
```

## The complete code

Let's review our code in one piece one more time to see how all these parts work together:

```rust,no_run,noplayground
{{#include ../../examples/factory.rs:all }}
```
