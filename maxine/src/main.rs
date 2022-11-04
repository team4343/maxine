use arfur::prelude::*;
use mekena::prelude::*;
use miette::{IntoDiagnostic, Result};

#[main]
async fn main(system: System) -> Result<()> {
    let robot = RobotBuilder::default().initialize().into_diagnostic()?;

    system
        .add_node(subsystems::drivetrain::Drivetrain::new(robot))
        .start()
        .await
        .into_diagnostic()
}

pub mod subsystems {
    pub mod drivetrain {

        use std::f32::consts::FRAC_PI_2;

        use arfur::prelude::*;
        use maxine::transforms;
        use mekena::prelude::*;
        use nalgebra::{Isometry2, Vector2};

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
        }

        #[node]
        impl Node for Drivetrain {
            async fn running(&mut self, _ctx: &Context) {
                use tokio::time::{self, Duration};

                loop {
                    // Get joystick values. These should all be ranges from 0 to 1.
                    let x = 0.;
                    let y = 0.;
                    let θ = 0.;

                    debug_assert!(x <= 1. && x >= -1.);
                    debug_assert!(y <= 1. && y >= -1.);
                    debug_assert!(θ <= 1. && θ >= -1.);

                    // Apply transformations.
                    let x = transforms::deadband(x);
                    let y = transforms::deadband(y);
                    let θ = transforms::deadband(θ);

                    // Calculate the isometry.
                    let isometry = Isometry2::new(Vector2::new(x, y), θ * FRAC_PI_2);

                    // Run the applied transforms on self.motors
                    self.motors.set(isometry);

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

            pub fn set(&mut self, isometry: Isometry2<f32>) {
                // The given isometry is robot-relative. Transform it to
                // module-relative before passing it on to each module.

                // for module in [&self.lf, &self.lr, &self.rf, &self.rr] {}

                // TODO: potentially move the mounting isometry data inside of
                // each Module.
                let lf = isometry * Isometry2::new(Vector2::new(0., 0.), 0. * FRAC_PI_2);
                let lr = isometry * Isometry2::new(Vector2::new(0., 0.), 0. * FRAC_PI_2);
                let rf = isometry * Isometry2::new(Vector2::new(0., 0.), 0. * FRAC_PI_2);
                let rr = isometry * Isometry2::new(Vector2::new(0., 0.), 0. * FRAC_PI_2);

                todo!()
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
