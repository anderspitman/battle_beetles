use simulation::Simulate;
use simulation::GeneticAlgorithm;
use simulation::fight_simulation::FightSimulation;
use ui::UI;
use game::{Game, FieldState};
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

    fn setup(&mut self) {
    }

    fn get_game(&self) -> &Game {
        self.game
    }

    fn get_ui(&self) -> &UI {
        self.ui
    }

    fn run_generation(&mut self) {

        let population_size = self.game.field_state.beetles.len();

        for beetle in self.game.field_state.beetles.values_mut() {
            beetle.health = beetle.max_health();
            beetle.color = Color { r: 213, g: 77, b: 77, a: 255 };
        }

        {
            let check_done_callback = |state: &FieldState| {
                state.beetles.len() < ((population_size / 2) + 10) as usize
            };

            let mut sim = FightSimulation::new(&mut self.game, check_done_callback);
            // TODO: should be a way to remove this. Currently its only
            // purpose is so the type checker knows what kind of closure to
            // implement above.
            let ui = self.ui.clone();
            sim.set_tick_callback(move |_state| {
                //ui.update_game_state(&_state);
                //thread::sleep(Duration::from_millis(SIMULATION_PERIOD_MS));
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

        // TODO: I think they're still running and killing a few off after
        // the sim ends
        
        self.ui.update_game_state(&self.game.field_state);
        thread::sleep(Duration::from_millis(SIMULATION_PERIOD_MS));
    }

}
