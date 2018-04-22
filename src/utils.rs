pub const SIMULATION_PERIOD_MS: u64 = 20;
pub const MAX_SPEED_UNITS_PER_SECOND: f32 = 200.0;
pub const MIN_SPEED_UNITS_PER_SECOND: f32 = 10.0;
pub const ROTATION_RADIANS_PER_SECOND: f32 = 3.14159;
pub const MS_PER_SECOND: f32 = 1000.0;
//pub const POPULATION_SIZE: i32 = 20;
pub const POPULATION_SIZE: i32 = 64;

pub fn convert_value_for_sim_period(value: f32) -> f32 {
    return value * ((SIMULATION_PERIOD_MS as f32) / MS_PER_SECOND);
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new() -> Color {
        Color{ r: 101, g: 224, b: 103, a: 255 }
    }
}

//pub fn mean(values: &Vec<f32>) -> f32 {
//    let mut sum = 0.0;
//
//    for value in values {
//        sum += value;
//    }
//
//    sum / (values.len() as f32)
//}
//
//pub fn max(values: &Vec<f32>) -> f32 {
//    let mut cur_max = 0.0;
//
//    for value in values {
//        if *value > cur_max {
//            cur_max = *value;
//        }
//    }
//
//    cur_max
//}
