use mekena::prelude::*;
use miette::{IntoDiagnostic, Result};

#[main]
async fn main(system: System) -> Result<()> {
    system
        .add_node(subsystems::drivetrain::Drivetrain::new())
        .start()
        .await
        .into_diagnostic()
}

pub mod subsystems {
    pub mod drivetrain {
        use mekena::prelude::*;

        pub struct Drivetrain {
            motors: Motors,
            state: State,
        }

        impl Drivetrain {
            pub fn new() -> Self {
                Self {
                    motors: Motors::new(),
                    state: State::Off,
                }
            }
        }

        #[node]
        impl Node for Drivetrain {
            async fn running(&mut self, _ctx: &Context) {
                use tokio::time::{self, Duration};

                loop {
                    // Get joystick values.
                    // Apply transformations.
                    // Run the applied transforms on self.motors
                    time::sleep(Duration::from_millis(50)).await;
                }
            }
        }

        /// Motor handles to all eight motors. Can possibly be one handle to a
        /// Swerve type in Arfur.
        pub struct Motors {}

        impl Motors {
            pub fn new() -> Self {
                Self {}
            }
        }

        /// Possible states.
        pub enum State {
            /// Completely turned off.
            Off,
            /// Follow a path with a SwerveController.
            Path(Path),
            /// Read values from joysticks and drive accordingly.
            Teleop,
        }

        /// A message demanding a change in state.
        pub struct ChangeState(State);

        /// A message asking to run a state after the current one has completed.
        /// Note that the teleop state never completes.
        pub struct AddState(State);

        /// A set of desired points. Should be built into Arfur.
        pub struct Path {}
    }
}
