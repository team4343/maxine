use std::f64::consts::FRAC_PI_2;

use arfur::{prelude::*, wpilib::ffi::root::frc::XboxController};
use maxine_lib::transforms;
use mekena::prelude::*;
use nalgebra::{Isometry2, Vector2};

pub struct Drivetrain {
    motors: Motors,
    state: State,
    controller: XboxController,
}

impl Drivetrain {
    pub fn new(robot: Robot) -> Self {
        Self {
            motors: Motors::new(robot, (10, 11), (12, 13), (14, 15), (16, 17)),
            state: State::Off,
            controller: unsafe { XboxController::new(0) },
        }
    }

    // TODO: find a better name for this method.
    async fn runnable(&mut self) {
        use tokio::time::{self, Duration};

        match self.state {
            State::Off => (),
            State::Teleop => {
                // Get joystick values. These should all be ranges from 0 to 1.
                // TODO: wrap this in a safer type in Arfur.
                let x = unsafe { self.controller.GetLeftX() };
                let y = unsafe { self.controller.GetLeftY() };
                let θ = unsafe { self.controller.GetRightX() };

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
            State::Path(_) => {
                tracing::error!("You set me to run autonomous, but I don't know how yet.")
            }
        };
    }
}

#[node]
impl Node for Drivetrain {
    async fn running(&mut self, ctx: &Context) {
        use tokio::select;

        loop {
            select! {
                Ok(next_state) = ctx.recv::<ChangeState>() => self.state = next_state.0,
                _ = self.runnable() => (),
            };
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

    pub fn set(&mut self, isometry: Isometry2<f64>) {
        // The given isometry is robot-relative. Transform it to
        // module-relative before passing it on to each module.

        // TODO: potentially move the mounting isometry data inside of
        // each Module.
        let lf = isometry * Isometry2::new(Vector2::new(0., 0.), 0. * FRAC_PI_2);
        let lr = isometry * Isometry2::new(Vector2::new(0., 0.), 0. * FRAC_PI_2);
        let rf = isometry * Isometry2::new(Vector2::new(0., 0.), 0. * FRAC_PI_2);
        let rr = isometry * Isometry2::new(Vector2::new(0., 0.), 0. * FRAC_PI_2);

        self.lf.set(lf);
        self.lr.set(lr);
        self.rf.set(rf);
        self.rr.set(rr);
    }
}

pub struct Module {
    drive: SparkMax,
    rotation: SparkMax,
}

impl Module {
    pub fn new(robot: Robot, drive_id: i32, rotation_id: i32) -> Self {
        Self {
            drive: SparkMax::new(robot, drive_id),
            rotation: SparkMax::new(robot, rotation_id),
        }
    }

    /// Set the drivetrain to drive in a module-relative isometry.
    ///
    /// Setting the same value again should result in no change.
    pub fn set(&mut self, isometry: Isometry2<f64>) {
        // Calculate the speed and rotation based on the isometry.
        let (drive, rotation) = transforms::swerve(isometry);

        self.drive.set_percentage(drive);
        self.rotation.set_percentage(rotation);
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

impl ChangeState {
    pub fn new(state: State) -> Self {
        Self(state)
    }
}

/// A message asking to run a state after the current one has completed.
/// Note that the teleop state never completes.
pub struct AddState(State);

/// A set of desired points. Should be built into Arfur.
pub struct Path {}
