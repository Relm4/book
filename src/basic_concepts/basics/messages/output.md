# Output messages

**Output** messages are sent by components to other components and handled differently depending on the type of components that receives them. We can think of them as our outbox 🚚.

Let take our previous `MailboxComponent` example and add the following.

```rust,no_run,noplayground
enum Outbox {
    SendEmail(Email),
}
```

We can modify our previous example and send one of the emails to somebody else.

```rust,no_run,noplayground
match message {
    Outbox::SendEmail(email) => EmailProvider::SendEmail(email)
}
```

> This is handled in the parent component.

When we send the email, out outbox will deliver that email to the email provider, in this case, our `EmailProvider` component will take care of delivering our email.

`EmailProvider` is the parent component of `MailboxComponent`.
