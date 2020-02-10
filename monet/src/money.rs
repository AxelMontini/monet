use monet_traits::Currency;
use std::convert::TryFrom;
use std::fmt;

#[cfg(feature = "i128")]
type Amount = i128;
#[cfg(not(feature = "i128"))]
type Amount = i64;

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub struct Money<C: Currency> {
    pub amount: Amount,
    _phantom: std::marker::PhantomData<C>,
}

impl<C: Currency> Money<C> {
    pub fn with_amount(amount: Amount) -> Self {
        Self {
            amount,
            _phantom: Default::default(),
        }
    }
}

impl<C: Currency> fmt::Display for Money<C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let code = C::CODE;
        let precision = C::UNITS as u32;

        if precision == 0 {
            write!(f, "{code} {units}", code = code, units = self.amount)
        } else {
            let units = self.amount / 10i128.pow(precision);
            let decimals_short = format!("{}", self.amount % 10i128.pow(precision));
            let mut decimals: String = std::iter::repeat("0")
                .take(precision as usize - decimals_short.len())
                .collect();
            decimals.push_str(&decimals_short);

            let mut units_width = 0u32;
            loop {
                let i = units / 10i128.pow(units_width);

                // negative numbers would create and infinite loop, and that's not good
                if i.abs() >= 10 {
                    units_width += 1;
                } else {
                    break;
                }
            }

            // The "-" is an extra character
            if units.signum() == -1 {
                units_width += 1;
            }

            write!(
                f,
                "{code} {units}.{decimals:width$}",
                code = code,
                units = units,
                decimals = decimals,
                width = f
                    .width()
                    .map(|width| width - code.len() - 2 - units_width as usize - precision as usize)
                    .unwrap_or(precision as usize)
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoneyDynamic<'a> {
    pub amount: Amount,
    // currency_name: &'a str,
    pub currency_code: &'a str,
    pub currency_units: u8,
}

impl<'a> fmt::Display for MoneyDynamic<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let code = self.currency_code;
        let precision = self.currency_units as u32;

        if precision == 0 {
            write!(f, "{code} {units}", code = code, units = self.amount)
        } else {
            let units = self.amount / 10i128.pow(precision);
            let decimals_short = format!("{}", self.amount % 10i128.pow(precision));
            let mut decimals: String = std::iter::repeat("0")
                .take(precision as usize - decimals_short.len())
                .collect();
            decimals.push_str(&decimals_short);

            write!(
                f,
                concat!("{code} {units}.{decimals}"),
                code = code,
                units = units,
                decimals = decimals
            )
        }
    }
}

impl<'a> MoneyDynamic<'a> {
    pub fn new(amount: Amount, code: &'a str, units: u8) -> Self {
        Self {
            amount,
            currency_code: code,
            currency_units: units,
        }
    }
}

impl<'a, C: Currency> TryFrom<MoneyDynamic<'a>> for Money<C> {
    type Error = crate::ConvertError<'a>;

    fn try_from(money_dynamic: MoneyDynamic<'a>) -> crate::ConvertResult<Self> {
        if C::CODE == money_dynamic.currency_code {
            Ok(Money::with_amount(money_dynamic.amount))
        } else {
            Err(crate::ConvertError::DifferentCurrency(
                money_dynamic,
                std::any::type_name::<C>(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Amount;
    use crate::{Money, MoneyDynamic};

    mod currency {
        crate::define_currency_array!([("US Dollar", "USD", 2)]);
    }

    #[test]
    fn fmt_display_money() {
        let money = Money::<currency::USD>::with_amount(100);
        assert_eq!(format!("{}", money), "USD 1.00".to_string());
    }

    #[test]
    fn size_of_money() {
        let money = Money::<currency::USD>::with_amount(100);
        assert_eq!(std::mem::size_of::<Amount>(), std::mem::size_of_val(&money));
    }

    #[test]
    fn fmt_display_money_dynamic() {
        let money = MoneyDynamic::new(100, "EUR", 2);
        assert_eq!(format!("{}", money), "EUR 1.00".to_string());
    }

    #[test]
    fn try_from() {
        use std::convert::TryFrom;

        let dynamic = MoneyDynamic::new(100, "USD", 2);
        let non_dynamic: Money<currency::USD> = Money::try_from(dynamic.clone()).unwrap();

        assert_eq!(format!("{}", dynamic), format!("{}", non_dynamic));
    }

    #[test]
    fn try_from_panic() {
        use std::convert::TryFrom;

        let dynamic = MoneyDynamic::new(100, "CHF", 2);
        let err = Money::<currency::USD>::try_from(dynamic.clone());

        assert_eq!(
            Err(crate::ConvertError::DifferentCurrency(
                dynamic,
                std::any::type_name::<currency::USD>()
            )),
            err
        );
    }
}
