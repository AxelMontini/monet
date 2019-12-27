use crate::Result;
use crate::{Exponent, Money, Rates};

/// A generic operation trait
pub trait Operation {
    /// Execute this operation agains some defined rates.
    fn execute(self, rates: &Rates) -> Result<Money>;
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
    fn execute(self, rates: &Rates) -> Result<Money> {
        let money_a = self.0.execute(rates)?;
        let money_b = self.1.execute(rates)?;

        Ok(Money::new(
            money_a.amount + money_b.into_code(money_a.currency_code, rates)?.amount,
            money_a.currency_code,
        ))
    }
}

impl<A: Operation, B: Operation> Operation for Sub<A, B> {
    fn execute(self, rates: &Rates) -> Result<Money> {
        let money_a = self.0.execute(rates)?;
        let money_b = self.1.execute(rates)?;

        Ok(Money::new(
            money_a.amount - money_b.into_code(money_a.currency_code, rates)?.amount,
            money_a.currency_code,
        ))
    }
}

impl<A: Operation> Operation for Mul<A> {
    fn execute(self, rates: &Rates) -> Result<Money> {
        let exponent = &self.1;
        let money_a = self.0.execute(rates)?;

        Ok(Money::new(
            money_a.amount * exponent.amount / 10i128.pow(u32::from(exponent.exponent)).into(),
            money_a.currency_code,
        ))
    }
}

