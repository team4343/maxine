use std::sync::Arc;

use async_trait::async_trait;
use flume::{Receiver, Sender};
use maxine_common::{readers::Reader, scheduler::Schedulable};
use miette::Diagnostic;
use thiserror::Error;
use tokio::task::yield_now;
use tracing::{instrument, trace};

/// Reads the HID inputs from the DS and translates it into a shared state.
/// TODO: this should have some handle to the DS.
#[derive(Debug)]
pub struct HIDReader {
    ds_handle: (),
    state_handle: Arc<HIDState>,
}

#[derive(Diagnostic, Error, Debug)]
pub enum HIDReaderError {}

impl HIDReader {
    pub fn new(state_handle: Arc<HIDState>) -> Self {
        Self {
            ds_handle: (),
            state_handle,
        }
    }
}

#[async_trait]
impl Schedulable for HIDReader {
    type E = HIDReaderError;

    #[instrument(skip(self), fields(scheduler = std::any::type_name::<Self>()))]
    async fn run(self) -> Result<Self, Self::E> {
        trace!("Starting to run the HID Reader...");

        let state_handle = self.state_handle.clone();

        let spawn_handle = tokio::spawn(async move {
            trace!("Hello, world! {:?}", state_handle.a.1.recv_async().await);
        });

        self.state_handle.a.0.send_async(true).await.unwrap();

        spawn_handle.await.unwrap();

        let mut x = 0;

        loop {
            // TODO: fill this in.
            // (1) read HID values from DS
            // (2) get the current HID state
            // (3) if there's a change, update appropriately

            yield_now().await;
        }
    }
}

#[async_trait]
impl Reader for HIDReader {
    type State = HIDState;
}

#[derive(Debug)]
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
