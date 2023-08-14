# Continuous Integration

We recommend that you establish a CI build for your Relm4 app. This guide describes how to do it, and the caveats you must observe to make the build work.

## GitHub Actions

Starting with Relm 0.6.1, you can set up a CI build for your app on GitHub Actions.

If your project builds with Cargo in a reasonably self-contained way (e.g. using `build.rs` to do packaging rather than external shell scripts), you can do this with an almost entirely standard Cargo build workflow, as shown below.

**Caveats:**

- You must `apt-get install` the relevant system libraries for the parts of GTK or Adwaita that your Relm4 app uses.
- You must synchronise the GNOME version your app uses with the GNOME version available on the GitHub Actions runner. (For example, on the standard Ubuntu 22.04 runner, you must use GNOME 42.) Set this in Cargo.toml with `relm4 = { features = ["gnome_<version>"] }`.

```yaml
name: Rust

on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]

jobs:
  build:
    runs-on: ubuntu-22.04
    steps:
    - uses: actions/checkout@v3
    - run: sudo apt-get update
    - run: sudo apt-get install -y libgtk-4-dev libadwaita-1-dev
    - name: Build
      run: cargo build --verbose
    - name: Run tests
      run: cargo test --verbose
```