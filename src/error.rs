use ndarray::ShapeError;
pub use ndarray_linalg::error::{MemoryContError, StrideError};

pub type Result<T> = ::std::result::Result<T, Error>;

use super::nae::NAEInputMismatchError;

#[derive(Debug, IntoEnum)]
pub enum Error {
    InvalidPlanError(InvalidPlanError),
    ShapeError(ShapeError),
    StrideError(StrideError),
    MemoryContError(MemoryContError),
    NAEInputMismatchError(NAEInputMismatchError),
}

#[derive(Debug, new)]
pub struct InvalidPlanError {}
