use simulation::Simulate;
use simulation::GeneticAlgorithm;
use simulation::fight_simulation::FightSimulation;
use ui::UI;
use game::{Game};
use beetle_genome::{BeetleGenome};
use std::thread;
use std::time::{Duration};
use utils::{SIMULATION_PERIOD_MS, Color};
use rand::{Rng, thread_rng};
use std::f32;

pub struct BattleGA<'a> {
    ui: &'a UI,
    game: &'a mut Game,
}

impl<'a> BattleGA<'a> {
    pub fn new(game: &'a mut Game, ui: &'a UI) -> BattleGA<'a> {
        BattleGA {
            ui,
            game,
        }
    }
}

impl<'a> GeneticAlgorithm for BattleGA<'a> {

    fn get_game(&self) -> &Game {
        self.game
    }

    fn get_ui(&self) -> &UI {
        self.ui
    }

    fn run_generation(&mut self) -> (Vec<f32>, Vec<BeetleGenome>) {

        let mut genomes = Vec::new();

        let population_size = self.game.field_state.beetles.len();

        {
            let mut sim = FightSimulation::new(&mut self.game);
            // TODO: should be a way to remove this. Currently its only
            // purpose is so the type checker knows what kind of closure to
            // implement above.
            sim.set_tick_callback(|_state| {
            });
            sim.run();
        }

        while self.game.field_state.beetles.len() < population_size as usize {

            let mut offspring;

            // scope to control borrowing
            {
                let rando_id = self.get_random_individual_id();
                let rando = self.game.field_state.beetles.get(&rando_id).unwrap();
                offspring = self.mutate(&rando);
                let mut rng = thread_rng();
                let rand_x: f32 = rng.gen_range(0.0, 500.0);
                let rand_y: f32 = rng.gen_range(0.0, 500.0);
                offspring.position.x = rand_x;
                offspring.position.y = rand_y;
                offspring.team_id = offspring.id;
            }

            self.game.add_beetle(offspring);
        }

        for (_, beetle) in self.game.field_state.beetles.iter_mut() {
            beetle.health = beetle.max_health();
            beetle.color = Color { r: 213, g: 77, b: 77, a: 255 };
            genomes.push(beetle.genome.clone());
        }

        self.ui.update_game_state(self.game.tick());
        thread::sleep(Duration::from_millis(SIMULATION_PERIOD_MS));

        (Vec::new(), genomes)
    }

}
