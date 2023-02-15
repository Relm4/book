# Workers

Workers are simply components that don't have any widgets.
They can be quite useful for applications that need to handle long tasks while remaining responsive.
In particular, they are suitable for CPU-bound tasks which need to be handled **one at the time** because they run on a different thread.

## Implementing a worker

A worker is implemented similarly to a component by using the `Worker` trait.
Since workers don't have widgets, you don't need to provide a `Widgets` type.

```rust,ignore
{{#include ../examples/worker.rs:worker_impl}}
```

Workers are constructed similarly to components, too.
Use the provided builder to retrieve a `WorkerController`.

```rust,ignore
{{#include ../examples/worker.rs:worker_construction}}
```

Through the `WorkerController`, you can send and receive messages from the worker.
The worker's `update` function will run on a separate thread, so your other components won't be blocked.

```rust,ignore
{{#include ../examples/worker.rs:app_model}}
```
