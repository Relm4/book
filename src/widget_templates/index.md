# Widget templates

Widget templates are a simple way to define reusable UI elements. 
When building complex UIs, they allow you to focus on the application logic instead of complex trees of widgets.
Yet most importantly, widget templates help you to reduce redundant code.
For example, if you use a widget with the same properties multiple times in your code, templates will make your code a lot shorter.

> The app we will write in this chapter is also available [here](https://github.com/Relm4/Relm4/blob/main/examples/widget_template.rs).
> Run `cargo run --example widget_template` from the [example directory](https://github.com/Relm4/Relm4/tree/main/examples) if you want to see the code in action.
## Defining templates

To define a widget template, you need to implement the `WidgetTemplate` trait for a new type.
You could do this manually, but the easiest solution is to use the `#[relm4::widget_template]` attribute macro.
The macro will create the type and implement the trait for you.

For example, the following code block will create a template for a `gtk::Box` with a certain margin and custom CSS.

```rust,no_run,noplayground
{{#include ../examples/widget_template.rs:box_template }}
```
Similarly, we can create a template for a `gtk::Spinner` that already spins when it's created.
```rust,no_run,noplayground
{{#include ../examples/widget_template.rs:spinner_template }}
```
> To create public templates, you can use `#[relm4::widget_template(pub)]`, similar to the `#[relm4::component(pub)]` macro.
### Template children
Templates are more than just pre-initialized widgets.
They can also have children, which can be referred to later as template children.
This is very useful if you use nested widget in you UI, because the template allows you to flatten the structure.
In other words, no matter how deeply nested a template child is, it will always be accessible directly from the template.
We'll see how this works in the next section, but first we'll create a deeply nested template.
We use the templates we defined earlier by using the `#[template]` attribute.
Also, we assign the name `child_label` to our last widget, which is all we need to make it a template child.
In general, naming a widget in a template is all that's needed to make it a template child.
```rust,no_run,noplayground
{{#include ../examples/widget_template.rs:nested_template }}
```
## Using templates
To use templates in a component, we use the `#[template]` and `#[template_child]` attributes.
In this case, we use the `CustomBox` type we just defined with the `#[template]` attribute we already used.
To access its `child_label` template child, we only need to use the `#[template_child]` attribute and the name of the child.
As you can see, we now have access to the `whild_label` widget, which actually is wrapped into 4 `gtk::Box` widgets.
We can even use assign or overwrite properties of the template and its children, similar to regular widgets.
Here, we use the `#[watch]` attribute to update the label with the latest counter value.
```rust,no_run,noplayground
{{#include ../examples/widget_template.rs:component_start }}
```
### Some notes on orders
If you run this code, you will notice that the label appears above the two buttons, which is contrary to our widget definition.
This happens because widget templates are initialized before other modifications happen.
The `CustomBox` template will initialize its `child_label` and append it to its internal `gtk::Box` widget and only then the two buttons are added.
However, you can work around this by using methods like `prepend`, `append` or `insert_child_after` (if you use a `gtk::Box` as container) or by splitting your templates into smaller ones.
> To make template children appear in the same order as they are used, widget templates would require dynamic initialization of its children.
> This would increase the complexity of the internal implementation by a lot (or might not be possible at all) and is therefore not planned at the moment.
## The complete code
```rust,no_run,noplayground
{{#include ../examples/widget_template.rs:all }}
```