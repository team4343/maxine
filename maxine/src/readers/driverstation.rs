use async_trait::async_trait;
use flume::{Receiver, Sender};
use maxine_common::{readers::Reader, scheduler::Schedulable};
use miette::Diagnostic;
use thiserror::Error;

/// Reads from the DS and updates the state accordingly. Includes current game
/// state, and possibly other values.
pub struct DriverStationReader<'a> {
    ds_handle: (),
    state_handle: &'a DriverStationState,
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

impl<'a> DriverStationReader<'a> {
    pub fn new(state_handle: &'a DriverStationState) -> Self {
        Self {
            ds_handle: (),
            state_handle,
        }
    }
}

#[async_trait]
impl<'a> Schedulable for DriverStationReader<'a> {
    type E = DriverStationReaderError;

    async fn run(&mut self) -> Result<(), Self::E> {
        loop {
            // (1) read DS state from DS
            // (2) get the current DS state
            // (3) if there's a change, update appropriately

            todo!()
        }
    }
}

#[async_trait]
impl<'a> Reader for DriverStationReader<'a> {
    type State = DriverStationState;
}

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
