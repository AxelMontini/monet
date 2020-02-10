//! This example doesn't actually load stuff from a database,
//! but it shows how it should be done.

use monet::{Money, MoneyDynamic};

mod currency {
    monet::define_currency_toml!("monet/examples/currencies.toml");
}

fn main() {
    let good = price_list(false);
    assert!(good.is_ok());
    println!("Good Price list: {:?}", good);

    let bad = price_list(true);
    assert!(bad.is_err());
    println!("Bad Price list: {:?}", bad);
}

/// Loads a list of prices. We only accept CHF as currency and the database should provide only that,
/// but what if it doesn't? We must check and returns an error if the database returns another currency.
fn price_list<'a>(bad: bool) -> monet::ConvertResult<'a, Vec<Money<currency::CHF>>> {
    use std::convert::TryFrom;

    let dynamic = if bad {
        load_database_bad().into_iter()
    } else {
        load_database().into_iter()
    };

    // We can collect an iterator of Result<T, E> into a Result<Vec<T>, E>. If any error occurs, Err(E) is returned. Otherwise, Ok(Vec<T>) is.
    // This way we can try to convert all currencies and return an error if any fails.
    dynamic.map(Money::try_from).collect()
}

fn load_database<'a>() -> Vec<MoneyDynamic<'a>> {
    vec![MoneyDynamic::new(100, "CHF", 2), MoneyDynamic::new(1250, "CHF", 2), MoneyDynamic::new(390, "CHF", 2)]
}

fn load_database_bad<'a>() -> Vec<MoneyDynamic<'a>> {
    vec![MoneyDynamic::new(100, "USD", 2), MoneyDynamic::new(1250, "CHF", 2), MoneyDynamic::new(390, "CHF", 2)]
}