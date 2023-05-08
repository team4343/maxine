pub mod error;
pub mod readers;
pub mod robot;
pub mod routines;
pub mod subsystems;

use std::{error::Error, sync::Arc};

use error::MaxineError;
use futures::{future::try_join_all, join, TryFutureExt};
use maxine_common::scheduler::Schedulable;
use miette::{IntoDiagnostic, Result};
use readers::{driverstation::DriverStationReader, hid::HIDReader};
use robot::RobotController;
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

    let robot_controller = tokio::spawn(robot_controller.start().map_err(MaxineError::from));
    let hid_reader = tokio::spawn(hid_reader.start().map_err(MaxineError::from));
    let driverstation_reader =
        tokio::spawn(driverstation_reader.start().map_err(MaxineError::from));

    let robot_controller = robot_controller.await.unwrap()?;
    let hid_reader = hid_reader.await.unwrap()?;
    let driverstation_reader = driverstation_reader.await.unwrap()?;

    let robot_controller = tokio::spawn(robot_controller.run().map_err(MaxineError::from));
    let hid_reader = tokio::spawn(hid_reader.run().map_err(MaxineError::from));
    let driverstation_reader = tokio::spawn(driverstation_reader.run().map_err(MaxineError::from));

    let robot_controller = robot_controller.await.unwrap()?;
    let hid_reader = hid_reader.await.unwrap()?;
    let driverstation_reader = driverstation_reader.await.unwrap()?;

    let robot_controller = tokio::spawn(robot_controller.end().map_err(MaxineError::from));
    let hid_reader = tokio::spawn(hid_reader.end().map_err(MaxineError::from));
    let driverstation_reader = tokio::spawn(driverstation_reader.end().map_err(MaxineError::from));

    let robot_controller = robot_controller.await.unwrap()?;
    let hid_reader = hid_reader.await.unwrap()?;
    let driverstation_reader = driverstation_reader.await.unwrap()?;

    Ok(())
}
