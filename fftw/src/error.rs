use crate::array::Alignment;
use thiserror::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid Plan")]
    InvalidPlanError {},

    #[error("Input array mismatch: expect={:?}, actual={:?}", expect, actual)]
    InputArrayMismatch {
        expect: (usize, Alignment),
        actual: (usize, Alignment),
    },

    #[error("Output array mismatch: expect={:?}, actual={:?}", expect, actual)]
    OutputArrayMismatch {
        expect: (usize, Alignment),
        actual: (usize, Alignment),
    },
}
