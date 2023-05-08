use std::sync::Arc;

use miette::Diagnostic;
use thiserror::Error;

use crate::error::MaxineError;

use self::{driverstation::DriverStationReaderError, hid::HIDReaderError};

pub mod driverstation;
pub mod hid;
pub mod routine;

#[derive(Default, Debug)]
pub struct ReaderState {
    pub hid: Arc<hid::HIDState>,
    pub driverstation: Arc<driverstation::DriverStationState>,
    // TODO: rest of the states.
}

#[derive(Diagnostic, Error, Debug)]
#[error(transparent)]
pub enum ReaderError {
    DriverStationReaderError(#[from] driverstation::DriverStationReaderError),
    HIDReaderError(#[from] hid::HIDReaderError),
}

// TODO: This should be macro-able.
impl From<DriverStationReaderError> for crate::error::MaxineError {
    fn from(e: DriverStationReaderError) -> Self {
        MaxineError::from(ReaderError::from(e))
    }
}

impl From<HIDReaderError> for crate::error::MaxineError {
    fn from(e: HIDReaderError) -> Self {
        MaxineError::from(ReaderError::from(e))
    }
}
