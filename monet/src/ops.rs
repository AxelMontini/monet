use super::Currency;
use super::Money;
use std::ops::{Add, Sub};

impl<'c, C: Currency<'c>> Add for Money<'c, C> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self.amount += other.amount;
        self
    }
}

impl<'c, C: Currency<'c>> Sub for Money<'c, C> {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        self.amount -= other.amount;
        self
    }
}

impl<'r, 'c, C: Currency<'c>> Add<&'r Self> for Money<'c, C> {
    type Output = Self;

    fn add(mut self, other: &'r Self) -> Self::Output {
        self.amount += other.amount;
        self
    }
}

impl<'r, 'c, C: Currency<'c>> Sub<&'r Self> for Money<'c, C> {
    type Output = Self;

    fn sub(mut self, other: &'r Self) -> Self::Output {
        self.amount -= other.amount;
        self
    }
}

impl<'c, C: Currency<'c>> std::iter::Sum for Money<'c, C> {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        let first = iter.next().unwrap();
        iter.fold(first, Add::add)
    }
}

impl<'r, 'c: 'r, C: Currency<'c>> std::iter::Sum<&'r Self> for Money<'c, C> {
    fn sum<I: Iterator<Item = &'r Self>>(iter: I) -> Self {
        iter.fold(Money::with_amount(0), Add::add)
    }
}

#[cfg(test)]
mod tests {
    use crate::Money;

    mod currency {
        crate::define_currency_array!([
            ("Test currency", "TEST", 1)
        ]);
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
        ].into_iter().sum();

        assert_eq!(Money::with_amount(900), result);
    }
}
