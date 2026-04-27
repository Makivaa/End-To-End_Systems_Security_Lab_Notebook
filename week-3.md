# Overview
TODO
## Rustlings
Do up to and including rustlings 18_iterators
### Generics 
Generics is the topic of generalizing types and functionalities to broader cases.
This is extremely useful for reducing code duplication in many ways, but can call for some rather involved syntax.
Namely, being generic requires taking great care to specify over which types a generic type is actually considered valid.
The simplest and most common use of generics is for type parameters.
Monomorphization turns different "types" of generics into a specialized version of themselves

```
Option<T>(&t) -> Option_u32(&u32), Option_i32(&i32), etc
```

is an example of monomorphization.

### Traits
A trait is a collection of methods.

Data types can implement traits. To do so, the methods making up the trait are defined for the data type. For example, the `String` data type implements the `From<&str>` trait. This allows a user to write `String::from("hello")`.

In this way, traits are somewhat similar to Java interfaces and C++ abstract classes.

Some additional common Rust traits include:

- `Clone` (the `clone` method)
- `Display` (which allows formatted display via `{}`)
- `Debug` (which allows formatted display via `{:?}`)

Because traits indicate shared behavior between data types, they are useful when writing generics. 

### Lifetimes
Lifetimes tell the compiler how to check whether references live long
enough to be valid in any given situation. For example lifetimes say
"make sure parameter 'a' lives as long as parameter 'b' so that the return
value is valid".

They are only necessary on borrows, i.e. references,
since copied parameters or moves are owned in their scope and cannot
be referenced outside. Lifetimes mean that calling code of e.g. functions
can be checked to make sure their arguments are valid. Lifetimes are
restrictive of their callers.

To derive lifetime of variables 
```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

Must also be used in structs
```
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}
```

### Tests
General information for testing code, it should be noted there are multiple types of asserts

```
assert!(fn(val_1) == val_2); <- Assert something is equal within statement
assert_eq!(fn(val_1), val_2); <- Assert val_1 equal val_2
```

### Iterators
TODO

## Reading - Arrakis: The Operating System is the Control Plane

### Questions
TODO

### Brief summary
TODO

### List the major contributions (what is it that's novel) compared to previous work
TODO

### Strengths / Weaknesses of their approach
TODO