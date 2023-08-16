# Accessing Nested Template Elements

Starting from the version **0.6.2**, you can access nested elements on templates.

Imagine a template called "MainWindow" which contains pages as Widget Templates:
```rust,no_run,noplayground
{{#include ../examples/widget_template_nested_access.rs:main_window_template }}
```

`SettingsPage` and `HomePage` are also a widget template:
```rust,no_run,noplayground
{{#include ../examples/widget_template_nested_access.rs:home_page_template }}
```
```rust,no_run,noplayground
{{#include ../examples/widget_template_nested_access.rs:settings_page_template }}
```

If you want to handle `MainWindow->SettingsPage->btn_dark_mode`'s clicked event, you can simply do it like this:
```rust,no_run,noplayground
{{#include ../examples/widget_template_nested_access.rs:component_start }}
```

## The complete code
```rust,no_run,noplayground
{{#include ../examples/widget_template_nested_access.rs:all }}
```