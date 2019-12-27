use crate::error::{Error, Result};
use crate::CurrencyAmount;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

/// Tuple struct used to define an amount with an exponent.
/// Useful when used in Mul/Div operations:
///
/// ```
/// use monet::Exponent;
///
/// assert_eq!(Exponent::new(1_000.into(), 2), Exponent::new(10.into(), 0));
///
/// ```
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Exponent {
    pub amount: CurrencyAmount,
    pub exponent: u8,
}

impl Exponent {
    pub fn new(amount: CurrencyAmount, exponent: u8) -> Self {
        Exponent { amount, exponent }
    }
}

impl PartialEq for Exponent {
    /// Compares two `Exponent`s.
    ///
    /// # Warning!
    ///
    /// Two might result equal due to precision losses.
    fn eq(&self, other: &Exponent) -> bool {
        (self.amount / 10i128.pow(u32::from(self.exponent)).into())
            == (other.amount / (10i128).pow(u32::from(other.exponent)).into())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Rates {
    map: HashMap<CurrencyCode, CurrencyAmount>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct CurrencyCode {
    code: [u8; 3],
}

impl std::ops::Deref for CurrencyCode {
    type Target = [u8; 3];

    fn deref(&self) -> &Self::Target {
        &self.code
    }
}

impl Rates {
    /// Creates a new Rates struct and populates it from
    pub fn new() -> Self {
        Rates::default()
    }

    // pub fn populate(&mut self) -> Result<(), reqwest::Error> {
    //     //reqwest::get(reqwest::Url::parse_with_params("https://openexchangerates.org/api/latest.json", &[("app_id", )]))

    //     self.map.insert("USD".try_into().unwrap(), 1_000_000u128);
    //     self.map.insert("CHF".try_into().unwrap(), 1_100_000u128);

    //     Ok(())
    // }

    /// Construct a Rates struct with given rates.
    pub fn with_rates(map: HashMap<CurrencyCode, CurrencyAmount>) -> Self {
        Rates { map }
    }

    /// Get the worth of a currency as an `CurrencyAmount`.
    /// The `worth` could be seen as "how many base units are needed to make one of this".
    /// If a USD is worth `1_000_000` and a CHF is worth `2_000_000`, that means that 2 USD are
    /// needed to make 1 CHF.
    pub fn worth(&self, code: CurrencyCode) -> Result<CurrencyAmount> {
        self.map
            .get(&code)
            .copied()
            .ok_or(Error::RateNotFound(code))
    }
}

impl<'s> TryFrom<&'s str> for CurrencyCode {
    type Error = Error;

    fn try_from(s: &'s str) -> Result<Self> {
        if s.len() != 3 {
            Err(Error::MalformedCode(s.into()))
        } else {
            let bytes = s.as_bytes();
            Ok(CurrencyCode {
                code: [bytes[0], bytes[1], bytes[2]],
            })
        }
    }
}

impl<'a> TryFrom<&'a CurrencyCode> for &'a str {
    type Error = std::str::Utf8Error;
    fn try_from(code: &'a CurrencyCode) -> std::result::Result<Self, Self::Error> {
        std::str::from_utf8(&code[..])
    }
}

impl FromStr for CurrencyCode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::try_from(s)
    }
}
