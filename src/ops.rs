use crate::{Exponent, Money, Rates};

/// A generic operation trait
pub trait Operation {
    /// Execute this operation agains some defined rates.
    fn execute(self, rates: &Rates) -> Option<Money>;
}

/// An operation adding two currencies. The output has same currency code as `A`.
pub struct Add<A: Operation, B: Operation>(pub A, pub B);
/// Operation subtracting two currencies. The output has same currency code as `A`.
pub struct Sub<A: Operation, B: Operation>(pub A, pub B);
/// Operation multiplying a money by an amount. The output has same currency code as `A`.
pub struct Mul<A: Operation>(pub A, pub Exponent);
/// Operation dividing a money by an amount. The output has same currency code as `A`.
pub struct Div<A: Operation>(pub A, pub Exponent);

impl<A: Operation, B: Operation> Operation for Add<A, B> {
    fn execute(self, rates: &Rates) -> Option<Money> {
        let money_a = self.0.execute(rates)?;
        let money_b = self.1.execute(rates)?;

        Some(Money::new(
            money_a.amount + money_b.into_code(money_a.currency_code, rates)?.amount,
            money_a.currency_code,
        ))
    }
}

impl<A: Operation, B: Operation> Operation for Sub<A, B> {
    fn execute(self, rates: &Rates) -> Option<Money> {
        let money_a = self.0.execute(rates)?;
        let money_b = self.1.execute(rates)?;

        Some(Money::new(
            money_a.amount - money_b.into_code(money_a.currency_code, rates)?.amount,
            money_a.currency_code,
        ))
    }
}

impl<A: Operation> Operation for Mul<A> {
    fn execute(self, rates: &Rates) -> Option<Money> {
        let exponent = &self.1;
        let money_a = self.0.execute(rates)?;

        Some(Money::new(
            money_a.amount * exponent.amount / 10i128.pow(u32::from(exponent.exponent)).into(),
            money_a.currency_code,
        ))
    }
}

impl<A: Operation> Operation for Div<A> {
    fn execute(self, rates: &Rates) -> Option<Money> {
        let exponent = &self.1;
        let money_a = self.0.execute(rates)?;

        Some(Money::new(
            money_a.amount * 10i128.pow(u32::from(exponent.exponent)).into() / exponent.amount,
            money_a.currency_code,
        ))
    }
}

// Impl chaining for Add
impl<O: Operation, _A: Operation, _B: Operation> std::ops::Add<O> for Add<_A, _B> {
    type Output = crate::ops::Add<Self, O>;
    fn add(self, other: O) -> Self::Output {
        crate::ops::Add(self, other)
    }
}

impl<O: Operation, _A: Operation, _B: Operation> std::ops::Sub<O> for Add<_A, _B> {
    type Output = crate::ops::Sub<Self, O>;
    fn sub(self, other: O) -> Self::Output {
        crate::ops::Sub(self, other)
    }
}

impl<_A: Operation, _B: Operation> std::ops::Mul<Exponent> for Add<_A, _B> {
    type Output = crate::ops::Mul<Self>;
    fn mul(self, exp: Exponent) -> Self::Output {
        crate::ops::Mul(self, exp)
    }
}

impl<_A: Operation, _B: Operation> std::ops::Div<Exponent> for Add<_A, _B> {
    type Output = crate::ops::Div<Self>;
    fn div(self, exp: Exponent) -> Self::Output {
        crate::ops::Div(self, exp)
    }
}

// Impl chaining for Sub
impl<O: Operation, _A: Operation, _B: Operation> std::ops::Add<O> for Sub<_A, _B> {
    type Output = crate::ops::Add<Self, O>;
    fn add(self, other: O) -> Self::Output {
        crate::ops::Add(self, other)
    }
}

impl<O: Operation, _A: Operation, _B: Operation> std::ops::Sub<O> for Sub<_A, _B> {
    type Output = crate::ops::Sub<Self, O>;
    fn sub(self, other: O) -> Self::Output {
        crate::ops::Sub(self, other)
    }
}

impl<_A: Operation, _B: Operation> std::ops::Mul<Exponent> for Sub<_A, _B> {
    type Output = crate::ops::Mul<Self>;
    fn mul(self, exp: Exponent) -> Self::Output {
        crate::ops::Mul(self, exp)
    }
}

impl<_A: Operation, _B: Operation> std::ops::Div<Exponent> for Sub<_A, _B> {
    type Output = crate::ops::Div<Self>;
    fn div(self, exp: Exponent) -> Self::Output {
        crate::ops::Div(self, exp)
    }
}

