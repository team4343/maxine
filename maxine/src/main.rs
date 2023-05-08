pub mod error;
pub mod readers;
pub mod robot;
pub mod routines;
pub mod subsystems;

use std::sync::Arc;

use error::MaxineError;
use futures::{join, TryFutureExt};
use maxine_common::scheduler::Schedulable;
use miette::{IntoDiagnostic, Result};
use tracing_subscriber::{fmt, prelude::*};

#[tokio::main]
async fn main() -> Result<()> {
    // TODO: external log collectors.
    let subscriber = tracing_subscriber::Registry::default().with(fmt::layer());
    tracing::subscriber::set_global_default(subscriber).into_diagnostic()?;

    let reader_state = Arc::new(readers::ReaderState::default());

    let mut robot_controller = robot::RobotController::new(reader_state.clone());

    let reader_reader_state = reader_state.clone();
    let mut hid_reader = readers::hid::HIDReader::new(reader_state.hid.clone());
    let mut driverstation_reader =
        readers::driverstation::DriverStationReader::new(reader_state.driverstation.clone());

    // TODO: propogate errors.

    async_scoped::TokioScope::scope_and_block(|s| {
        s.spawn(robot_controller.start().map_err(MaxineError::from));
        s.spawn(hid_reader.start().map_err(MaxineError::from));
        s.spawn(driverstation_reader.start().map_err(MaxineError::from));
    });

    async_scoped::TokioScope::scope_and_block(|s| {
        s.spawn(robot_controller.run().map_err(MaxineError::from));
        s.spawn(hid_reader.run().map_err(MaxineError::from));
        s.spawn(driverstation_reader.run().map_err(MaxineError::from));
    });

    async_scoped::TokioScope::scope_and_block(|s| {
        s.spawn(robot_controller.end().map_err(MaxineError::from));
        s.spawn(hid_reader.end().map_err(MaxineError::from));
        s.spawn(driverstation_reader.end().map_err(MaxineError::from));
    });

    Ok(())
}
