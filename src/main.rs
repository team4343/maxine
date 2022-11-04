use arfur::prelude::*;
use mekena::prelude::*;
use miette::{IntoDiagnostic, Result};

#[main]
async fn main(system: System) -> Result<()> {
    let robot = RobotBuilder::default().initialize()?;

    system
        .add_node(subsystems::drivetrain::Drivetrain::new(robot))
        .start()
        .await
        .into_diagnostic()
}

pub mod subsystems {
    pub mod drivetrain {

        use arfur::prelude::*;
        use mekena::prelude::*;

        pub struct Drivetrain {
            motors: Motors,
            state: State,
        }

        impl Drivetrain {
            pub fn new(robot: Robot) -> Self {
                Self {
                    motors: Motors::new(robot, (10, 11), (12, 13), (14, 15), (16, 17)),
                    state: State::Off,
                }
            }

            /// TODO: move this to Arfur.
            /// TODO: review speed/benchmark and optimise
            fn deadband(x: f32) -> f32 {
                const CUTOFF: f32 = 0.1;
                const WEIGHT: f32 = 0.2;

                fn cubic(x: f32, weight: f32) -> f32 {
                    weight * x * x * x + (1. - weight) * x
                }

                if x.abs() < CUTOFF {
                    0
                } else {
                    cubic(x, WEIGHT)
                        - (x.abs() / x) * cubic(CUTOFF, WEIGHT) / (1. - cubic(CUTOFF, WEIGHT))
                }
            }
        }

        #[node]
        impl Node for Drivetrain {
            async fn running(&mut self, _ctx: &Context) {
                use tokio::time::{self, Duration};

                loop {
                    // Get joystick values.
                    let x = 0.;
                    let y = 0.;
                    let θ = 0.;

                    // Apply transformations.
                    let x = Self::deadband(x);
                    let y = Self::deadband(y);
                    let θ = Self::deadband(θ);

                    // Run the applied transforms on self.motors

                    // Sleep, and do it all again.
                    time::sleep(Duration::from_millis(50)).await;
                }
            }
        }

        /// Motor handles to all eight motors. Can possibly be one handle to a
        /// Swerve type in Arfur.
        pub struct Motors {
            lf: Module,
            lr: Module,
            rf: Module,
            rr: Module,
        }

        impl Motors {
            pub fn new(
                robot: Robot,
                lf_ids: (i32, i32),
                lr_ids: (i32, i32),
                rf_ids: (i32, i32),
                rr_ids: (i32, i32),
            ) -> Self {
                Self {
                    lf: Module::new(robot, lf_ids.0, lf_ids.1),
                    lr: Module::new(robot, lr_ids.0, lr_ids.1),
                    rf: Module::new(robot, rf_ids.0, rf_ids.1),
                    rr: Module::new(robot, rr_ids.0, rr_ids.1),
                }
            }
        }

        pub struct Module {
            rotation: SparkMax,
            drive: SparkMax,
        }

        impl Module {
            pub fn new(robot: Robot, rotation_id: i32, drive_id: i32) -> Self {
                Self {
                    rotation: SparkMax::new(robot, rotation_id),
                    drive: SparkMax::new(robot, drive_id),
                }
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
