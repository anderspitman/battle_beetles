pub const SIMULATION_PERIOD_MS: u64 = 20;
pub const MAX_SPEED_UNITS_PER_SECOND: f32 = 200.0;
pub const MIN_SPEED_UNITS_PER_SECOND: f32 = 10.0;
pub const ROTATION_RADIANS_PER_SECOND: f32 = 3.14159;
pub const MS_PER_SECOND: f32 = 1000.0;
pub const POPULATION_SIZE: i32 = 128;

pub fn convert_value_for_sim_period(value: f32) -> f32 {
    return value * ((SIMULATION_PERIOD_MS as f32) / MS_PER_SECOND);
}
