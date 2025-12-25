# Command Line Interfaces

The handling of CLI arguments in Relm4 has some specifics you should be aware of.

The first one is that Relm4/GTK tries to parse the arguments again even if you parsed them yourself already.
This means the program will crash with an error like `Unknown option --non-gtk-arg`.
To fix this you can use the [`with_args`](https://docs.rs/relm4/latest/relm4/struct.RelmApp.html#method.with_args) method to provide the arguments the GTK app should parse.
The easiest way is to just provide an empty `Vec` but this has the disadvantage that the standard GTK arguments don't work anymore.

We will now make it work in combination with the popular [`clap`](https://docs.rs/clap/latest/clap/) crate.
To be precise we will use the `derive` feature which you can learn about in the [`clap` documentation](https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html) but it works with the builder pattern too of course.

To pass a `Vec` of GTK arguments we need to separate the arguments we want to consume ourselves from those we want to pass to GTK.
In `clap` you can achieve this using a combination of [`allow_hyphen_values`](https://docs.rs/clap/latest/clap/struct.Arg.html#method.allow_hyphen_values) and [`trailing_var_arg`](https://docs.rs/clap/latest/clap/struct.Arg.html#method.trailing_var_arg).
```rust,no_run,noplayground
{{#include ../examples/cli.rs:args_struct }}
```

Now in our main function we can parse the CLI arguments using `Args::parse()` and pass `args.gtk_options` to GTK/Relm4.
The first argument is (as per convention) the program invocation so we need to add that first:
```rust,no_run,noplayground
{{#include ../examples/cli.rs:main }}
```

## Result
To compile, run and pass arguments to the built binary in one command we can use `cargo run --` and pass our arguments after that.
> If you wonder what the `--` means: This is the [*end of options* convention](https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap12.html#tag_12_02):
> *"The first -- argument that is not an option-argument should be accepted as a delimiter indicating the end of options. Any following arguments should be treated as operands, even if they begin with the '-' character."*

We can now look at the result using `cargo run -- --help`:
```
Usage: cli [OPTIONS] [GTK_OPTIONS]...

Arguments:
  [GTK_OPTIONS]...  Unknown arguments or everything after -- gets passed through to GTK

Options:
      --non-gtk-arg  some argument to test
  -h, --help         Print help
  -V, --version      Print version
```

This is the help text provided by `clap`.
If you want to see the GTK help text you can use `cargo run -- -- --help`:
```
Usage:
  cli [OPTION?]

Help Options:
  -h, --help                 Show help options
  --help-all                 Show all help options
  --help-gapplication        Show GApplication options
```
And if the GTK option is unique and not used by your program the (second) `--` is not needed anymore, e.g. `cargo run -- --help-all`:
```
Usage:
  cli [OPTION?]

Help Options:
  -h, --help                 Show help options
  --help-all                 Show all help options
  --help-gapplication        Show GApplication options

GApplication Options:
  --gapplication-service     Enter GApplication service mode (use from D-Bus service files)
```

Of course you can replace `cargo run --` by your binary name later, e.g.: `your-cool-app --help-all`.

## The complete code

Here is a minimal working example code with some debug output:

```rust,no_run,noplayground
{{#include ../examples/cli.rs:all }}
```
