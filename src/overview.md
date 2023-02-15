# Overview

| Option                         | Synchronous | Async | Non-blocking | !Send |
| ------------------------------ | :---------: | :---: | :----------: | :---: |
| Workers                        | ✅          | ❌    | ❌           | ❌    |
| Async components and factories | ❌          | ✅    | ❌           | ✅    |
| Commands                       | ✅          | ✅    | ✅           | ❌    |

In this context, non-blocking means you can have run multiple instances at the same time.

`!Send` means that the types involved don't need to implement `Send` so you can use widgets or `Rc` for example.

## Summary

+ **Async components and factories:**
  + Run asynchronous tasks on the main runtime
  + Allow other components to keep running while awaiting futures
  + Await during initialization or updates

+ **Commands:**
  + Run tasks on a runtime in the background
  + Supports **both synchronous and asynchronous** tasks
  + Run **several tasks in parallel**
  + Drop tasks as soon as the component is destroyed

+ **Workers:**
  + Handle IO-bound or CPU-intensive tasks **one at the time** on a different thread
  + The update function should be executed in another thread
  + You need a model to store state for processing messages
