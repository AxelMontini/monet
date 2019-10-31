# Monet

[![Build Status](https://github.com/AxelMontini/monet/workflows/Rust/badge.svg)](https://github.com/AxelMontini/monet/actions)
[![MPL-2.0 Licensed](https://img.shields.io/crates/l/monet/0.1.0)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/monet)](https://crates.io/crates/monet)
[![Docs](https://docs.rs/monet/badge.svg)](https://docs.rs/crate/monet/)

Handle currency conversion and common operations (addition, subtraction, multiplication, division).

## How it works

It defines some base types:

* `CurrencyAmount` is `i128` and, as the name says, is used to store amounts.
* `CurrencyCode` is a wrapper for `[u8; 3]` that can be created from / converted into `&str`s.
* `Money` is the most important type. Money holds both a `currency_code` and an `amount`. It's used
  to store money and to perform operations. It can be converted into another currency code by providing
  `Rates`.
* `Rates` is a wrapper for a `HashMap`. It can be constructed from such pre-defined map or ~~populated from
  an external source such as websites~~ (Not yet, TODO: Implement).
* `Exponent` exists because there are no `float`s involved here. It has two fields: `amount` and `exponent`. Its decimal value is `amount / (10).pow(exponent)`.

## Dangers

Even this library isn't safe from precision losses. For example, an `Exponent`'s amount could be cut out by its exponent. Also errors when converting money could occurr.

## Examples

### Summing two `Money`s of the same type

```rust

use monet::{Money, CurrencyAmount, Rates, Operation};
use std::convert::TryInto;

// Custom rates.
let map = vec![("USD", 1_000_000)].into_iter()
    .map(|(code, worth)| (code.try_into().unwrap(), worth.into())
    .collect();
let rates = Rates::with_rates(map);

let money_owned = Money::with_str_code(CurrencyAmount::with_unit(2), "USD").unwrap();
let money_paid = Money::with_str_code(CurrencyAmount::with_unit(1), "USD").unwrap();

let remaining = (money_owned - money_paid).execute(&rates);

assert_eq!(remaining, Money::with_str_code(CurrencyAmount::with_unit(1), "USD"));
assert_eq!(remaining, Money::with_str_code(1_000_000.into(), "USD"));


```

### Summing two `Money`s of different type

```rust

use monet::{Money, CurrencyAmount, Rates, Operation};
use std::convert::TryInto;

// Custom rates.
let map = vec![
    ("USD", 1_000_000),
    ("CHF", 1_100_000),
].into_iter()
    .map(|(code, worth)| (code.try_into().unwrap(), worth.into())
    .collect();

let rates = Rates::with_rates(map);

let money_one = Money::with_str_code(1_000_000.into(), "CHF").unwrap();
let money_two = Money::with_str_code(1_100_000.into(), "USD").unwrap();

// Note: sum has currency code "CHF": when summing,
// the currency code of the first money is used
let sum = (money_one + money_two).execute(&rates);

assert_eq!(remaining, Money::with_str_code(2_000_000.into(), "CHF"));

```

### Chaining operations

```rust

use monet::{Money, CurrencyAmount, Rates, Operation};
use std::convert::TryInto;

// Custom rates.
let map = vec![
    ("USD", 1_000_000),
    ("CHF", 1_100_000),
].into_iter()
    .map(|(code, worth)| (code.try_into().unwrap(), worth.into())
    .collect();

let rates = Rates::with_rates(map);

let money_one = Money::with_str_code(1_000_000.into(), "CHF").unwrap();
let money_two = Money::with_str_code(1_100_000.into(), "USD").unwrap();
let money_three = Money::with_str_code(2_000_000.into(), "CHF").unwrap();

// Note: sum has currency code "CHF"
let sum = (money_one + money_two - money_three).execute(&rates);

assert_eq!(remaining, Money::with_str_code(0.into(), "CHF"));

```
