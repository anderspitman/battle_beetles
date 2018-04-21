pub mod speed_ga;
pub mod battle_ga;
pub mod fight_simulation;

use game::Game;
use ui::UI;
use beetle::{Beetle, Beetles};
use beetle_genome::{BeetleGenome};
use rand::{Rng, thread_rng};

const NUM_GENERATIONS: i32 = 128;
const MUTATION_RATE: f32 = 0.1;

pub trait Simulate {
    fn run(&mut self);
}

pub trait GeneticAlgorithm {
    fn run(&mut self) {

        println!("Run GA");

        let mut average_fitnesses = Vec::new();
        let mut max_fitnesses = Vec::new();
        let mut average_sizes: Vec<f32> = Vec::new();
        let mut average_densities: Vec<f32> = Vec::new();
        let mut average_strengths: Vec<f32> = Vec::new();
        let mut average_quicknesses: Vec<f32> = Vec::new();

        for _ in 0..NUM_GENERATIONS {

            let (fitnesses, genomes) = self.run_generation();

            let average_fitness = mean(&fitnesses);
            average_fitnesses.push(average_fitness);
            let max_fitness = max(&fitnesses);
            max_fitnesses.push(max_fitness);

            let sizes = genomes.iter().map(|x| x.size()).collect();
            let average_size = mean(&sizes);
            average_sizes.push(average_size);

            let densities = genomes.iter().map(|x| x.carapace_density()).collect();
            let average_density = mean(&densities);
            average_densities.push(average_density);

            let strengths = genomes.iter().map(|x| x.strength()).collect();
            let average_strength = mean(&strengths);
            average_strengths.push(average_strength);

            let quicknesses = genomes.iter().map(|x| x.quickness()).collect();
            let average_quickness = mean(&quicknesses);
            average_quicknesses.push(average_quickness);
        }

        self.get_ui().update_game_state(&self.get_game().field_state);
        self.get_ui().update_charts(
            average_fitnesses, max_fitnesses, average_sizes, average_densities,
            average_strengths, average_quicknesses);
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
            let num_genes = offspring.genome.get_num_genes();
            let random_gene_index = thread_rng().gen_range::<i32>(0, num_genes);
            offspring.genome.set_gene_value(random_gene_index, random_val);
        }

        offspring
    }
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
