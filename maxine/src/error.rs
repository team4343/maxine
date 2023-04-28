use miette::Diagnostic;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, MaxineError>;

#[derive(Diagnostic, Error, Debug)]
pub enum MaxineError {
    #[error("An unknown error occured")]
    #[diagnostic(code(robot::unknown), url(docsrs))]
    Unknown,
}
