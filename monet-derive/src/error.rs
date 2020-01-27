use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Record at index {0} is missing its name")]
    MissingName(usize),
    #[error("Record at index {0} is missing its units")]
    MissingUnits(usize),
    #[error("Record at index {0} has malformed units: {1}")]
    MalformedUnits(usize, ParseIntError),
    #[error("Record at index {0} is missing its code")]
    MissingCode(usize),
    #[error("An error occured at index {0}: {1}")]
    Other(usize, Box<dyn std::error::Error>),
}
