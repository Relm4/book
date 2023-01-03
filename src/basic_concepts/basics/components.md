# Components

Components are the fundamental building blocks of Relm4. To create a component you need to implement the `Component` trait.

## The `Component` trait
The `Component` trait is the base of every component inside Relm4, it defines how a component should behave, communicate and produce widgets.

## The `SimpleComponent` trait
The `SimpleComponent` trait is a convenience trait that implements the `Component` trait, but removes some advanced features that are not relevant for most use-cases. 

 For every implementation of `SimpleComponent` Relm4 will also implement `Component` automatically. You just need to implement one of them and your type will implement `Component` either way. This is called [blanket implementation](https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods) and is used for traits like `From` in the standard library as well.