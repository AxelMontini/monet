use super::Currency;
use super::{Money, MoneyDynamic};
use std::ops::{Add, Sub};

impl<C: Currency> Add for Money<C> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self.amount += other.amount;
        self
    }
}

impl<C: Currency> Sub for Money<C> {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        self.amount -= other.amount;
        self
    }
}

impl<'r, C: Currency> Add<&'r Self> for Money<C> {
    type Output = Self;

    fn add(mut self, other: &'r Self) -> Self::Output {
        self.amount += other.amount;
        self
    }
}

impl<'r, C: Currency> Sub<&'r Self> for Money<C> {
    type Output = Self;

    fn sub(mut self, other: &'r Self) -> Self::Output {
        self.amount -= other.amount;
        self
    }
}

impl<'a, 'b> Add<MoneyDynamic<'b>> for MoneyDynamic<'a> {
    type Output = Self;

    fn add(mut self, other: MoneyDynamic<'b>) -> Self::Output {
        assert_eq!(self.currency_code, other.currency_code);
        assert_eq!(self.currency_units, other.currency_units);

        self.amount += other.amount;
        self
    }
}

impl<'a, 'b> Sub<MoneyDynamic<'b>> for MoneyDynamic<'a> {
    type Output = Self;

    fn sub(mut self, other: MoneyDynamic<'b>) -> Self::Output {
        assert_eq!(self.currency_code, other.currency_code);
        assert_eq!(self.currency_units, other.currency_units);

        self.amount -= other.amount;
        self
    }
}

impl<'c, C: Currency> std::iter::Sum for Money<C> {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        let first = iter.next().unwrap();
        iter.fold(first, Add::add)
    }
}

impl<'r, C: Currency + 'r> std::iter::Sum<&'r Self> for Money<C> {
    fn sum<I: Iterator<Item = &'r Self>>(iter: I) -> Self {
        iter.fold(Money::with_amount(0), Add::add)
    }
}

#[cfg(test)]
mod tests {
    use crate::Money;

    mod currency {
        crate::define_currency_array!([("Test currency", "TEST", 1), ("Useless", "USE", 2)]);
    }

    #[test]
    fn sum() {
        let (m1, m2) = (
            Money::<currency::TEST>::with_amount(100),
            Money::<currency::TEST>::with_amount(300),
        );

        assert_eq!(Money::<currency::TEST>::with_amount(400), m1 + m2)
    }

    #[test]
    fn sum_iter() {
        let result = [
            Money::<currency::TEST>::with_amount(100),
            Money::<currency::TEST>::with_amount(300),
            Money::<currency::TEST>::with_amount(500),
        ]
        .into_iter()
        .sum();

        assert_eq!(Money::with_amount(900), result);
    }

    #[test]
    fn sub() {
        let (m1, m2) = (
            Money::<currency::TEST>::with_amount(300),
            Money::<currency::TEST>::with_amount(200),
        );

        assert_eq!(Money::with_amount(100), m1 - m2);
    }

    #[test]
    fn sum_negative() {
        let (m1, m2) = (
            Money::<currency::TEST>::with_amount(300),
            Money::<currency::TEST>::with_amount(-200),
        );

        assert_eq!(Money::with_amount(100), m1 + m2);
    }

    #[test]
    fn sum_negative_iter() {
        let result = [
            Money::<currency::TEST>::with_amount(300),
            Money::<currency::TEST>::with_amount(-200),
            Money::<currency::TEST>::with_amount(100),
            Money::<currency::TEST>::with_amount(-25),
            Money::<currency::TEST>::with_amount(10),
        ]
        .into_iter()
        .sum();

        assert_eq!(Money::with_amount(185), result);
    }
}
