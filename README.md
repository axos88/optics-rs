# optics-rs

[![CI Status](https://github.com/axos88/optics-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/axos88/optics-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/optics.svg)](https://crates.io/crates/optics)
[![Docs.rs](https://docs.rs/optics/badge.svg)](https://docs.rs/optics)
[![Rust 1.87+](https://img.shields.io/badge/rust-1.87%2B-blue.svg)](https://blog.rust-lang.org/)
[![License](https://img.shields.io/crates/l/optics.svg)](https://github.com/axos88/optics-rs/blob/main/LICENSE)

## Summary
`optics` is a set of **composable, type-safe tools** for accessing, transforming, and navigating data structures.
It takes inspiration from the optics concepts you'd find in functional languages like Haskell —
but it’s designed by someone who does not have a complete grasp on type theory or Van Laarhoven/profunctor lenses.

It tries to mimic similar functionality within the constraints of Rust’s type system without higher-kinded types.

The goal was simple:

👉 Build something useful and composable for everyday Rust projects — no magic.

### ✨ Features
- Lenses — for focusing on subfields of structs
- Prisms — for working with enum variants
- Isomorphisms — for invertible type transformations
- Fallible Isomorphisms — for conversions that might fail (e.g., String ↔ u16)
- Composable — optics can be chained together to drill down into nested structures


- No dependencies — pure Rust, no external crates
- `no_std` support — usable in embedded and other restricted environments
- Type-safe, explicit interfaces
- Honest documentation

### 📦 Philosophy

This is a **layman's implementation** of optics. I don’t fully grasp all the deep type theory behind
profunctor optics or Van Laarhoven lenses. Instead, I built something practical and composable,
within the limitations of Rust’s type system and my own understanding.

Some of the generic type bounds are clunky. I ran into situations where missing negative trait bounds
in Rust forced some awkward decisions. There’s also a lot of repetition in the code — some of it could
likely be reduced with macros, but I’m cautious about that since excessive macro usage tends to kill
readability and maintainability.

I genuinely welcome critics, feedback, and suggestions. If you see a way to clean up the generics, improve trait compositions, or simplify the code structure, I’m all ears. Drop me a PR, an issue, or a comment.

### 📌 Status

This is a **pre-release**, and the code is **unfinished** — but it’s good enough to start experimenting with in real projects.

There’s a lot of room for simplification and improvement. Type-level constraints, trait bounds, and generic compositions are kind of bloated right now, and I wouldn’t mind help tightening it up.

### 💬 Call for Critics

If you know your type theory, or even if you just have an eye for clean Rust APIs — I’d love for you to take a look. Suggestions, critiques, and even teardown reviews are welcome.
This is very much a **learning-while-doing** project for me.


---

## Documentation

This crate provides an implementation of various optic types (Lenses, Prisms, Isos, and Fallible Isos), 
which are used to focus on and manipulate parts of data structures in a type-safe and composable manner. 
The crate allows for the combination of optics, providing powerful abstractions for working with deeply nested 
or structured data.

The core trait `Optic<S, A>` serves as the base for all optic types in this crate, and the goal of this library 
is to allow the manipulation of data types via optics while ensuring safety and correctness in a functional 
programming style.

### `Optic<S, A>` — The Base Trait
The `Optic<S, A>` trait represents a fundamental abstraction of an optic. 
All optic types in this crate implement this trait, providing a unified interface for working with lenses, 
prisms, isos, and fallible isos. The S type represents the source, and the A type represents the target of the optic.

This trait provides a method for getting and one for setting values.

### Optic Types
This crate defines several types of optics that extend the functionality of the `Optic<S,A>` trait:

  - **Lenses**: Lenses focus on a part of a structure and provide a way to get and set the value of that part, such as `Point` -> `x: u32`

  - **Prisms**: Prisms in general allow for focusing on a specific variant of a sum type (like enums in Rust). 
    They can be used to extract or modify the value of that variant, or a focusing operation that may fail because the 
    value that may or may not be present, such as `Option<u32>` -> `u32`

  - **Isos** (Isomorphisms): Isos provide a bijective mapping between types. 
    They can be used to transform data between two types while preserving structure, such as an `IpAddrV4` <=> `u32`.
    
  - **Fallible Isos**: Fallible isos extend the concept of isos by introducing the possibility of failure. 
    Both the getting and setting operations may fail, and they return Result types that allow you to handle errors. 
    This can be used for parsing or validating data, such as converting a string to an integer.

### Combining Optics
One of the most powerful features of this crate is the ability to combine optics. 
Since prisms, isos, and lenses are related 
(i.e. not considering semantics a lens is the infallible version of a prism, and an iso is a bidirectional lens), 
you can safely combine these optics to manipulate different parts of your data structures in a compositional way.

For instance, you can compose a lens with another lens to operate on nested data structures, or you can combine 
a prism with a lens to focus on a particular variant of an enum. 
Fallible isos can also be composed with other fallible isos, allowing you to build complex data manipulation 
logic that may involve potential failures.

### Type Safety and Composability
This crate ensures that all optics are type-safe and can be composed together while preserving their types. 
For example, combining a Lens<S, A> with a Prism<A, B> results in a new optic that focuses on S and provides 
access to B, ensuring that the types align correctly during the composition process.

### Limitations
  - Type Theory and Category Theory: The library is still evolving, and the underlying concepts may not fully 
    implement category theory or other type-theoretical ideas. While this crate aims to be powerful, there is plenty 
    of room for improvement and refinement.

  - HKT (Higher-Kinded Types): Rust's lack of support for higher-kinded types (HKT) means that some abstractions 
    are probably not possible. For example, you cannot write a general Prism that would work for any `Option<T>` -> `T`.

  - Error Handling: All prisms in the crate require the error type to implement `From<NoFocus>` and all lens need to implement `From<Infallible>`.

### Future Improvements
The crate is designed to be extensible and will likely grow to include more optics such as `Traversals` if needed. 
The focus is on allowing safe, composable transformations of data while providing powerful abstractions for 
common patterns in functional programming.

### `Prism` and Custom Errors
Currently, Prism uses a predefined error type for failures, but in the future, this may be enhanced to 
allow returning any custom error type. This will make Prism behave more like a general Optic type, 
and would make Prism redundant and will be deprecated once this change is made.

## Examples
Below is a simplified example of how the optics work in this crate. The code below illustrates how to use and combine the various optic types.

```rust
use optics::{LensImpl, FallibleIsoImpl, PrismImpl, Optic, NoFocus};
use optics::composers::{ComposableLens, ComposablePrism};

#[derive(Debug, Clone)]
struct HttpConfig {
  bind_address: Option<String>,
  workers: usize,
}

#[derive(Debug, Clone)]
struct AppConfig {
  http: HttpConfig,
  name: String,
}

struct MyError;

impl From<MyError> for NoFocus {
  fn from(_: MyError) -> Self {
    NoFocus
  }
}

impl From<NoFocus> for MyError {
  fn from(_: NoFocus) -> Self {
    MyError
  }
}


fn main() {
  // Define lenses to focus on subfields
  let http_lens = LensImpl::<AppConfig, HttpConfig>::new(
    |app| app.http.clone(),
    |app, http| app.http = http,
  );

  let bind_address_prism = PrismImpl::<HttpConfig, String>::new(
    |http| http.bind_address.clone(),
    |http, addr| http.bind_address = Some(addr),
  );

  let minimum_port = 1024;
  // Define a fallible isomorphism between String and u16 (parsing a port)
  let port_fallible_iso = FallibleIsoImpl::<String, u16, MyError, _, _>::new(
    |addr: &String| {
      addr.rsplit(':')
        .next()
        .and_then(|port| port.parse::<u16>().ok()).ok_or(MyError)
    },
    move |port: &u16| if *port > minimum_port { Ok(format!("0.0.0.0:{}", port)) } else { Err(MyError) }
  );

  // Compose lens and fallible iso into a ComposedFallibleIso

  let http_bind_address_prism = http_lens.compose_lens_with_prism(bind_address_prism);
  let http_bind_address_port_prism = http_bind_address_prism.compose_prism_with_fallible_iso::<MyError>(port_fallible_iso);

  let mut config = AppConfig {
    http: HttpConfig {
      bind_address: Some("127.0.0.1:8080".to_string()),
      workers: 4,
    },
    name: "my_app".into(),
  };

  // Use the composed optic to get the port
  let port = http_bind_address_port_prism.try_get(&config).unwrap();
  println!("Current port: {}", port); // 8080

  // Use it to increment the port and update the config
  http_bind_address_port_prism.set(&mut config, port + 1);

  println!("Updated bind address: {:?}", config.http.bind_address); // port is now 8081
}

main();
```


