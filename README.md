# optics-rs

[![CI Status](https://github.com/axos88/optics-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/axos88/optics-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/optics.svg)](https://crates.io/crates/optics)
[![Docs.rs](https://docs.rs/optics/badge.svg)](https://docs.rs/optics)
[![Rust 1.87+](https://img.shields.io/badge/rust-1.87%2B-blue.svg)](https://blog.rust-lang.org/)
[![License](https://img.shields.io/crates/l/optics.svg)](https://github.com/axos88/optics-rs/blob/main/LICENSE)

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

## Code Examples
Below is a simplified example of how the optics work in this crate. The code below is inspired by the test suite (test.rs) and illustrates how to use Lens, Prism, Iso, and FallibleIso.

```rust
#[test]
fn optic_example() {
// Create a Config object with some default values
let mut config = Config::default();

    // Lens to focus on the main DatabaseConfig field of Config
    let main_lens = LensImpl::<Config, DatabaseConfig>::new(
        |c| c.main.clone(),
        |c, v| c.main = v,
    );

    // Get and assert the main config
    assert_eq!(main_lens.get(&config), config.main);

    // Lens for the port of DatabaseConfig
    let port_lens = LensImpl::<DatabaseConfig, Option<u16>>::new(
        |c| c.port.clone(),
        |c, v| c.port = v,
    );

    // Compose two lenses into a new lens focusing on config.main.port
    let composed_lens = main_lens.compose_lens_with_lens(port_lens);
    assert_eq!(composed_lens.get(&config), config.main.port);

    // Prism to focus on the Seconds variant of the Timespan enum
    let timespan_seconds_prism = PrismImpl::<Timespan, u32>::new(
        |c| match c {
            Timespan::Seconds(s) => Some(*s),
            _ => None,
        },
        |c, v| match c {
            Timespan::Seconds(_) => *c = Timespan::Seconds(v),
            _ => (),
        }
    );

    // Compose a Lens and a Prism
    let delay_seconds_prism = main_lens.compose_lens_with_prism(timespan_seconds_prism);
    assert_eq!(delay_seconds_prism.preview(&config), None);

    // Now set the value using a FallibleIso
    let delay_iso = FallibleIsoImpl::<Timespan, u16, String>::new(
        |c| match c {
            Timespan::Seconds(s) => u16::try_from(*s).map_err(|_| "Out of bounds".to_string()),
            _ => Err("Invalid Time".to_string()),
        },
        |c| Ok(Timespan::Seconds(*c as u32)),
    );

    let fallible_iso = delay_seconds_prism.compose_fallible_iso_with_fallible_iso::<String>(delay_iso);

    assert_eq!(fallible_iso.try_get(&config), Ok(11));
}
```

## Summary
This crate provides a robust and flexible way to work with nested data structures in Rust. By using optics like lenses, prisms, and isos, you can safely manipulate and combine parts of your data. The library is extensible and supports both fallible and non-fallible transformations, making it a powerful tool for functional-style programming.

