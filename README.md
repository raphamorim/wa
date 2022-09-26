# WA

> A modern Rust, WebAssembly, FFI utility library.
>
> Inspired by JavaScript Lodash library.
>

No dependencies. 

- Made for Rust
- Made for WebAssembly (check `wa-sm/wasm` folder)
- Made for JavaScript and NodeJs (check `wa-sm` folder)
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

## Features

- `string` (Examples: `cargo run --example strings`)
- `vector` (Examples: `cargo run --example vectors`)
- `array` (Examples: `cargo run --example arrays`)
- `util` (Examples: `cargo run --example utils`)
- `wasm` (Examples: `cargo run --example webassembly`)

### String

`#[cfg(feature = "string")]`

#### template_vec

(String) Creates a compiled template function that can interpolate data properties in "interpolate" delimiters using vector

Example

```rust
use wa::string::template_vec;

let url_template = template_vec("https://api.com/{1}/products/{2}?by={1}".to_string());

let url: String = url_template(vec!["85", "23"]); // "https://api.com/85/products/23?by=85"
let url_2: String = url_template(vec!["23", "85"]); // "https://api.com/23/products/85?by=23"
```

#### template

(String) Creates a compiled template function that can interpolate data properties in "interpolate" delimiters

Example:

```rust
use std::collections::HashMap;
use wa::string::template;

let url_template = template("https://api.com/{ user_id }/products/{ product_id }".to_string());
let url: String = url_template(HashMap::from([
    ("user_id", "85"),
   ("product_id", "23"),
]));

assert_eq!(url, "https://api.com/85/products/23"); // true
```

#### replace_extended_ascii

(String) replace extended ASCII codes to standards.

Example:

```rust
use wa::string::replace_extended_ascii;

assert_eq!(replace_extended_ascii("São Paulo".to_string()), "Sao Paulo");
assert_eq!(replace_extended_ascii("Água".to_string()), "Agua");
assert_eq!(replace_extended_ascii("Pão".to_string()), "Pao");
assert_eq!(replace_extended_ascii("Åke".to_string()), "Ake");
assert_eq!(replace_extended_ascii("Södermalm".to_string()), "Sodermalm");
assert_eq!(replace_extended_ascii("Rio de Janeiro".to_string()), "Rio de Janeiro");
```

#### camel_case

(String) Converts the input string into Camel-Case format.

Note: All special characters will be removed and only valid letters remained.

To replace extended ASCII codes to standards, e.g: "ã" to "a". Check `wa::string::replace_extended_ascii`.

Example

```rust
use wa::string::camel_case;

camel_case("São Paulo".to_string()); // "sãoPaulo"
camel_case("____Rio____de___JANEIRO".to_string()); // "rioDeJaneiro"
camel_case("Rio de Janeiro".to_string()); // "rioDeJaneiro"
camel_case("--stock-----holm--".to_string()); // "stockHolm"
camel_case("--stock-----holm--".to_string()); // "stockHolm"
camel_case("Rio2DE2janeiro".to_string()); // "rio2de2janeiro"
```

#### kebab_case

(String) Converts the input string into Kebab-Case format.

Note: All the special characters will be removed and only valid letters remained.

To replace extended ASCII codes to standards, e.g: "ã" to "a". Check `wa::string::replace_extended_ascii`.

Example

```rust
use wa::string::kebab_case;

kebab_case("São Paulo".to_string()); // "são-paulo"
kebab_case("Rio de Janeiro".to_string()); // "foo-bar"
kebab_case("--rio--1--".to_string()); // "foo-bar"
kebab_case("__rIO_-de-Jan_eiro_".to_string()); // "f-oo-ba-r"
```

#### ends_with

(String) Checks if string ends with the given target string.
Note: If position is not provided, it will search through the whole string by default.

Example:

```rust
use wa::string::ends_with;
let is_ends_with = ends_with("abc".to_string(), "c".to_string(), None);
// true
let is_ends_with = ends_with("abc".to_string(), "b".to_string(), None);
// false
let is_ends_with = ends_with("abc".to_string(), "bc".to_string(), Some(2));
// false
```