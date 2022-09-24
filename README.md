# WA

> A modern Rust utility library.
>
> Inspired by JavaScript Lodash library.
>

No dependencies. 

- Made for Rust
- Made for Web Assembly (check `webassembly` folder)
- Made for [Foreign function interface](https://en.wikipedia.org/wiki/Foreign_function_interface)

## Installation

Add `wa` to your dependencies on `Cargo.toml`. You need to set the features that will be used as well:

```shell
wa = { version = "0.2.0", features = ["string", "vector", "array"] }
```

## Why?

Wa makes Rust easier by taking the hassle out of working with arrays, numbers, objects, strings, etc. Wa’s modular methods are great for:

- Iterating arrays, objects, & strings
- Manipulating & testing values
- Creating composite functions
