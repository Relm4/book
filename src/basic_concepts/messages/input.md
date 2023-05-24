# Input messages

Input messages are a way for our components to receive information, think of them as our inbox 📬.

Let's look at it with a simple `MailboxComponent` example:

We have our `Inbox`, capable of receiving emails from other people.

```rust,no_run,noplayground
enum Inbox {
    GetEmail(Email),
}
```

These messages are received by our component and handled in the `update` function.

```rust,no_run,noplayground
fn update(&mut self, message: Self::Input, ...) {
    match message {
        Inbox::GetEmail(email) => self.emails.push(email)
    }
}
```

Our `MailboxComponent` can not only receive emails from other people, but we can also send emails to ourselves.

Components work in the same way, they can either receive messages from other components or send themselves messages to update their own model.
