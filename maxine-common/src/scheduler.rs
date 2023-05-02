use std::error::Error;

use async_trait::async_trait;

/// Start/Run/End.
#[async_trait]
pub trait Schedulable {
    type E: Error;

    async fn start(&mut self) -> Result<(), Self::E> {
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Self::E> {
        Ok(())
    }

    async fn end(&mut self) -> Result<(), Self::E> {
        Ok(())
    }
}
