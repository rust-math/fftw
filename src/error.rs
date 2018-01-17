use ndarray::ShapeError;

pub type Result<T> = ::std::result::Result<T, Error>;

use super::fftw::NAEInputMismatchError;
use super::plan::InputMismatchError;

#[derive(Debug, IntoEnum)]
pub enum Error {
    InvalidPlanError(InvalidPlanError),
    ShapeError(ShapeError),
    NAEInputMismatchError(NAEInputMismatchError),
    InputMismatchError(InputMismatchError),
}

#[derive(Debug, new)]
pub struct InvalidPlanError {}
