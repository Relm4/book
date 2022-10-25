# Components

Components are the fundamental building blocks of Relm4. To create a component you need to implement the `Component` trait.

## The `Component` trait
The `Component` trait is the base of every component inside Relm4, it defines how a component should behave, communicate and produce widgets.

## The `SimpleComponent` trait
The `SimpleComponent` trait is a convenience trait that implements the `Component` trait, but removes some advanced features that are not relevant for most use-cases. 

There's an implementation for `Component` for every type that implements `SimpleComponent` so in the end you only implement one of them.