# optics-rs

# MAJOR REFACTOR IN PROGRESS!
  -  Code refactor completed, docs still out of sync.


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

### 📌 Status

This is a **pre-release**, and the code is **unfinished** — but it’s good enough to start experimenting with in real projects.

There’s a lot of room for simplification and improvement. Type-level constraints, trait bounds, and generic compositions are kind of bloated right now, and I wouldn’t mind help tightening it up.

### ✨ Features
- No dependencies — pure Rust, no external crates except for testing
- `no_std` support — usable in embedded and other restricted environments
- Type-safe, explicit interfaces


### 🧠 Philosophy

This is a **layman's implementation** of optics. I don’t fully grasp all the deep type theory behind
profunctor optics or Van Laarhoven lenses. Instead, I built something practical and composable,
within the limitations of Rust’s type system and my own understanding.

### 📦 Composability

All optic implementations implement a set of base traits that define the operations they can perform. Currently three base operations are defined: (`HasSetter`, `HasGetter`, `HasReverseGet`). This will likely be extended in the future to make it possible to add a `Traversal` for example.

Concrete structs of implementations of the optics are private, and interaction with optics is only allowed when wrapped in an exposed `Impl` struct (constructor functions returning `Impl` are exposed). This can be used to combine optics or to downgrade an optic, such as a `Lens` into a `Getter`, if the desired behaviour is to restrict the optic to only allow reading data. 

Optics - even if they are of different types can be combined. The rule of thumb is that the combination of two optics X<S, I> and Y<I, A> will result in the most advanced optic type that requires base traits that both components implement: 

|               | `PartialGetter` | Getter        | Prism | Lens | Iso | `FallibleIso` | Setter |
|:--------------|:---------------|:--------------|:--------|:------|:-----|:--------------|:-------|
| **`PartialGetter`**      | `PartialGetter` | `PartialGetter` | `PartialGetter` | `PartialGetter` | `PartialGetter` | `PartialGetter` | -      |
| **Getter**             | `PartialGetter` | Getter        | `PartialGetter` | Getter        | Getter        | `PartialGetter` | -      |
| **Prism**              | `PartialGetter` | `PartialGetter` | Prism         | Prism         | Prism         | Prism         | -      |
| **Lens**               | `PartialGetter` | Getter        | Prism         | Lens          | Lens          | Prism         | -      |
| **Iso**                | `PartialGetter` | Getter        | Prism         | Lens          | Iso           | `FallibleIso`   | -      |
| **`FallibleIso`**        | `PartialGetter` | `PartialGetter` | Prism         | Prism         | `FallibleIso`   | `FallibleIso`   | -      |
| **Setter**             | -             | -             | Setter        | Setter        | Setter        | Setter        | -      |

### 🔎 Implemented optic types
- [`PartialGetter`] - for fallible read-only access to data
- [`Getter`] - for read-only access to data
- [`Setter`] - for write-only access to data
- [`Prism`] — mainly for working with enum variants (e.g. `SocketAddr` -> `SocketAddrV4`)
- [`Lens`] — mainly for focusing on subfields of structs (e.g. `Point` -> `x: u32`)
- [`Iso`]morphisms — for bijective, invertible mappings (eg. `Ipv4` ↔ `u32`)
- [`FallibleIso`]morphisms — for conversions that might fail (e.g., `String` ↔ `u16`)

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

### 🧩 Extensibility

The crate was designed in a way that allows for easy extensibility. Both in terms of adding new optic types (w/ base traits), or adding new implementations of existing optics, such as a lens that can handle Options of any type.

## Optics Crate Architecture and Implementation Conventions
This section outlines the structural conventions and design patterns employed in the optics crate. Adhering to these guidelines ensures consistency, maintainability, and extensibility across the crate's codebase, and is to some extent enforced by tests.

### Module Structure
Each optic type (e.g., Lens, Prism, Iso) is encapsulated within its own module. These modules are not directly exposed to downstream users. The organization within each module is as follows:

#### Marker Trait
A marker trait is defined to represent the optic type. This trait defines as supertraits the necessary base optic traits required for its functionality. For example, a Prism marker trait would extend `HasPartialGetter` and `HasSetter`. The marker trait has a blanket implementation to be automatically implement for all structs that implement the required base traits.

#### Implementation Struct (Impl)
An Impl struct (e.g., `LensImpl`) serves as the public interface for the optic type. The semantics of the Impl wrapper is "a container of an optics that currently acts as a ..." This struct wraps the concrete implementation opaquely and is directly returned by the crate's API. Utilizing a concrete struct also allows to use correct combinating function signatures and allow downgrading an optic to an inferior type (lens to prism, iso to getter).

The Impl struct is responsible for:
- Implementing all base optic traits its wrapped optic implements, and allowing for casting between different optic types (e.g., from Iso to Lens or Prism).
- Providing `combine_with_xxx` functions to compose optics, returning an Impl of the resulting optic type.
- Providing `cast_to_xxx` functions returning an Impl of an inferior optic type (e.g., from Lens to Prism or Getter).

