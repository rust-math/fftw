use ndarray::ShapeError;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, IntoEnum)]
pub enum Error {
    InvalidPlanError(InvalidPlanError),
    ShapeError(ShapeError),
}

#[derive(Debug)]
pub struct InvalidPlanError {}
