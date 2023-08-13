# Resource Bundles

Some Relm4 apps require static assets or resources (such as icons or images) to function.

In GTK apps, static assets are transformed into GResource bundles, which are then loaded by the app. This guide shows how to set up GResource bundles within a Relm4 project.

## Cargo

This demonstrates a Cargo-only approach to including resources (i.e. it does not require extra build tools like Meson, or out-of-band shell scripts).

### `data` directory

We add the static resources (in this example, icon files), plus a `gresource` descriptor, to the project's `data` folder:

```
data/
  icons/
    icon-foo.svg
    icon-bar.svg
  icons.gresource.xml
```

The icons are placed under the `data/icons` directory.

The `icons.gresource.xml` file looks like this (adapt it as required, e.g. using `-symbolic` icon names):

```xml
<?xml version="1.0" encoding="UTF-8"?>
<gresources>
    <gresource prefix="com/example/Foobar/icons/24x24/actions/">
        <file preprocess="xml-stripblanks" alias="icon-foo.svg">icons/icon-foo.svg</file>
        <file preprocess="xml-stripblanks" alias="icon-bar.svg">icons/icon-bar.svg</file>
    </gresource>
</gresources>
```

### `Cargo.toml`

In `Cargo.toml`, we add a build dependency on `glib-build-tools`. This gives us access to the `glib_build_tools::compile_resources` function which we will need later:

```toml
[package]
name = "foobar"

[build-dependencies]
glib-build-tools = "0.17.10"
```

Note: you should ensure that the `glib-build-tools` version aligns with the general GLib version you are building for.

### `build.rs`

In `build.rs`, we call the `compile_resources` function which creates a GResource bundle from the icons:

```rust
use glib_build_tools::compile_resources;

fn main() {
    compile_resources(
        &["data"],
        "data/icons.gresource.xml",
        "icons.gresource",
    );
}
```

### `main.rs`

In `main.rs` (or wherever you initialise your Relm4 app), we load the `icons.gresource` bundle that Cargo generates:

```rust
fn initialize_custom_icons() {
    gio::resources_register_include!("icons.gresource").unwrap();

    let display = gdk::Display::default().unwrap();
    let theme = gtk::IconTheme::for_display(&display);
    theme.add_resource_path("/com/example/Foobar/icons");
}

fn main() {
    let app = RelmApp::new("com.example.Foobar");

    // (optional) initialize default icons
    relm4_icons::initialize_icons();

    // custom icons
    initialize_custom_icons();
}
```

It should now be possible to reference the resources by name within your app, for example:

```rust
view! {
    gtk::Button {
        set_icon_name: "icon-foo"
    }
}
```