#### Composed Implementations
A `composed.rs` file within each module contains implementations that compose two optics to form the current optic type. For instance, a `ComposedPrism` might combine a `Lens` and a `FallibleIso`. In some cases errors need to be wrapped either automatically if they implement `Into<>`, or by mapping functions.

The module is entirely private to the crate, only a constructor function `new` is exposed.

#### Mapped Implementations

Though not strictly required, currently all optics provide an implementation using closures.

The modules are entirely private to the crate, only a constructor function `new` is exposed. 

#### Additional Implementations
Other files within the module may provide alternative implementations of the optic type, such as concrete structs for specific higher-kinded types (HKTs) that cannot be expressed with closures alone, such as Some<T>, Result<T, Err>, and Vec<T>. Due to Rust's lack of native support for HKTs, it's not possible to implement a general Functor<_> trait as in Haskell. Consequently, each mapped implementation can only be tailored to a specific type constructor only.

These modules are also intended to be private, only exposing a constructor function returning an `Impl` struct. If you add new implementations either to the crate or to your own crate, please follow this guideline to avoid gotchas.

### Guidelines for Adding New Optic Types or Implementations

When introducing a new optic type or implementation:
- Create a New Module: Define a new module for the optic type, following the naming convention (e.g., lens, prism).
- Define the Marker Trait: Inside the module, define a marker trait that extends the appropriate base optic traits. Add a blanket implementation for all structs that implement the required base traits.
- Implement the Optic: Provide a concrete struct that implements the base optic traits and the marker trait. Add it as a submodule under the optic type module.
- Wrap with Impl Struct: Create an Impl struct in a wrapper module that wraps the concrete implementation and implements downgrading and combining functions.
- Compose Optics: If the optic type can be composed **from** existing optics, implement the composition in the composed.rs file.
- Additional Implementations: Consider alternative implementations using closures or concrete structs for specific HKTs, as needed.
- Pull requests are welcome :)

By following these conventions, the optics crate maintains a consistent and extensible framework for optic types, promoting code reuse and reducing the potential for conflicts in trait implementations.


|               | `Getter` | `TotalGetter` | `Prism` | `Lens` | `Iso` | `FallibleIso` | `Setter` |
|---------------|:-------------:|:-----------:|:-----:|:----:|:---:|:-----------:|:------:|
| `Getter`      |      ✓       |             |       |      |     |             |        |
| `TotalGetter`  |      ✓       |      ✓      |       |      |     |             |        |
| `Prism`         |      ✓       |             |   ✓   |      |     |             |   ✓    |
| `Lens`          |      ✓       |      ✓      |   ✓   |  ✓   |     |             |   ✓    |
| `Iso`           |      ✓       |      ✓      |   ✓   |  ✓   |  ✓  |     ✓       |   ✓    |
| `FallibleIso` |      ✓       |             |   ✓   |      |     |      ✓      |   ✓    |
| `Setter`        |              |             |       |      |     |             |   ✓    |


### 💬 Call for Critics

If you know your type theory, or even if you just have an eye for clean Rust APIs — I’d love for you to take a look. Suggestions, critiques, and even teardown reviews are welcome.
This is very much a **learning-while-doing** project for me.


## Examples
Below is a simplified example of how the optics work in this crate. The code below illustrates how to use and combine the various optic types.

```rust
use optics::{LensImpl, FallibleIsoImpl, PrismImpl, mapped_lens, mapped_prism, mapped_fallible_iso, HasSetter, HasGetter};

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

fn example() {
  // Define lenses to focus on subfields
  let http_lens = mapped_lens(
    |app: &AppConfig| app.http.clone(),
    |app, http| app.http = http,
  );

  let bind_address_prism = mapped_prism(
    |http: &HttpConfig| http.bind_address.clone().ok_or(()),
    |http, addr| http.bind_address = Some(addr),
  );

  let minimum_port = 1024;
  // Define a fallible isomorphism between String and u16 (parsing a port)
  let port_fallible_iso = mapped_fallible_iso(
    |addr: &String| {
      addr.rsplit(':')
        .next()
        .and_then(|port| port.parse::<u16>().ok()).ok_or(())
    },
    move |port: &u16| if *port > minimum_port { Ok(format!("0.0.0.0:{}", port)) } else { Err(()) }
  );

  // Compose lens and fallible iso into a ComposedFallibleIso

  let http_bind_address_prism = http_lens.compose_with_prism(bind_address_prism);
  let http_bind_address_port_prism = http_bind_address_prism.compose_with_fallible_iso::<(), _, _>(port_fallible_iso);

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

example();
```

### Disclaimer:

While the code was written with care, parts of the documentation and the tests are AI generated.