impl<A: Operation> Operation for Div<A> {
    fn execute(self, rates: &Rates) -> Result<Money> {
        let exponent = &self.1;
        let money_a = self.0.execute(rates)?;

        Ok(Money::new(
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
    fn execute(self, _rates: &Rates) -> Result<Money> {
        Ok(self)
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
    use crate::Result;
    use crate::{Exponent, Money, Operation};

    #[test]
    fn test_add_same_code_operation() -> Result<()> {
        let money1 = Money::new(1_000_000.into(), "USD".parse()?);
        let money2 = Money::new(2_000_001.into(), "USD".parse()?);
        let rates = rates();

        assert_eq!(
            Ok(Money::new(3_000_001.into(), "USD".parse()?)),
            (money1 + money2).execute(&rates)
        );

        Ok(())
    }

    #[test]
    fn test_add_operation() -> Result<()> {
        // Two equal amounts of money
        let money1 = Money::with_str_code(1_000_010.into(), "GBP")?;
        let money2 = Money::with_str_code(1_500_015.into(), "USD")?;
        let rates = rates();

        assert_eq!(
            Money::with_str_code(2_000_020.into(), "GBP"),
            (money1 + money2).execute(&rates)
        );

        Ok(())
    }

    #[test]
    fn test_add_negative_operation() -> Result<()> {
        // Two equal amounts of money
        let money1 = Money::with_str_code(1_000_010.into(), "GBP")?;
        let money2 = Money::with_str_code((-1_500_015).into(), "USD")?;
        let rates = rates();

        assert_eq!(
            Money::with_str_code(0.into(), "GBP"),
            (money1 + money2).execute(&rates)
        );

        Ok(())
    }

    #[test]
    fn test_sub_operation() -> Result<()> {
        // Two equal amounts of money
        let money1 = Money::with_str_code(1_000_010.into(), "GBP")?;
        let money2 = Money::with_str_code(1_500_015.into(), "USD")?;
        let rates = rates();

        assert_eq!(
            Money::with_str_code(0.into(), "GBP"),
            (money1 - money2).execute(&rates)
        );

        Ok(())
    }

    #[test]
    fn test_sub_negative_operation() -> Result<()> {
        // Two equal amounts of money
        let money1 = Money::with_str_code(1_000_010.into(), "GBP")?;
        let money2 = Money::with_str_code((-1_500_015).into(), "USD")?;
        let rates = rates();

        assert_eq!(
            Money::with_str_code(2_000_020.into(), "GBP"),
            (money1 - money2).execute(&rates)
        );

        Ok(())
    }

    #[test]
    fn test_mul_operation() -> Result<()> {
        let money = Money::with_str_code(1_000_001.into(), "USD")?;

        assert_eq!(
            (money * Exponent::new(1000.into(), 2)).execute(&rates()),
            Money::with_str_code(10_000_010.into(), "USD")
        );

        assert_eq!(
            (money * Exponent::new(1000.into(), 4)).execute(&rates()),
            Money::with_str_code(100_000.into(), "USD")
        );

        Ok(())
    }

    #[test]
    fn test_mul_negative_operation() -> Result<()> {
        let money = Money::with_str_code((-1_000_001).into(), "USD")?;

        assert_eq!(
            (money * Exponent::new(1000.into(), 2)).execute(&rates()),
            Money::with_str_code((-10_000_010).into(), "USD")
        );

        assert_eq!(
            (money * Exponent::new(1000.into(), 4)).execute(&rates()),
            Money::with_str_code((-100_000).into(), "USD")
        );

        Ok(())
    }

    #[test]
    fn test_div_operation() -> Result<()> {
        let money = Money::with_str_code(1_000_001.into(), "USD")?;

        assert_eq!(
            (money / Exponent::new(1000.into(), 2)).execute(&rates()),
            Money::with_str_code(100_000.into(), "USD")
        );

        assert_eq!(
            (money / Exponent::new(1000.into(), 4)).execute(&rates()),
            Money::with_str_code(10_000_010.into(), "USD")
        );

        Ok(())
    }

    #[test]
    fn test_div_negative_operation() -> Result<()> {
        let money = Money::with_str_code((-1_000_001).into(), "USD")?;

        assert_eq!(
            (money / Exponent::new(1000.into(), 2)).execute(&rates()),
            Money::with_str_code((-100_000).into(), "USD")
        );

        assert_eq!(
            (money / Exponent::new(1000.into(), 4)).execute(&rates()),
            Money::with_str_code((-10_000_010).into(), "USD")
        );

        Ok(())
    }

    #[test]
    fn test_money_operation() -> Result<()> {
        let money = Money::with_str_code(1_000_000.into(), "USD")?;
        let rates = rates();

        assert_eq!(money.execute(&rates), Ok(money));

        Ok(())
    }

    #[test]
    fn test_long_add_and_sub_chain() -> Result<()> {
        let money1 = Money::with_str_code(1_000_000.into(), "USD")?;
        let money2 = Money::with_str_code(1_000_000.into(), "USD")?;
        let money3 = Money::with_str_code(2_000_000.into(), "USD")?;
        let money4 = Money::with_str_code(2_000_000.into(), "USD")?;
        let money5 = Money::with_str_code(1_000_000.into(), "USD")?;
        let money6 = Money::with_str_code(1_000_000.into(), "USD")?;
        let money7 = Money::with_str_code(2_000_000.into(), "USD")?;

        let result =
            (money1 + money2 - money3 + money4 + money5 - money6 - money7).execute(&rates());

        assert_eq!(result, Money::with_str_code(0.into(), "USD"));

        Ok(())
    }

    #[test]
    fn test_long_add_and_sub_chain_with_negative_outcome() -> Result<()> {
        let money1 = Money::with_str_code(1_000_000.into(), "USD")?;
        let money2 = Money::with_str_code(1_000_000.into(), "USD")?;
        let money3 = Money::with_str_code(2_000_000.into(), "USD")?;
        let money4 = Money::with_str_code(2_000_000.into(), "USD")?;
        let money5 = Money::with_str_code(1_000_000.into(), "USD")?;
        let money6 = Money::with_str_code(1_000_000.into(), "USD")?;
        let money7 = Money::with_str_code(3_000_000.into(), "USD")?;

        let result =
            (money1 + money2 - money3 + money4 + money5 - money6 - money7).execute(&rates());

        assert_eq!(result, Money::with_str_code((-1_000_000).into(), "USD"));

        Ok(())
    }
}
