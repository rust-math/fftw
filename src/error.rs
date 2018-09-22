use ndarray::ShapeError;

pub type Result<T> = ::std::result::Result<T, Error>;

use super::plan::Alignment;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Invalid Plan")]
    InvalidPlanError {},

    #[fail(display = "Shape mismatch: {:?}", error)]
    ShapeError { error: ShapeError },

    #[fail(
        display = "Alignment mismatch: origin={:?}, arg={:?}",
        origin,
        args
    )]
    InputMismatchError { origin: Alignment, args: Alignment },
}

impl From<ShapeError> for Error {
    fn from(error: ShapeError) -> Self {
        Error::ShapeError { error }
    }
}
