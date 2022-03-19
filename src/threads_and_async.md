# Overview

| *Category* | Components | Workers | Message handlers |
|:---|:---:|:---:|:---:|
| Run on different thread | ✅ | ✅ | ✅ |
| Async | ❌ | ✅ | ✅ |
| Non-blocking message handling | ❌ | ❌ | ✅ |

## When to use ...

+ **components:**
  + Abstract parts of your UI
  + The update function should be run on a different thread

+ **workers:**
  + Handle IO-bound or CPU-intensive tasks **one** at the time on a different thread
  + You need a model to store state for processing messages

+ **message handlers:**
  + Handle **multiple** IO-bound or CPU-intensive tasks at the time
  + All the information you need is sent inside the message

## Threads

Workers are usually used to run tasks on a different thread to allow the main thread to run the UI. Let's see how this works!

### Running a component on a different thread

You might remember this section of code from the example application in the components chapter.

```rust,no_run,noplayground
{{#include ../examples/components.rs:components_impl }}
```

In order to run the dialog component on a new thread, we just need to change one line:

```rust,no_run,noplayground
impl Components<AppModel> for AppComponents {
    fn init_components(
        parent_model: &AppModel,
        parent_widgets: &AppWidgets,
        parent_sender: Sender<AppMsg>,
    ) -> Self {
        AppComponents {
            header: RelmComponent::new(parent_model, parent_widgets, parent_sender.clone()),
            dialog: RelmComponent::with_new_thread(parent_model, parent_widgets, parent_sender),
        }
    }
}
```

Instead of `RelmComponent::new` we used `RelmComponent::with_new_thread`. The same is true for workers. `RelmWorker::new` runs the worker on the same thread and `RelmWorker::with_new_thread` spawns a new thread for the worker.

> Components have widgets that, in the case of GTK4, neither implement `Send` nor `Sync`. That means we can't run the view function from a different thread, but only the update function that just operates on the model. Internally, Relm4 sends the model from a new thread that handles the update function to the main thread that then handles the view function and back to the new thread again. This is not optimal regarding performance and therefore only recommended if you don't send a lot of messages to the component. Alternatively, you can always do the heavy work in a worker or a message handler because they don't have this problem.

## Async

Async update functions are exclusive for workers and message handlers currently (if you need async components please open an issue). If you enable the tokio-rt feature, you can use an `AsyncRelmWorker` type that uses an async update function from the `AsyncComponentUpdate` trait. Apart from that, they are just like normal workers that run in a new thread. The ["tokio" example](https://github.com/AaronErhardt/relm4/blob/main/relm4-examples/examples/tokio.rs) shows how this can be used with for async HTTP requests.

### Non blocking async

Technically, even async workers will block the execution between messages. They can run non-blocking code from their update function but they can not handle more than one message at the time. This can be too slow in some cases. 

For example, if you have an app that fetches the avatar images of many users and you send one message to your worker for every avatar image, the worker will fetch the images one after the other. This wouldn't be much better than blocking requests and may take some time.

There are three ways to improve this: 

+ Create your own async runtime in message handler. This is shown in the [non_blocking_async example](https://github.com/AaronErhardt/relm4/blob/main/relm4-examples/examples/non_blocking_async.rs).
+ Send a vector with all avatar images you need to your worker, so it can send all asynchronous requests at once.
+ Spawn a new thread for each message that sends a HTTP request and sends a message back.

### The message queue problem

Because workers tend to take a lot of time during the update function you should make sure to not bombard them with messages. Imagine you have a button in your application that allows the user to update a web page. If the user presses the button, a new request is sent by a worker that responds with a message once the request is completed. If the button can be clicked and a message is sent for each click while the worker is fetching the web page you could quickly have a lot of unprocessed messages in the queue of your worker. To avoid this, make sure to only send the message once and wait until the worker is finished.

### Multiple threads and async without workers

One reason you always get a new sender passed into your update function is that you can spawn a new thread and move a cloned sender into it. This can sometimes be more flexible than defining a worker or even a message handler. You can simply use `std::thread::spawn` for this or spawn any async runtime you want.

For example you could do this in your update function:

```rust,no_run,noplayground
std::thread::spawn(move || {
    send_request();
    send!(sender, AppMsg::RequestComplete);
});
```

### Async inside the main event loop

GTK uses an event loop from glib to handle asynchronous events. In fact the senders we've been using all the time use channels on that event loop. This event loop also allows us to execute futures. Relm4 provides a `spawn_future` function to do exactly that. The only drawback of this is that most crates relying on a tokio runtime won't work and that the future is run on the main thread. The ["future" example](https://github.com/AaronErhardt/relm4/blob/main/relm4-examples/examples/future.rs) shows how this can be used.
