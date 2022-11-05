use arfur::wpilib::ffi::root::{
    HAL_ControlWord, HAL_GetControlWord, HAL_ObserveUserProgramAutonomous,
    HAL_ObserveUserProgramDisabled, HAL_ObserveUserProgramTeleop, HAL_ObserveUserProgramTest,
};
use mekena::prelude::*;
use miette::IntoDiagnostic;

pub struct Observer {
    previous_state: State,
}

impl Observer {
    pub fn new() -> Self {
        Self {
            previous_state: State::Disabled,
        }
    }
}

#[node]
impl Node for Observer {
    async fn running(&mut self, ctx: &Context) {
        use tokio::time::{self, Duration};

        // Get the state from DS
        // TODO: move this into Arfur.
        let state = unsafe {
            // Not sure if this is the proper initialization.
            let control_word = HAL_ControlWord::new_bitfield_1(0, 0, 0, 0, 0, 0, 0);
            let mut control_word = HAL_ControlWord {
                _bitfield_align_1: [],
                _bitfield_1: control_word,
            };

            HAL_GetControlWord(&mut control_word);

            // God, I hate WPILib's internals.
            if control_word.enabled() != 1 {
                State::Disabled
            } else if control_word.autonomous() == 1 {
                State::Autonomous
            } else if control_word.test() == 1 {
                State::Test
            } else {
                State::Teleop
            }
        };

        unsafe {
            // Send the DS an Observe
            match state {
                State::Disabled => HAL_ObserveUserProgramDisabled(),
                State::Autonomous => HAL_ObserveUserProgramAutonomous(),
                State::Test => HAL_ObserveUserProgramTest(),
                State::Teleop => HAL_ObserveUserProgramTeleop(),
            };
        }

        // If we have a new state, broadcast a message to the drivetrain
        // claiming so.
        if self.previous_state != state {
            use crate::subsystems::drivetrain::{Path, State as DrivetrainState};

            let drivetrain_state = match state {
                State::Teleop => DrivetrainState::Teleop,
                State::Autonomous => DrivetrainState::Path(Path {}),
                _ => DrivetrainState::Off,
            };

            ctx.send(drivetrain_state).await.into_diagnostic().unwrap();
        };

        time::sleep(Duration::from_millis(500)).await;
    }
}

#[derive(Eq, PartialEq)]
pub enum State {
    Disabled,
    Autonomous,
    Test,
    Teleop,
}
