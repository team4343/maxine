use std::{error::Error, fmt::Debug};

use async_trait::async_trait;
use tracing::{instrument, trace};

/// Start/Run/End.
#[async_trait]
pub trait Schedulable: Debug {
    type E: Error;

    #[instrument(skip(self), fields(scheduler = std::any::type_name::<Self>()))]
    async fn start(&self) -> Result<(), Self::E> {
        trace!("Ran Self::start. Override this method from the Schedulable trait.");
        Ok(())
    }

    #[instrument(skip(self), fields(scheduler = std::any::type_name::<Self>()))]
    async fn run(&self) -> Result<(), Self::E> {
        trace!("Ran Self::run. Override this method from the Schedulable trait.");
        Ok(())
    }

    #[instrument(skip(self), fields(scheduler = std::any::type_name::<Self>()))]
    async fn end(&self) -> Result<(), Self::E> {
        trace!("Ran Self::end. Override this method from the Schedulable trait.");
        Ok(())
    }
}
