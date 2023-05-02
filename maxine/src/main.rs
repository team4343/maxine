pub mod error;
pub mod readers;
pub mod robot;
pub mod routines;
pub mod subsystems;

use miette::{IntoDiagnostic, Result};
use tracing_subscriber::{fmt, prelude::*};

fn main() -> Result<()> {
    // TODO: external log collectors.
    let subscriber = tracing_subscriber::Registry::default().with(fmt::layer());
    tracing::subscriber::set_global_default(subscriber).into_diagnostic()?;

    let reader_state = readers::ReaderState::default();
    let hid_reader = readers::hid::HIDReader::new(&reader_state.hid);
    let driverstation_reader =
        readers::driverstation::DriverStationReader::new(&reader_state.driverstation);

    Ok(())
}
