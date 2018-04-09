use simulation::Simulate;
use ui::UI;
use game::{Game};
use beetle::{BeetleBuilder, BeetleGenome, Beetle, Beetles,};
use cgmath::{Rad, Point2};
use rand::{Rng, thread_rng};

const POPULATION_SIZE: i32 = 128;
const NUM_GENERATIONS: i32 = 128;
const SELECTION_BIAS: f32 = 0.8;
const MUTATION_RATE: f32 = 0.1;
//const NUM_GENERATIONS: i32 = 3;

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

            let rand_pos = random_position();

            let mut genome = BeetleGenome::new();
                genome.set_size(rng.gen());
                genome.set_carapace_density(rng.gen());
                genome.set_strength(rng.gen());
                genome.set_quickness(rng.gen());
            let mut beetle = BeetleBuilder::new()
                .max_speed_units_per_tick(converted_speed)
                .rotation_radians_per_tick(Rad(converted_rotation))
                .x_pos(rand_pos.x)
                .y_pos(rand_pos.y)
                .genome(genome)
                .build();
            self.game.add_beetle(beetle);
        }
    }

    fn run_generation(&mut self) -> Vec<f32> {

        let mut fitnesses = Vec::new();

        let mut new_population = Beetles::new();

        let mut id = 1;
        while new_population.len() < self.game.field_state.beetles.len() {

            let parent1_id = self.tournament_select_individual();
            let parent2_id = self.tournament_select_individual();

            let parent1 = self.game.field_state.beetles.get(&parent1_id).unwrap();
            let parent2 = self.game.field_state.beetles.get(&parent2_id).unwrap();

            let mut offspring1 = self.mutate(&parent1);
            let mut offspring2 = self.mutate(&parent2);

            fitnesses.push(SpeedSimulation::fitness(&offspring1));
            fitnesses.push(SpeedSimulation::fitness(&offspring2));

            offspring1.id = id;
            offspring1.position = random_position();
            new_population.insert(id, offspring1);
            id += 1;
            offspring2.id = id;
            offspring2.position = random_position();
            new_population.insert(id, offspring2);
            id += 1;
        }

        self.game.field_state.beetles = new_population;

        return fitnesses;
    }

    fn tournament_select_individual(&self) -> i32 {
        let id1 = self.get_random_individual_id();
        let id2 = self.get_random_individual_id();

        let indy1 = self.game.field_state.beetles.get(&id1).unwrap();
        let indy2 = self.game.field_state.beetles.get(&id2).unwrap();

        let fit1 = SpeedSimulation::fitness(&indy1);
        let fit2 = SpeedSimulation::fitness(&indy2);

        let select_more_fit = thread_rng().gen::<f32>() < SELECTION_BIAS;

        let selected;

        if select_more_fit {
            if fit1 > fit2 {
                selected = id1;
            }
            else {
                selected = id2;
            }
        }
        else {
            if fit1 > fit2 {
                selected = id2;
            }
            else {
                selected = id1;
            }
        }

        selected
    }

    fn get_random_individual_id(&self) -> i32 {
        self.game.get_random_beetle_id()
    }

    fn fitness(beetle: &Beetle) -> f32 {
        beetle.speed()
    }

    fn mutate(&self, parent: &Beetle) -> Beetle {
        let mut offspring = parent.clone();

        let mutate = thread_rng().gen::<f32>() < MUTATION_RATE;

        if mutate {
            let random_val = thread_rng().gen::<f32>();
            let num_genes = offspring.genome.get_num_genes();
            let random_gene_index = thread_rng().gen_range::<i32>(0, num_genes);
            offspring.genome.set_gene_value(random_gene_index, random_val);
        }

        offspring
    }

}

impl<'a> Simulate for SpeedSimulation<'a> {
    fn run(&mut self) {
        self.initialize_population();

        let mut average_fitnesses = Vec::new();
        let mut max_fitnesses = Vec::new();

        for i in 0..NUM_GENERATIONS {
            println!("Run generation {}", i);
            let fitnesses = self.run_generation();
            let average_fitness = mean(&fitnesses);
            let max_fitness = max(&fitnesses);
            average_fitnesses.push(average_fitness);
            max_fitnesses.push(max_fitness);
        }

        self.ui.update_game_state(&self.game.field_state);
        self.ui.update_charts(average_fitnesses, max_fitnesses);
    }
}

fn convert_value_for_sim_period(value: f32) -> f32 {
    return value * ((SIMULATION_PERIOD_MS as f32) / MS_PER_SECOND);
}

fn mean(values: &Vec<f32>) -> f32 {
    let mut sum = 0.0;

    for value in values {
        sum += value;
    }

    sum / (values.len() as f32)
}

fn max(values: &Vec<f32>) -> f32 {
    let mut cur_max = 0.0;

    for value in values {
        if *value > cur_max {
            cur_max = *value;
        }
    }

    cur_max
}

fn random_position() -> Point2<f32> {
    let mut rng = thread_rng();
    let rand_x: f32 = rng.gen_range(0.0, 500.0);
    let rand_y: f32 = rng.gen_range(0.0, 500.0);

    Point2::new(rand_x, rand_y)
}
