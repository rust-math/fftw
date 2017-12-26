use ndarray::ShapeError;
pub use ndarray_linalg::error::{MemoryContError, StrideError};

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, IntoEnum)]
pub enum Error {
    InvalidPlanError(InvalidPlanError),
    ShapeError(ShapeError),
    StrideError(StrideError),
    MemoryContError(MemoryContError),
    AlignmentMismatchError(AlignmentMismatchError),
    SizeMismatchError(SizeMismatchError),
}

#[derive(Debug, new)]
pub struct InvalidPlanError {}

#[derive(Debug, new)]
pub struct AlignmentMismatchError {}

#[derive(Debug, new)]
pub struct SizeMismatchError {
    n: usize,
    n_in: usize,
    n_out: usize,
}
