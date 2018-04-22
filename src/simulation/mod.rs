pub mod speed_ga;
pub mod food_ga;
pub mod battle_ga;
pub mod fight_simulation;

use game::{Game};
use ui::UI;
use beetle::{Beetle, Beetles};
use beetle_genome::{BeetleGenome};
use rand::{Rng, thread_rng};

const NUM_GENERATIONS: i32 = 128;
const MUTATION_RATE: f32 = 0.1;

pub trait Simulate<T> {
    fn run(&mut self);

    fn get_tick_callback(&self) -> Option<&T>;
}

pub trait GeneticAlgorithm {
    fn run(&mut self) {

        println!("Run GA");

        for _ in 0..NUM_GENERATIONS {

            let (_speeds, _genomes) = self.run_generation();

            self.get_ui().update_charts_incremental(
                &self.get_game().field_state.beetles
            );
        }

        self.get_ui().update_game_state(&self.get_game().field_state);
    }

    fn run_generation(&mut self) -> (Vec<f32>, Vec<BeetleGenome>);

    fn get_game(&self) -> &Game;
    fn get_ui(&self) -> &UI;

    fn get_population(&self) -> &Beetles {
        &self.get_game().field_state.beetles
    }

    fn get_random_individual_id(&self) -> i32 {
        self.get_game().get_random_beetle_id()
    }

    fn mutate(&self, parent: &Beetle) -> Beetle {
        let mut offspring = parent.clone();

        let mutate = thread_rng().gen::<f32>() < MUTATION_RATE;

        if mutate {
            let random_val = thread_rng().gen::<f32>();
            let random_gene_index = BeetleGenome::get_random_gene_index();
            offspring.genome.set_gene_value(random_gene_index, random_val);
        }

        offspring
    }
}
