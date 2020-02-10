# Monet

[![Build Status](https://github.com/AxelMontini/monet/workflows/Rust/badge.svg)](https://github.com/AxelMontini/monet/actions)
[![MPL-2.0 Licensed](https://img.shields.io/crates/l/monet)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/monet)](https://crates.io/crates/monet)
[![Docs](https://docs.rs/monet/badge.svg)](https://docs.rs/crate/monet/)

Handle currency conversion and common operations (addition, subtraction, multiplication, division).

## Complete rewrite

Everything that has been written in versions older than 2.0 is now deprecated. I wasn't happy with how it turned out.
Too many allocations, errors would only pop out at runtime, pretty weak implementation in general
(I couldn't sum an amount of `Money`s only known at runtime).

For this reason I've decided to reimplement everything from scratch, with new goals and values in mind.

### Goals

Provide a library to handle currency painlessly:

- Compile time error checking.
    - Cannot sum different currencies without explicitly converting them.
    - Invalid currency codes.
    - //TODO: Add extra useful stuff.
- Compile time currency definition.
    - Parse currency definitions at compile time.
- Serde support, to store Moneys in a reliable format.
