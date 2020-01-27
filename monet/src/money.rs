use monet_traits::Currency;
use std::fmt;

#[cfg(feature = "u128")]
type Amount = u128;
#[cfg(not(feature = "u128"))]
type Amount = u64;

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub struct Money<'c, C: Currency<'c>> {
    pub amount: Amount,
    _phantom: std::marker::PhantomData<&'c C>,
}

impl<'c, C: Currency<'c>> Money<'c, C> {
    pub fn with_amount(amount: Amount) -> Self {
        Self {
            amount,
            _phantom: std::marker::PhantomData::<&'c C>,
        }
    }
}

impl<'c, C: Currency<'c>> fmt::Display for Money<'c, C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let code = C::CODE;
        let precision = C::UNITS as u32;

        if precision == 0 {
            write!(f, "{code} {units}", code = code, units = self.amount)
        } else {
            let units = self.amount / 10u128.pow(precision);
            let decimals_short = format!("{}", self.amount % 10u128.pow(precision));
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
