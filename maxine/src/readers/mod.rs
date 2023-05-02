pub mod driverstation;
pub mod hid;
pub mod routine;

#[derive(Default)]
pub struct ReaderState {
    pub hid: hid::HIDState,
    pub driverstation: driverstation::DriverStationState,
    // TODO: rest of the states.
}
