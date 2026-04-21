# Overview
Got sick so just focusing on reading and Rustlings, first paper reading due next week 4/28
## Rustlings
Do up to and including rustlings 13_error_handling
### Structs
Regular - 
    struct RegularStruct {
        name: type,
        ...,
        name: type,
    }

Tuple - 
    struct TupleStruct(type,..., type);

Unit - 
    struct UnitStruct;

You can implement logic using a struct by passing a function (with correct inputs) a reference to
the structs self and return values based on the names and types provided by said struct

### Enums
Rust allows you to define types called "enums" which enumerate possible values.
Enums are a feature in many languages, but their capabilities differ in each language. Rust's enums are most similar to algebraic data types in functional languages, such as F#, OCaml, and Haskell.
Useful in combination with enums is Rust's "pattern matching" facility, which makes it easy to run different code for different values of an enumeration.

fn process(&mut self, enum: Enum) {
        // Create a match expression to process the different Enum
        // variants using the methods defined above.
        match enum {
            Enum::Variant { val, val } => self.variant(val, val),
            Enum::Variant(struct) => self.variant(struct),
            Enum::Variant(s) => self.variant(s),
            Enum::Variant(a, b, c) => self.variant(a, b, c),
            Enum::Variant => self.variant(),
        }
    }

### Strings
Rust has two string types, a string slice (`&str`) and an owned string (`String`).
Each string type is capable of different things, see documentation for differences honestly

### Modules
mod module {
    fn some_function <- private
    pub fn some_function <- public
}

mod module {
    pub use self::cat_1::A as a; <- makes "private" value public
    pub use self::cat_2::C as c; <- makes "private" value public

    mod cat_1 {
        pub const A: &str = "a"; <- private here
        pub const B: &str = "b"; <- private here
    }

    mod cat_2 {
        pub const C: &str = "c"; <- private here
        pub const D: &str = "d"; <- private here
    }
}

Use the `use` keyword to bring module paths from modules from
anywhere and especially from the standard library into scope.
use std::time::{SystemTime, UNIX_EPOCH}; <- example

### Hashmaps
A *hash map* allows you to associate a value with a particular key.
let mut name = HashMap::<val, val>::new(); <- instantiate hashmap

### Options
Type Option represents an optional value: every Option is either Some and contains a value, or None, and does not.
Option types are very common in Rust code, as they have a number of uses:

- Initial values
- Return values for functions that are not defined over their entire input range (partial functions)
- Return value for otherwise reporting simple errors, where None is returned on error
- Optional struct fields
- Struct fields that can be loaned or "taken"
- Optional function arguments
- Nullable pointers
- Swapping things out of difficult situationss

While let Some nesting and if let Some nesting explained???

### Error Handling
Most errors aren't serious enough to require the program to stop entirely.
Sometimes, when a function fails, it's for a reason that you can easily interpret and respond to.
For example, if you try to open a file and that operation fails because the file doesn't exist, you might want to create the file instead of terminating the process.