// Impl chaining for Mul
impl<O: Operation, _A: Operation> std::ops::Add<O> for Mul<_A> {
    type Output = crate::ops::Add<Self, O>;
    fn add(self, other: O) -> Self::Output {
        crate::ops::Add(self, other)
    }
}

impl<O: Operation, _A: Operation> std::ops::Sub<O> for Mul<_A> {
    type Output = crate::ops::Sub<Self, O>;
    fn sub(self, other: O) -> Self::Output {
        crate::ops::Sub(self, other)
    }
}

impl<_A: Operation> std::ops::Mul<Exponent> for Mul<_A> {
    type Output = crate::ops::Mul<Self>;
    fn mul(self, exp: Exponent) -> Self::Output {
        crate::ops::Mul(self, exp)
    }
}

impl<_A: Operation> std::ops::Div<Exponent> for Mul<_A> {
    type Output = crate::ops::Div<Self>;
    fn div(self, exp: Exponent) -> Self::Output {
        crate::ops::Div(self, exp)
    }
}

// Impl chaining for Div
impl<O: Operation, _A: Operation> std::ops::Add<O> for Div<_A> {
    type Output = crate::ops::Add<Self, O>;
    fn add(self, other: O) -> Self::Output {
        crate::ops::Add(self, other)
    }
}

impl<O: Operation, _A: Operation> std::ops::Sub<O> for Div<_A> {
    type Output = crate::ops::Sub<Self, O>;
    fn sub(self, other: O) -> Self::Output {
        crate::ops::Sub(self, other)
    }
}

impl<_A: Operation> std::ops::Mul<Exponent> for Div<_A> {
    type Output = crate::ops::Mul<Self>;
    fn mul(self, exp: Exponent) -> Self::Output {
        crate::ops::Mul(self, exp)
    }
}

impl<_A: Operation> std::ops::Div<Exponent> for Div<_A> {
    type Output = crate::ops::Div<Self>;
    fn div(self, exp: Exponent) -> Self::Output {
        crate::ops::Div(self, exp)
    }
}

// Impl Operation for money, to allow easier chaining

impl Operation for Money {
    fn execute(self, _rates: &Rates) -> Option<Money> {
        Some(self)
    }
}

// Impl chaining for Money
impl<O: Operation> std::ops::Add<O> for Money {
    type Output = crate::ops::Add<Self, O>;
    fn add(self, other: O) -> Self::Output {
        crate::ops::Add(self, other)
    }
}

impl<O: Operation> std::ops::Sub<O> for Money {
    type Output = crate::ops::Sub<Self, O>;
    fn sub(self, other: O) -> Self::Output {
        crate::ops::Sub(self, other)
    }
}

impl std::ops::Mul<Exponent> for Money {
    type Output = crate::ops::Mul<Self>;
    fn mul(self, exp: Exponent) -> Self::Output {
        crate::ops::Mul(self, exp)
    }
}

impl std::ops::Div<Exponent> for Money {
    type Output = crate::ops::Div<Self>;
    fn div(self, exp: Exponent) -> Self::Output {
        crate::ops::Div(self, exp)
    }
}

#[cfg(test)]
mod tests {
    use crate::rates;
    use crate::{Exponent, Money, Operation};
    use std::convert::TryInto;

    #[test]
    fn test_add_same_code_operation() {
        let money1 = Money::new(1_000_000.into(), "USD".try_into().unwrap());
        let money2 = Money::new(2_000_001.into(), "USD".try_into().unwrap());
        let rates = rates();

        assert_eq!(
            Some(Money::new(3_000_001.into(), "USD".try_into().unwrap())),
            (money1 + money2).execute(&rates)
        );
    }

    #[test]
    fn test_add_operation() {
        // Two equal amounts of money
        let money1 = Money::with_str_code(1_000_010.into(), "GBP").unwrap();
        let money2 = Money::with_str_code(1_500_015.into(), "USD").unwrap();
        let rates = rates();

        assert_eq!(
            Money::with_str_code(2_000_020.into(), "GBP"),
            (money1 + money2).execute(&rates)
        );
    }

    #[test]
    fn test_add_negative_operation() {
        // Two equal amounts of money
        let money1 = Money::with_str_code(1_000_010.into(), "GBP").unwrap();
        let money2 = Money::with_str_code((-1_500_015).into(), "USD").unwrap();
        let rates = rates();

        assert_eq!(
            Money::with_str_code(0.into(), "GBP"),
            (money1 + money2).execute(&rates)
        );
    }

