# Output messages

Output messages are sent by components to other components and handled differently depending on the type of components that receives them. We can think of them as our outbox 🚚.

Let's take our previous `MailboxComponent` example and add the following.

```rust,no_run,noplayground
enum Outbox {
    SendEmail(Email),
}
```

We can modify our previous example to forward the emails to somebody else.

```rust,no_run,noplayground
fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
    match message {
        Inbox::GetEmail(email) => sender.output(Outbox::SendEmail(email)),
    }
}
```

Usually, output messages are handled by the parent component, which is the component that creates and stores our `MailboxComponent`.
You can think of it like a tree with one component at the root and many child components that branch out.
