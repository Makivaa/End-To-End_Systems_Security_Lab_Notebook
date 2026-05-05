# Overview
Mid-quarter notebook review next week
## Rustlings
Do up to and including 20_threads

### Smart Pointers
In Rust, smart pointers are variables that contain an address in memory and reference some other data, but they also have additional metadata and capabilities.
Smart pointers in Rust often own the data they point to, while references only borrow data.

Rc<T> - Stands for reference counted and allows multiple owners of a single resource ?
```
let resource = Rc::new(resource);
let owner_1 = enum::value_1(Rc::clone(&resource));
let owner_2 = enum::value_2(Rc::clone(&resource));
```
Arc - Thread safe version of Rc ?
```
let shared_resource = Arc::new(resource);
let child_resource = Arc::clone(&resource);
```
Cow` (Clone-On-Write) - Enclose and provide immutable access to borrowed data and clone the data
```
// Clone occurs because `input` needs to be mutated
let mut input = Cow::from(&resource);

// No clone occurs because `input` doesn't need to be mutated
let mut input = Cow::from(&resource);
```
Box - A smart pointer used to store data on the heap, which also allows us to wrap a recursive type.
```
// Use of a `Box` in the enum definition
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Create an empty cons list.
fn create_empty_list() -> List {
    List::Nil
}

// Create a non-empty cons list.
fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))
}
```

### Threads
In most current operating systems, an executed program's code is run in a process, and the operating system manages multiple processes at once.
Within your program, you can also have independent parts that run simultaneously. The features that run these independent parts are called threads.
```
use std::{
    thread,
    time::{Duration, Instant},
};
```

When dealing with mutable shared resources across threads we have to slightly modify how we handle smart pointers.
```
// `Arc` isn't enough if you want a **mutable** shared state.
let status = Arc::new(Mutex::new(Status { done: 0 }));

// You must take an action before you update a shared value.
let mut status = status_shared.lock().unwrap();
status.done += 1;

// Must also unlock to print the value of `JobStatus.jobs_done`.
println!("Done: {}", status.lock().unwrap().done);
```