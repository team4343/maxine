use async_trait::async_trait;
use maxine_common::scheduler::Schedulable;
use miette::Diagnostic;
use thiserror::Error;

use crate::readers::ReaderState;

/// Given HID inputs and a desired RoutineDescriptor, decides what the next
/// desired RobotState will be. It may use the DS state to make its decision.
/// For autonomous routines, it will separate each action and run them in order
/// properly. It will use the actual RobotState to update accordingly (e.g. only
/// run the arm when the robot has driven to a point).
///
/// In order to select an autonomous Routine, it uses the RoutineBuilder class
/// and passes it the desired RoutineDescriptor. This will return the proper
/// routine.
/// TODO: store a handle for subsystem states/robot state.
pub struct RobotController<'a> {
    reader_state_handle: &'a ReaderState,
}

#[derive(Diagnostic, Error, Debug)]
pub enum RobotControllerError {}

#[async_trait]
impl<'a> Schedulable for RobotController<'a> {
    type E = RobotControllerError;
}
