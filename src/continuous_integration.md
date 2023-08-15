# Continuous Integration

We recommend that you establish a CI build for your Relm4 app. This guide describes how to do it, and the caveats you must observe to make the build work.

## GitHub Actions

Starting with Relm 0.6.1, you can set up a CI build for your app on GitHub Actions.

We recommend that you use the [`gtk4-rs`](https://github.com/gtk-rs/gtk4-rs/pkgs/container/gtk4-rs%2Fgtk4) container approach, as shown below:

```yaml
name: Rust

on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]

jobs:
  build:
    runs-on: ubuntu-latest
    container:
      image: ghcr.io/gtk-rs/gtk4-rs/gtk4:latest # TODO enable minor version tags / pinning
    steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        profile: minimal
        toolchain: stable
    - name: Build
      run: cargo build
    - name: Test
      run: cargo test
```

**Note:** You *can* alternatively just run the `cargo build` on the `ubuntu-latest` base image. However, this will tie your Relm app's GNOME and GTK version to Ubuntu's 2-year LTS release cycle, so you will not be able to use newer GNOME / GTK versions in the meantime. We therefore recommend that most projects use the `gtk4-rs` container approach instead.