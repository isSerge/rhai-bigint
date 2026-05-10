# rhai-bigint

[![Crates.io](https://img.shields.io/crates/v/rhai-bigint.svg)](https://crates.io/crates/rhai-bigint)
[![Docs.rs](https://docs.rs/rhai-bigint/badge.svg)](https://docs.rs/rhai-bigint)
[![License](https://img.shields.io/crates/l/rhai-bigint.svg)](https://crates.io/crates/rhai-bigint)
[![Build Status](https://img.shields.io/github/actions/workflow/status/isSerge/rhai-bigint/ci.yml?branch=main)](https://github.com/isSerge/rhai-bigint/actions)

A [Rhai](https://rhai.rs/) scripting engine plugin providing seamless, arbitrary-precision BigInt arithmetic.

## Why this exists

By default, the Rhai scripting engine limits integers to 64-bit (or 128-bit via feature flags). When building financial applications, cryptography tools, or Web3 indexers, values frequently exceed these limits (e.g., 1 ETH = $10^{18}$ Wei). 

Naively casting these large numbers to floating-point (`f64`) results in catastrophic precision loss (IEEE 754 limits). Casting them to strings preserves the value, but breaks the ability to use native math operators inside the user's scripts.

`rhai-bigint` solves this by injecting `num_bigint::BigInt` directly into the Rhai memory space and overloading the standard math and comparison operators. Users get the native ergonomics of standard operators, while the engine guarantees 100% lossless precision under the hood.

## Features

- Constructs `BigInt` from integers (`i64`), floats (`f64`, truncated toward zero), and strings.
- Overloads standard arithmetic operators (`+`, `-`, `*`, `/`, `%`).
- Overloads unary negation (`-`).
- Overloads comparison operators (`==`, `!=`, `>`, `>=`, `<`, `<=`).

## Installation

### Add via Cargo

```bash
cargo add rhai-bigint
```

### Manual Configuration

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rhai = "1.22.2"
rhai-bigint = "0.1.1"
```

### Feature Flags

*   `sync`: Enables `rhai/sync` support. Turn this on if your Rhai engine requires thread-safe types (e.g., when evaluating scripts across a Tokio thread pool).

## Usage

### 1. Registering the Package in Rust

Using the plugin is as simple as registering the package with your Rhai `Engine`.

```rust
use rhai::Engine;
use rhai::packages::Package;
use rhai_bigint::BigIntPackage;

fn main() {
    let mut engine = Engine::new();
    
    // Register the package into the engine
    BigIntPackage::new().register_into_engine(&mut engine);

    // Now your scripts can seamlessly handle massive numbers!
    let script = r#"
        let a = bigint("1500000000000000000"); // 1.5 ETH
        let b = bigint("500000000000000000");  // 0.5 ETH
        
        let sum = a + b;
        
        sum > bigint("1900000000000000000") // evaluates to true
    "#;

    let result: bool = engine.eval(script).unwrap();
    assert!(result);
}
```

### 2. Scripting Examples

Once registered, your users can write natural, ergonomic scripts.

#### Basic Arithmetic
```js
let a = bigint(42);          // from integer
let a = bigint(1.5);         // from float — truncates to 1
let b = bigint("100000000000000000000000000000"); // from string

let sum = a + b;
let diff = b - a;
let prod = a * b;
let quotient = b / a;
let remainder = b % a;
let negative = -a;
```

#### Comparisons
```js
let price = bigint("2000000000000000000");
let threshold = bigint("1500000000000000000");

if price >= threshold {
    print("Threshold met!");
}
```

## Bridging Rust and Rhai

If you are writing a host application and need to inject a `BigInt` into a script's Scope, you can use the `Dynamic` wrapper:

```rust
use rhai::{Engine, Scope, Dynamic, packages::Package};
use num_bigint::BigInt;
use rhai_bigint::BigIntPackage;

let mut engine = Engine::new();
BigIntPackage::new().register_into_engine(&mut engine);

let mut scope = Scope::new();

// Inject a massive number from Rust into the Rhai scope
let my_rust_bigint = BigInt::parse_bytes(b"999999999999999999999999", 10).unwrap();
scope.push("balance", Dynamic::from(my_rust_bigint));

// The script can interact with it natively
let script = "balance > bigint(100)";
let is_rich: bool = engine.eval_with_scope(&mut scope, script).unwrap();
```

## License

* MIT license (http://opensource.org/licenses/MIT)
