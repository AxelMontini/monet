#![deny(warnings)]

mod currency;
mod error;
mod ops;

pub use currency::{CurrencyCode, Exponent, Rates};
pub use error::{Error, Result};
pub use ops::Operation;

use std::convert::TryInto;
use std::fmt;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

/// How much `amount` makes a unit
pub const AMOUNT_UNIT: i128 = 1_000_000;

/// Holds an amount of currency. The `i128` it holds is
/// expressed in fractions of a unit.
/// `CurrencyAmount(`[`AMOUNT_UNIT`](constant.AMOUNT_UNIT.html)`)` makes a unit.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct CurrencyAmount(i128);

impl CurrencyAmount {
    pub fn with_unit(unit: i128) -> Self {
        CurrencyAmount(unit * AMOUNT_UNIT)
    }

    pub fn with_tenths(tenths: i128) -> Self {
        CurrencyAmount(tenths * AMOUNT_UNIT / 10)
    }

    pub fn with_cents(cents: i128) -> Self {
        CurrencyAmount(cents * AMOUNT_UNIT / 100)
    }

    pub fn with_thousands(thousands: i128) -> Self {
        CurrencyAmount(thousands * AMOUNT_UNIT / 1000)
    }

    pub fn into_unit(self) -> Self {
        CurrencyAmount(self.0 / AMOUNT_UNIT)
    }

    pub fn into_tenths(self) -> Self {
        CurrencyAmount(self.0 * 10 / AMOUNT_UNIT)
    }

    pub fn into_cents(self) -> Self {
        CurrencyAmount(self.0 * 100 / AMOUNT_UNIT)
    }

    pub fn into_thousands(self) -> Self {
        CurrencyAmount(self.0 * 1000 / AMOUNT_UNIT)
    }
}

impl std::ops::Deref for CurrencyAmount {
    type Target = i128;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Add for CurrencyAmount {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        CurrencyAmount(self.0 + other.0)
    }
}

impl std::ops::Sub for CurrencyAmount {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        CurrencyAmount(self.0 - other.0)
    }
}

impl std::ops::Mul for CurrencyAmount {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        CurrencyAmount(self.0 * other.0)
    }
}

impl std::ops::Div for CurrencyAmount {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        CurrencyAmount(self.0 / other.0)
    }
}

impl From<i128> for CurrencyAmount {
    fn from(i: i128) -> Self {
        CurrencyAmount(i)
    }
}

impl From<CurrencyAmount> for i128 {
    fn from(amount: CurrencyAmount) -> Self {
        *amount
    }
}

/// A struct containing an `amount` of money having a certain `currency_code`.
/// Note that `amount` contains fractions of a unit. See [`AMOUNT_UNIT`](constant.AMOUNT_UNIT.html).
///
/// ## Examples
///
/// ```
///
/// use monet::{Money, CurrencyAmount, Rates, Operation};
/// use std::convert::TryInto;
///
/// // Custom rates.
/// let map = vec![("USD", 1_000_000)].into_iter()
///     .map(|(code, worth)| (code.parse().unwrap(), worth.into()))
///     .collect();
/// let rates = Rates::with_rates(map);
///
/// let money_owned = Money::with_str_code(CurrencyAmount::with_unit(2), "USD").unwrap();
/// let money_paid = Money::with_str_code(CurrencyAmount::with_unit(1), "USD").unwrap();
///
/// let remaining = (money_owned - money_paid).execute(&rates);
///
/// assert_eq!(remaining, Money::with_str_code(CurrencyAmount::with_unit(1), "USD"));
///
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Money {
    pub amount: CurrencyAmount,
    pub currency_code: CurrencyCode,
}

impl Money {
    pub fn new(amount: CurrencyAmount, currency_code: CurrencyCode) -> Self {
        Money {
            amount,
            currency_code,
        }
    }

    pub fn into_code(self, code: CurrencyCode, rates: &Rates) -> Result<Money> {
        let worth_self = rates.worth(self.currency_code)?;
        let worth_new = rates.worth(code)?;

        Ok(Money {
            amount: self.amount * worth_self / worth_new,
            currency_code: code,
        })
    }

    /// Creates `Money` with given amount and code. Returns `None` if the given code is not three characters long.
    pub fn with_str_code(amount: CurrencyAmount, currency_code: &str) -> Result<Money> {
        Ok(Money::new(amount, currency_code.parse()?))
    }

