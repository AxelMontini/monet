#![deny(warnings)]

mod ops;

mod money;
pub use money::Money;

pub use monet_traits::Currency;

pub use monet_derive::*;

#[cfg(test)]
mod tests {
    use crate::{Currency, Money};

    pub struct USD;

    impl Currency<'static> for USD {
        const UNITS: u8 = 2;

        const CODE: &'static str = "USD";

        const NAME: &'static str = "US Dollar";
    }

    #[test]
    fn display() {
        let money = Money::<USD>::with_amount(100);
        assert_eq!(format!("{}", money), "USD 1.00".to_string());
    }
}
