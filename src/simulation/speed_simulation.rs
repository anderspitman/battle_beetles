use simulation::Simulate;
use ui::UI;
use game::{Game};
use beetle::{BeetleBuilder, BeetleGenome};
use cgmath::Rad;
use rand::{Rng, thread_rng};

const POPULATION_SIZE: i32 = 128;
//const NUM_GENERATIONS: i32 = 64;

const SIMULATION_PERIOD_MS: u64 = 40;
const MS_PER_SECOND: f32 = 1000.0;
const MAX_SPEED_UNITS_PER_SECOND: f32 = 200.0;
const ROTATION_RADIANS_PER_SECOND: f32 = 3.14159;

pub struct SpeedSimulation<'a> {
    ui: &'a UI,
    game: Game,
}

impl<'a> SpeedSimulation<'a> {
    pub fn new(ui: &UI) -> SpeedSimulation {
        SpeedSimulation {
            ui,
            game: Game::new(),
        }
    }

    //pub fn get_field_state(&self) -> &FieldState {
    //    &self.game.field_state
    //}

    pub fn get_game(&mut self) -> &mut Game {
        &mut self.game
    }
}

impl<'a> Simulate for SpeedSimulation<'a> {
    fn run(&mut self) {

        let mut rng = thread_rng();

        let converted_speed =
            convert_value_for_sim_period(MAX_SPEED_UNITS_PER_SECOND);

        let converted_rotation =
            convert_value_for_sim_period(ROTATION_RADIANS_PER_SECOND);


        for _ in 0..POPULATION_SIZE {

            let rand_x: f32 = rng.gen_range(0.0, 1900.0);
            let rand_y: f32 = rng.gen_range(0.0, 500.0);

            let mut genome = BeetleGenome::new();
                genome.set_size(rng.gen());
                genome.set_carapace_density(rng.gen());
                genome.set_strength(rng.gen());
                genome.set_quickness(rng.gen());
            let mut beetle = BeetleBuilder::new()
                .max_speed_units_per_tick(converted_speed)
                .rotation_radians_per_tick(Rad(converted_rotation))
                .x_pos(rand_x)
                .y_pos(rand_y)
                .genome(genome)
                .build();
            self.game.add_beetle(beetle);
        }

        self.ui.update(&self.game.field_state);
    }
}

fn convert_value_for_sim_period(value: f32) -> f32 {
    return value * ((SIMULATION_PERIOD_MS as f32) / MS_PER_SECOND);
}
