use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum Error {}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ConvertError<'d> {
    #[error("Cannot convert {0:?} into target with currency {}, since the currencies differ.")]
    DifferentCurrency(crate::MoneyDynamic<'d>, &'static str),
}

pub type Result<T> = std::result::Result<T, Error>;
pub type ConvertResult<'d, T> = std::result::Result<T, ConvertError<'d>>;
