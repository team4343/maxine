#![allow(mixed_script_confusables)]

pub mod subsystems;

use arfur::prelude::*;
use mekena::prelude::*;
use miette::{IntoDiagnostic, Result};

#[main]
async fn main(system: System) -> Result<()> {
    let robot = RobotBuilder::default().initialize().into_diagnostic()?;

    system
        .add_node(subsystems::observer::Observer::new())
        .add_node(subsystems::drivetrain::Drivetrain::new(robot))
        .start()
        .await
        .into_diagnostic()
}