    /// Creates `Money` like `Money::with_str_code` does, but with cents instead of an amount
    ///  for the sake of a shorter function name.
    pub fn with_cents(cents: i128, currency_code: &str) -> Result<Money> {
        Money::with_str_code(CurrencyAmount::with_cents(cents), currency_code)
    }
}

/// Money can be displayed in the following format: `12.10 CHF`.
///
/// Default precision is dependent on the currency code (see ISO 4217 exponent).
/// A custom precision in range `0..=6` can be provided like this:
///
/// ```
///
/// use monet::Money;
///
/// let money = Money::with_str_code(12_100_000.into(), "CHF").unwrap();
///
/// assert_eq!(
///     &format!("{}", money),
///     "12.10 CHF"
/// );
///
/// assert_eq!(
///     &format!("{:.2}", money),
///     "12.10 CHF"
/// );
///
/// assert_eq!(
///     &format!("{:.6}", money),
///     "12.100000 CHF"
/// );
///
/// // Note: the formatted version has lost a decimal due to the
/// // lower precision
/// assert_eq!(
///     &format!("{:.0}", money),
///     "12 CHF"
/// );
///
/// ```
impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use iso4217::alpha3;

        let code: &str = (&self.currency_code).try_into().unwrap();
        let units = *self.amount / AMOUNT_UNIT;
        let decimals = *self.amount % AMOUNT_UNIT;
        let precision = f
            .precision()
            .unwrap_or_else(|| alpha3(code).unwrap().exp as usize);

        if precision > 0 {
            write!(
                f,
                "{units}.{decimals} {code}",
                units = units,
                decimals = decimals
                    .checked_div(AMOUNT_UNIT / 10i128.pow(precision as u32))
                    .ok_or(fmt::Error)?,
                code = code
            )
        } else {
            write!(f, "{units} {code}", units = units, code = code)
        }
    }
}

// pub trait CurrencyAmount: std::fmt::Debug + Clone + Copy + Eq + PartialEq + Default {}

// impl CurrencyAmount for u8 {}
// impl CurrencyAmount for u16 {}
// impl CurrencyAmount for u32 {}
// impl CurrencyAmount for u64 {}
// impl CurrencyAmount for u128 {}
// impl CurrencyAmount for i8 {}
// impl CurrencyAmount for i16 {}
// impl CurrencyAmount for i32 {}
// impl CurrencyAmount for i64 {}
// impl CurrencyAmount for i128 {}

// pub trait Currency {
//     const NAME: &'static str = std::any::type_name;
//     /// The exponent of this currency
//     const EXPONENT: i8;
//     const VALUE:
// }

#[cfg(test)]
fn rates() -> Rates {
    let map = vec![
        ("USD", 1_000_000),
        ("CHF", 1_100_000),
        ("EUR", 1_200_000),
        ("GBP", 1_500_000),
    ]
    .into_iter()
    .map(|(code, worth)| (code.try_into().unwrap(), worth.into()))
    .collect();
    Rates::with_rates(map)
}

#[cfg(test)]
mod tests {

    mod money {
        use crate::rates;
        use crate::CurrencyAmount;
        use crate::Money;

        #[cfg(feature = "serialize")]
        use serde::{Deserialize, Serialize};

        #[test]
        fn test_into_code() -> crate::Result<()> {
            let money_chf = Money::new(CurrencyAmount::with_unit(1_000_000), "CHF".parse()?);
            let money_usd = money_chf.into_code("USD".parse()?, &rates());

            assert_eq!(
                money_usd,
                Ok(Money::new(
                    CurrencyAmount::with_unit(1_100_000),
                    "USD".parse()?
                ))
            );

            Ok(())
        }

        #[cfg(feature = "serialize")]
        #[cfg_attr(feature = "serialize", test)]
        fn test_de_serialize() {
            fn deserialize<'de, D: Deserialize<'de>>() {}
            fn serialize<D: Serialize>() {}

            deserialize::<Money>();
            serialize::<Money>();
        }

        #[test]
        fn test_display() {
            let money = Money::with_str_code(CurrencyAmount::with_cents(2125), "CHF").unwrap();

            assert_eq!(format!("{}", money), "21.25 CHF".to_string());
            assert_eq!(format!("{:.2}", money), "21.25 CHF".to_string());
            assert_eq!(format!("{:.6}", money), "21.250000 CHF".to_string());
            assert_eq!(format!("{:.0}", money), "21 CHF".to_string());
        }

        #[test]
        #[should_panic]
        fn test_display_panic() {
            let money = Money::with_str_code(CurrencyAmount::with_cents(2125), "CHF").unwrap();
            let _formatted = format!("{:.8}", money);
        }
    }
}
