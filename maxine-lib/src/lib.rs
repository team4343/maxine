#![allow(mixed_script_confusables)]

pub mod transforms {
    use std::f64::consts::FRAC_1_PI;

    use nalgebra::Isometry2;

    /// TODO: move this to Arfur.
    /// TODO: review speed/benchmark and optimise
    pub fn deadband(x: f64) -> f64 {
        const CUTOFF: f64 = 0.1;
        const WEIGHT: f64 = 0.2;

        fn cubic(x: f64, weight: f64) -> f64 {
            weight * x * x * x + (1. - weight) * x
        }

        if x.abs() < CUTOFF {
            0.
        } else {
            cubic(x, WEIGHT) - (x.abs() / x) * cubic(CUTOFF, WEIGHT) / (1. - cubic(CUTOFF, WEIGHT))
        }
    }

    /// Given an isometry, calculate the rotational and driving values
    /// (between -1 and 1) for a swerve module.
    pub fn swerve(isometry: Isometry2<f64>) -> (f64, f64) {
        // Extract some information from the isometry for ease-of-use.
        let x = isometry.translation.x;
        let y = isometry.translation.y;
        let θ = isometry.rotation.angle();

        // First, let's calculate the rotation.
        //
        // We divide by PI at the end to get a value from -1 to 1.
        let rotation = (θ + (x.atan2(y))) / FRAC_1_PI;

        // Next, let's calculate the drive. For now, let's set this as the
        // sum of the x and y strengths.
        let drive = x + y;

        (drive, rotation)
    }
}
