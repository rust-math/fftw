use ndarray::ShapeError;

pub type Result<T> = ::std::result::Result<T, Error>;

use super::plan::InputMismatchError;

#[derive(Debug, IntoEnum)]
pub enum Error {
    InvalidPlanError(InvalidPlanError),
    ShapeError(ShapeError),
    InputMismatchError(InputMismatchError),
}

#[derive(Debug, new)]
pub struct InvalidPlanError {}
