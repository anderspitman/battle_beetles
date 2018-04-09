use simulation::Simulate;
use ui::UI;
use game::{Game};
use beetle::{BeetleBuilder, BeetleGenome, Beetle};
use cgmath::Rad;
use rand::{Rng, thread_rng};

const POPULATION_SIZE: i32 = 128;
//const NUM_GENERATIONS: i32 = 64;
const NUM_GENERATIONS: i32 = 3;

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

    fn initialize_population(&mut self) {
        let mut rng = thread_rng();

        let converted_speed =
            convert_value_for_sim_period(MAX_SPEED_UNITS_PER_SECOND);

        let converted_rotation =
            convert_value_for_sim_period(ROTATION_RADIANS_PER_SECOND);


        for _ in 0..POPULATION_SIZE {

            let rand_x: f32 = rng.gen_range(0.0, 500.0);
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
    }

    fn run_generation(&mut self) -> Vec<f32> {

        let mut fitnesses = Vec::new();

        for (_, beetle) in &self.game.field_state.beetles {
            let fitness = SpeedSimulation::fitness(beetle);
            fitnesses.push(fitness);
            //println!("  fitness: {}", fitness);
        }

        return fitnesses;
    }

    fn fitness(beetle: &Beetle) -> f32 {
        beetle.speed()
    }
}

impl<'a> Simulate for SpeedSimulation<'a> {
    fn run(&mut self) {
        self.initialize_population();

        let mut average_fitnesses = Vec::new();

        for i in 0..NUM_GENERATIONS {
            println!("Run generation {}", i);
            let fitnesses = self.run_generation();
            let average_fitness = mean(fitnesses);
            average_fitnesses.push(average_fitness);
        }

        self.ui.update_game_state(&self.game.field_state);
        self.ui.update_charts(average_fitnesses);
    }
}

fn convert_value_for_sim_period(value: f32) -> f32 {
    return value * ((SIMULATION_PERIOD_MS as f32) / MS_PER_SECOND);
}

fn mean(values: Vec<f32>) -> f32 {
    let mut sum = 0.0;

    for value in &values {
        sum += value;
    }

    sum / (values.len() as f32)
}
