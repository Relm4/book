# Workers

Workers are simply components that don't have any widgets. They don't have any advantages over components apart from being simpler and a few performance benefits they get from not having to call the view function (because they don't have widgets).

You might wonder why they even exist. We're talking about a GUI library all the time, right? Well, they can be quite useful for applications that need to handle long tasks while remaining responsive. Imagine your web browser would be completely frozen while it loads content from a slow website. This would in fact happen if you would send the HTTP requests in your update function. If you use a worker for that instead, it could handle the requests from a different thread and send a message back once finished.

## Implementing a worker

A worker is implemented similar to a component. One difference is that you use `()` as a placeholder for the `Widgets` type in the `Model` trait. Also, since you don't have widgets for the worker, you don't need to implement the `Widgets` trait.

```rust,no_run,noplayground
impl Model for WorkerModel {
    type Msg = WorkerMsg;
    type Widgets = ();
    type Components = ();
}
```

The last difference is that worker don't need the parent widgets in the `RelmWorker::new` function. So to initialize a worker with the `Components` trait you only pass two arguments.

```rust,no_run,noplayground
impl Components<AppModel> for AppComponents {
    fn init_components(
        parent_model: &AppModel,
        _parent_widgets: &AppWidgets,
        parent_sender: Sender<AppMsg>,
    ) -> Self {
        AppComponents {
            worker: RelmWorker::new(parent_model, parent_sender),
        }
    }
}
```

Apart from that workers are just like components so I won't discuss an example here. You just need to define the messages you want the parent to send to the worker and handle them in the update function. There you can also send messages back to the parent component or the main app to signal that the worker has finished its work.

