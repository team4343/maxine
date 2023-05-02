use async_trait::async_trait;
use flume::{Receiver, Sender};
use maxine_common::{readers::Reader, scheduler::Schedulable};
use miette::Diagnostic;
use thiserror::Error;

/// Reads the HID inputs from the DS and translates it into a shared state.
/// TODO: this should have some handle to the DS.
pub struct HIDReader<'a> {
    ds_handle: (),
    state_handle: &'a HIDState,
}

#[derive(Diagnostic, Error, Debug)]
pub enum HIDReaderError {}

impl<'a> HIDReader<'a> {
    pub fn new(state_handle: &'a HIDState) -> Self {
        Self {
            ds_handle: (),
            state_handle,
        }
    }
}

#[async_trait]
impl<'a> Schedulable for HIDReader<'a> {
    type E = HIDReaderError;

    async fn run(&mut self) -> Result<(), Self::E> {
        loop {
            // (1) read HID values from DS
            // (2) get the current HID state
            // (3) if there's a change, update appropriately

            todo!()
        }
    }
}

#[async_trait]
impl<'a> Reader for HIDReader<'a> {
    type State = HIDState;
}

pub struct HIDState {
    a: (Sender<bool>, Receiver<bool>),
}

impl Default for HIDState {
    fn default() -> Self {
        Self {
            // TODO: can this be bounded?
            a: flume::unbounded(),
        }
    }
}
