pub mod speed_ga;
pub mod food_ga;
pub mod battle_ga;
pub mod fight_simulation;

use game::{Game};
use ui::UI;
use entities::{Beetle, Beetles};
use beetle_genome::{BeetleGenome};
use rand::{Rng, thread_rng};

const NUM_GENERATIONS: i32 = 128;
const MUTATION_RATE: f32 = 0.1;
const SELECTION_BIAS: f32 = 0.8;

pub trait Simulate<T> {
    fn run(&mut self);

    fn get_tick_callback(&self) -> Option<&T>;
}

pub trait GeneticAlgorithm {
    fn setup(&mut self) {
        println!("Setup GA");
    }

    fn run(&mut self) {

        println!("Run GA");

        self.setup();

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

    fn fitness(&self, _beetle: &Beetle) -> f32 {
        1.0
    }

    fn tournament_select_individual(&self) -> i32 {
        let id1 = self.get_random_individual_id();
        let id2 = self.get_random_individual_id();

        let indy1 = self.get_game().field_state.beetles.get(&id1).unwrap();
        let indy2 = self.get_game().field_state.beetles.get(&id2).unwrap();

        let fit1 = self.fitness(&indy1);
        let fit2 = self.fitness(&indy2);

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
}
