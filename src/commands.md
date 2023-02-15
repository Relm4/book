# Commands

In this chapter, we'll have a look at commands, which are a simple yet extremely powerful mechanism to offload both CPU-bound and I/O-bound tasks to a separate runtime.

Commands are background tasks that can be spawned using a `ComponentSender` or `FactorySender`.
They run until they return their result as a `CommandOutput` message that will be processed by the component.

First, we define our message type so we can use it for the associated `CommandOutput` type in our component.

```rust,no_run,noplayground
{{#include ../examples/commands.rs:command_msg }}
```

```rust,no_run,noplayground
{{#include ../examples/commands.rs:command_output_type }}
```

> Note: This only works with the `Component` trait.
> The simplified `SimpleComponent` trait doesn't support commands.

In our update function, we start a new command using the `oneshot_command()` method.
This method allows us to spawn a future that will yield exactly one `CommandOutput` message at completion.
From the command, we call an asynchronous function that will handle the web request for us.
Once the future completes, the command returns a `CommandMsg`.

```rust,no_run,noplayground
{{#include ../examples/commands.rs:async_update }}
```

Now, we can process the `CommandMsg` similar to regular app updates.
The method we use is called `update_cmd()` and is very similar to the regular `update()` function.
Only the message type is `CommandOutput` instead of `Input`.
From here, we can simply assign the result of the web request to our model.

```rust,no_run,noplayground
{{#include ../examples/commands.rs:update_cmd }}
```

That's it!
It's really as simple as starting a task and processing a message on completion.

> With the [`command()`](https://relm4.org/docs/next/relm4/prelude/struct.ComponentSender.html#method.command) method, you are even more flexible because you can send multiple messages.

## Synchronous tasks

You can use commands for synchronous operations, too.
Compared to the asynchronous methods, we need to add the `spawn_` prefix to the method name to get the synchronous version.
Then, you can just pass a closure or a function pointer as task.

```rust,no_run,noplayground
{{#include ../examples/commands.rs:sync_update }}
```

The rest is identical to the asynchronous version.

```rust,no_run,noplayground
{{#include ../examples/commands.rs:sync_update_cmd }}
```

### Configuration

Commands run on a tokio runtime.
If you spawn a lot of commands in your application or want to fine-tune the runtime, you can set [two static variables](https://relm4.org/docs/next/relm4/index.html#statics) at the start of your main function to override the default value.
For example, Relm4 only uses one thread for asynchronous background tasks, which might not be enough.
Setting `RELM_THREADS` to 4 will increase the thread count by 3 additional threads.

> Note: Setting the static variables must be done early.
> As soon as the runtime is initialized (which happens when it's accessed for the first time), the values cannot be changes anymore.
