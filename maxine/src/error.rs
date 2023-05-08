use miette::Diagnostic;
use thiserror::Error;

use crate::{readers, robot};

pub type Result<T> = std::result::Result<T, MaxineError>;

#[derive(Diagnostic, Error, Debug)]
pub enum MaxineError {
    #[error("An error occured in the robot controller.")]
    RobotControllerError(#[from] robot::RobotControllerError),
    #[error("An error occured in a reader.")]
    ReaderError(#[from] readers::ReaderError),
    #[error("An unknown error occured")]
    #[diagnostic(code(robot::unknown), url(docsrs))]
    Unknown,
}
