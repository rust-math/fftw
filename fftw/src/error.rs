use crate::array::Alignment;
use failure::Fail;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Invalid Plan")]
    InvalidPlanError {},

    #[fail(
        display = "Input array mismatch: expect={:?}, actual={:?}",
        expect, actual
    )]
    InputArrayMismatch {
        expect: (usize, Alignment),
        actual: (usize, Alignment),
    },

    #[fail(
        display = "Output array mismatch: expect={:?}, actual={:?}",
        expect, actual
    )]
    OutputArrayMismatch {
        expect: (usize, Alignment),
        actual: (usize, Alignment),
    },
}