    #[test]
    fn test_sub_operation() {
        // Two equal amounts of money
        let money1 = Money::with_str_code(1_000_010.into(), "GBP").unwrap();
        let money2 = Money::with_str_code(1_500_015.into(), "USD").unwrap();
        let rates = rates();

        assert_eq!(
            Money::with_str_code(0.into(), "GBP"),
            (money1 - money2).execute(&rates)
        );
    }

    #[test]
    fn test_sub_negative_operation() {
        // Two equal amounts of money
        let money1 = Money::with_str_code(1_000_010.into(), "GBP").unwrap();
        let money2 = Money::with_str_code((-1_500_015).into(), "USD").unwrap();
        let rates = rates();

        assert_eq!(
            Money::with_str_code(2_000_020.into(), "GBP"),
            (money1 - money2).execute(&rates)
        );
    }

    #[test]
    fn test_mul_operation() {
        let money = Money::with_str_code(1_000_001.into(), "USD").unwrap();

        assert_eq!(
            (money * Exponent::new(1000.into(), 2)).execute(&rates()),
            Money::with_str_code(10_000_010.into(), "USD")
        );

        assert_eq!(
            (money * Exponent::new(1000.into(), 4)).execute(&rates()),
            Money::with_str_code(100_000.into(), "USD")
        );
    }

    #[test]
    fn test_mul_negative_operation() {
        let money = Money::with_str_code((-1_000_001).into(), "USD").unwrap();

        assert_eq!(
            (money * Exponent::new(1000.into(), 2)).execute(&rates()),
            Money::with_str_code((-10_000_010).into(), "USD")
        );

        assert_eq!(
            (money * Exponent::new(1000.into(), 4)).execute(&rates()),
            Money::with_str_code((-100_000).into(), "USD")
        );
    }

    #[test]
    fn test_div_operation() {
        let money = Money::with_str_code(1_000_001.into(), "USD").unwrap();

        assert_eq!(
            (money / Exponent::new(1000.into(), 2)).execute(&rates()),
            Money::with_str_code(100_000.into(), "USD")
        );

        assert_eq!(
            (money / Exponent::new(1000.into(), 4)).execute(&rates()),
            Money::with_str_code(10_000_010.into(), "USD")
        );
    }

    #[test]
    fn test_div_negative_operation() {
        let money = Money::with_str_code((-1_000_001).into(), "USD").unwrap();

        assert_eq!(
            (money / Exponent::new(1000.into(), 2)).execute(&rates()),
            Money::with_str_code((-100_000).into(), "USD")
        );

        assert_eq!(
            (money / Exponent::new(1000.into(), 4)).execute(&rates()),
            Money::with_str_code((-10_000_010).into(), "USD")
        );
    }

    #[test]
    fn test_money_operation() {
        let money = Money::with_str_code(1_000_000.into(), "USD").unwrap();
        let rates = rates();

        assert_eq!(money.execute(&rates), Some(money));
    }

    #[test]
    fn test_long_add_and_sub_chain() {
        let money1 = Money::with_str_code(1_000_000.into(), "USD").unwrap();
        let money2 = Money::with_str_code(1_000_000.into(), "USD").unwrap();
        let money3 = Money::with_str_code(2_000_000.into(), "USD").unwrap();
        let money4 = Money::with_str_code(2_000_000.into(), "USD").unwrap();
        let money5 = Money::with_str_code(1_000_000.into(), "USD").unwrap();
        let money6 = Money::with_str_code(1_000_000.into(), "USD").unwrap();
        let money7 = Money::with_str_code(2_000_000.into(), "USD").unwrap();

        let result =
            (money1 + money2 - money3 + money4 + money5 - money6 - money7).execute(&rates());

        assert_eq!(result, Money::with_str_code(0.into(), "USD"))
    }

    #[test]
    fn test_long_add_and_sub_chain_with_negative_outcome() {
        let money1 = Money::with_str_code(1_000_000.into(), "USD").unwrap();
        let money2 = Money::with_str_code(1_000_000.into(), "USD").unwrap();
        let money3 = Money::with_str_code(2_000_000.into(), "USD").unwrap();
        let money4 = Money::with_str_code(2_000_000.into(), "USD").unwrap();
        let money5 = Money::with_str_code(1_000_000.into(), "USD").unwrap();
        let money6 = Money::with_str_code(1_000_000.into(), "USD").unwrap();
        let money7 = Money::with_str_code(3_000_000.into(), "USD").unwrap();

        let result =
            (money1 + money2 - money3 + money4 + money5 - money6 - money7).execute(&rates());

        assert_eq!(result, Money::with_str_code((-1_000_000).into(), "USD"))
    }
}
