# Workers

Workers are simply components that don't have any widgets. They don't have any
advantages over components apart from being simpler and a few performance
benefits they get from not having to call the view function (because they don't
have widgets).

You might wonder why they even exist. We're talking about a GUI library all the
time, right? Well, they can be quite useful for applications that need to handle
long tasks while remaining responsive. Imagine your web browser would be
completely frozen while it loads content from a slow website. This would in fact
happen if you would send the HTTP requests in your update function. If you use a
worker for that instead, it could handle the requests from a different thread
and send a message back once finished.

## Implementing a worker

A worker is implemented similarly to a component, though you implement the
`Worker` trait instead. Also, since you don't have widgets for the worker, you
don't need to provide a `Widgets` type.

```rust,ignore
{{#include ../examples/worker.rs:worker_impl}}
```

Workers are constructed similarly to components, too. Use the provided builder
to retrieve a `WorkerController`.

```rust,ignore
{{#include ../examples/worker.rs:worker_construction}}
```

```rust,ignore
{{#include ../examples/worker.rs:app_model}}
```

Through the `WorkerController`, you can send and receive messages from the
worker. The worker's `update` function will run in a separate event loop, so
your other components won't be blocked.
