# Introduction

[**If you're already familiar with this chapter, you can skip to the summary**](#summary)

Many applications need to process data in the background.
Yet, at the same time, it's often desired to keep application responsive.

Let's look at a simple example.
Imagine, this would be the update function of one of our components:

```rust,no_run,noplayground
{{#include ../examples/threads_and_async.rs:slow_update }}
```

The `generate_rsa_key()` function takes some time to compute because generating the key is a difficult calculation.
We can treat it as if it was implemented like this:

```rust,no_run,noplayground
{{#include ../examples/threads_and_async.rs:rsa_key }}
```

So what would happen if something emitted the `RunHeavyCalculation` message?
The answer is simple: The application would freeze for 10 seconds.

GTK uses a single threaded runtime and component updates also run on the same runtime.
Blocking our only thread with a heavy computation stops our application from processing user inputs until the computation is complete.

## Commands

There are multiple possibilities to address this common problem. In this chapter we'll have a look at Commands which is a simple yet extremely powerful mechanism which covers most use cases.

Let's say you have an application that fetches data from a website.
This leaves us in a similar situation as before: If we use a synchronous HTTP library in the update function, we will block our main thread and freeze the application until the server responds.
So instead, we're going to use Commands in this example.

Commands are background tasks that can be spawned using a `ComponentSender` or a `FactoryComponentSender`.
They run until they return their result as a `CommandOutput` which in turn is processed again by the component.

First we define our message type, then we can use it as associated `CommandOutput` type in our component.

```rust,no_run,noplayground
{{#include ../examples/threads_and_async.rs:command_msg }}
```

```rust,no_run,noplayground
{{#include ../examples/threads_and_async.rs:command_output_type }}
```

> Note: This works only with the `Component` trait.
> The simplified `SimpleComponent` trait doesn't allow you to use components.

In our update function, we trigger a new command using the `oneshot_command()` method.
This method allows us to spawn a future that will yield exactly one `CommandOutput` message at completion.
From the command, we call an asynchronous function that will handle the web request for us.
Once the future completes, the command returns a `CommandMsg`.

```rust,no_run,noplayground
{{#include ../examples/threads_and_async.rs:async_update }}
```

Now, we can process the `CommandMsg` similar to regular app updates.
The method we use is called `update_cmd()` and is very similar to the regular `update()` function only the message type is `CommandOutput` instead of `Input`.
From here, we can simply assign the result of the web request to our model.

```rust,no_run,noplayground
{{#include ../examples/threads_and_async.rs:update_cmd }}
```

That's it!
It's really as simple as starting a task and processing a message on completion.
You can even use commands for synchronous operations too.

> With the [`command()`](https://relm4.org/docs/next/relm4/prelude/struct.ComponentSender.html#method.command) method we are even more flexible as we can send multiple messages.

### Configuration

Commands actually run on a tokio runtime.
This gives you great compatibility with Rust's async ecosystem and a lot of flexibility at the same time. In Relm you can customize the configuration of the runtime by overwriting [static variables](https://relm4.org/docs/next/relm4/index.html#statics) at the start of your main function.

By default Relm4 uses only one thread for asynchronous background tasks which might not be enough. For example, if you spawn many commands in your application, you can set `RELM_THREADS` to 4 to increase the thread count by 3 additional threads.

> Note: Setting the static variables must be done early.
> As soon as the runtime is initialized (which happens when it's accessed for the first time), the values cannot be changes anymore.

## Workers

Workers are basically just components without widgets.
However, since they don't have widgets, workers can be sent to other threads, which also makes it possible run their update function on another thread.
This means, that executing the slow `generate_rsa_key()` function from the first section won't freeze your application.

To create a worker, you just need to implement the [`Worker`](https://relm4.org/docs/next/relm4/worker/trait.Worker.html) trait and call [`detach_worker`](https://relm4.org/docs/next/relm4/component/struct.ComponentBuilder.html#method.detach_worker) from the `ComponentBuilder`. This will execute the worker in a new thread.

You might wonder why workers even exist if we already have commands.
Actually, workers have some unique properties: They can only run one task at a time and store their state like a regular component.

## Local futures

Both commands and workers run on different threads, so Rust's guarantees require the involved types to implement `Send`.
This can sometimes be problematic, for example when it comes to widgets.

Fortunately, the [`spawn_local`](https://relm4.org/docs/next/relm4/fn.spawn_local.html) function allows us to spawn local futures, which don't require `Send` because they run on the main thread.
This works because GTK uses an event loop from GLib to handle asynchronous events, which also allows you to execute futures.

The only drawback of this solution is that not all async libraries are fully compatible with the GLib executor, since they must use Tokyo.

# Summary

When to use ...

+ **commands:**
  + Run asynchronous tasks on a runtime in the background
  + Run CPU intensive synchronous tasks on a runtime in the background
  + Run many background tasks in parallel
  + Drop the background task as soon as the component is destroyed

+ **workers:**
  + Handle IO-bound or CPU-intensive tasks **one** at the time on a different thread
  + The update function should be executed in another thread
  + You need a model to store state for processing messages
  
+ **spawn_local:**
  + Your future or message type don't implement `Send`
