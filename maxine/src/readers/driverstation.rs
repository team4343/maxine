use std::sync::Arc;

use async_trait::async_trait;
use flume::{Receiver, Sender};
use maxine_common::{readers::Reader, scheduler::Schedulable};
use miette::Diagnostic;
use thiserror::Error;
use tokio::task::yield_now;
use tracing::{instrument, trace};

/// Reads from the DS and updates the state accordingly. Includes current game
/// state, and possibly other values.
#[derive(Debug)]
pub struct DriverStationReader {
    ds_handle: (),
    state_handle: Arc<DriverStationState>,
}

/// TODO: *delete this* and use the one including in the wpilib abstraction.
#[derive(Default)]
pub enum GameState {
    #[default]
    Disabled,
    Teleop,
    Autonomous,
    Test,
    Practice,
}

#[derive(Diagnostic, Error, Debug)]
pub enum DriverStationReaderError {}

impl DriverStationReader {
    pub fn new(state_handle: Arc<DriverStationState>) -> Self {
        Self {
            ds_handle: (),
            state_handle,
        }
    }
}

#[async_trait]
impl Schedulable for DriverStationReader {
    type E = DriverStationReaderError;

    #[instrument(skip(self), fields(scheduler = std::any::type_name::<Self>()))]
    async fn run(self) -> Result<Self, Self::E> {
        trace!("Starting to run the DS Reader...");

        loop {
            // (1) read DS state from DS
            // (2) get the current DS state
            // (3) if there's a change, update appropriately

            yield_now().await;
        }
    }
}

#[async_trait]
impl Reader for DriverStationReader {
    type State = DriverStationState;
}

#[derive(Debug)]
pub struct DriverStationState {
    game_state: (Sender<GameState>, Receiver<GameState>),
}

impl Default for DriverStationState {
    fn default() -> Self {
        Self {
            // TODO: can this be bounded?
            game_state: flume::unbounded(),
        }
    }
}
