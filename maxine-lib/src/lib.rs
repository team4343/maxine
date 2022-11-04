pub mod transforms {
    /// TODO: move this to Arfur.
    /// TODO: review speed/benchmark and optimise
    pub fn deadband(x: f32) -> f32 {
        const CUTOFF: f32 = 0.1;
        const WEIGHT: f32 = 0.2;

        fn cubic(x: f32, weight: f32) -> f32 {
            weight * x * x * x + (1. - weight) * x
        }

        if x.abs() < CUTOFF {
            0.
        } else {
            cubic(x, WEIGHT) - (x.abs() / x) * cubic(CUTOFF, WEIGHT) / (1. - cubic(CUTOFF, WEIGHT))
        }
    }
}
