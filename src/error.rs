use ndarray::ShapeError;
pub use ndarray_linalg::error::{MemoryContError, StrideError};

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, IntoEnum)]
pub enum Error {
    InvalidPlanError(InvalidPlanError),
    ShapeError(ShapeError),
    StrideError(StrideError),
    MemoryContError(MemoryContError),
}

#[derive(Debug, new)]
pub struct InvalidPlanError {}
