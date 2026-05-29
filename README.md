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

- Constructs `BigInt` from integers (`i64`), floats (`rhai::FLOAT`, truncated toward zero), and strings.
- Overloads standard arithmetic operators (`+`, `-`, `*`, `/`, `%`, `**`).
- Overloads unary negation (`-`).
- Overloads bitwise operators (`&`, `|`, `^`, `<<`, `>>`).
- Overloads comparison operators (`==`, `!=`, `>`, `>=`, `<`, `<=`).
- Converts `BigInt` back to a decimal string (`to_string`), hex string (`to_hex`), or float (`to_float`).
- Provides `to_bigint()` as a method on integers and floats for ergonomic conversion: `42.to_bigint()`, `1.5.to_bigint()`.
- Optional: Generate cryptographically secure random `BigInt`s by range or exact bit-length (via the `rand` feature).

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
rhai-bigint = "0.1.9"
```

### Feature Flags

*   `sync`: Enables `rhai/sync` support. Turn this on if your Rhai engine requires thread-safe types (e.g., when evaluating scripts across a Tokio thread pool).
*   `only_i32`: Passes `rhai/only_i32` through, making `rhai::INT` an `i32` instead of the default `i64`. All integer-accepting functions (`parse_bigint`, `to_bigint`, `**`, `<<`, `>>`) adapt automatically.
*   `metadata`: Enables `rhai/metadata`, which exposes function signature and documentation metadata on the Rhai `Engine`. Required if you want to call `engine.gen_fn_signatures()` or similar introspection APIs.
*   `rand`: Enables the generation of random `BigInt` values. Pulls in the `rand` crate and exposes the `rand_bigint` functions.

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
        let a = parse_bigint("1500000000000000000"); // 1.5 ETH
        let b = parse_bigint("500000000000000000");  // 0.5 ETH
        
        let sum = a + b;
        
        sum > parse_bigint("1900000000000000000") // evaluates to true
    "#;

    let result: bool = engine.eval(script).unwrap();
    assert!(result);
}
```

### 2. Scripting Examples

Once registered, your users can write natural, ergonomic scripts.

#### Basic Arithmetic
```js
let a = parse_bigint(42);                              // from integer
let b = parse_bigint("100000000000000000000000000000"); // from string
// let a = parse_bigint(1.5);  // from float — truncates toward zero, so this equals 1

let sum = a + b;
let diff = b - a;
let prod = a * b;
let quotient = b / a;
let remainder = b % a;
let power = a ** 3;    // exponentiation — exponent must be a non-negative integer
let negative = -a;

// Bitwise operators (two's complement semantics)
let and_result  = a & parse_bigint(0xFF);
let or_result   = a | parse_bigint(0xFF);
let xor_result  = a ^ parse_bigint(0xFF);
let left_shift  = a << 8;   // shift amount must be a non-negative integer
let right_shift = a >> 2;
```

#### Comparisons
```js
let price = parse_bigint("2000000000000000000");
let threshold = parse_bigint("1500000000000000000");

if price >= threshold {
    print("Threshold met!");
}
```

#### Random Number Generation (`rand` feature)

```js
// Generate a random, positive number of exactly 256 bits
let private_key = rand_bigint(256);

// Generate a random number within a specific range [min, max)
let min = parse_bigint(100);
let max = parse_bigint("1000000000000000000");
let random_amount = rand_bigint(min, max);
```

#### Cross-type comparisons are errors

All six comparison operators (`==`, `!=`, `<`, `<=`, `>`, `>=`) raise a runtime error
when one operand is a `BigInt` and the other is any other type (`int`, `float`, `string`,
`bool`). This prevents subtle bugs where a mismatched comparison silently returns `false`
without any indication that something is wrong. The error fires regardless of which side
the `BigInt` is on.

**int**
```js
// ❌ All operators raise a runtime error
parse_bigint(42) == 42
parse_bigint(42) != 42
parse_bigint(42) <  42
parse_bigint(42) >= 42
42 == parse_bigint(42)   // right-hand side equally affected

// ✅ Wrap the int first
parse_bigint(42) == parse_bigint(42)
parse_bigint(100) > parse_bigint(42)
```

**float**
```js
// ❌ Runtime error for every operator
parse_bigint(42) == 3.14
parse_bigint(42) <  3.14

// ✅ Convert the float to BigInt first via to_bigint() (truncates toward zero)
3.14.to_bigint() == parse_bigint(3)
parse_bigint(10) > (2.9).to_bigint()
```

**string**
```js
// ❌ Runtime error for every operator
parse_bigint(42) == "42"
parse_bigint(42) <  "42"

// ✅ Parse both sides first
parse_bigint("42") == parse_bigint("42")
parse_bigint("100") > parse_bigint("42")
```

**bool**
```js
// ❌ Runtime error — bool and BigInt are incompatible
parse_bigint(1) == true

// ✅ Express the intent explicitly with a BigInt comparison
parse_bigint(1) != parse_bigint(0)   // "is non-zero?"
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
let script = "balance > parse_bigint(100)";
let is_rich: bool = engine.eval_with_scope(&mut scope, script).unwrap();
```

## Related Crates

- **[rhai-evm](https://crates.io/crates/rhai-evm)** — Complements `rhai-bigint` with EVM-specific helpers: denomination constructors (`ether()`, `gwei()`, `usdc()`), Keccak-256 hashing, EIP-55 address utilities, and lossless conversion from `alloy-primitives` types (`U256`, `I256`) into `BigInt`.

## License

* MIT license (http://opensource.org/licenses/MIT)
