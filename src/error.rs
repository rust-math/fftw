use ndarray::ShapeError;

pub type Result<T> = ::std::result::Result<T, Error>;

use super::plan::NAEInputMismatchError;

#[derive(Debug, IntoEnum)]
pub enum Error {
    InvalidPlanError(InvalidPlanError),
    ShapeError(ShapeError),
    NAEInputMismatchError(NAEInputMismatchError),
}

#[derive(Debug, new)]
pub struct InvalidPlanError {}
