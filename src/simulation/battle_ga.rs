use std::f32;
use simulation::Simulate;
use simulation::GeneticAlgorithm;
use simulation::fight_simulation::FightSimulation;
use ui::UI;
use game::{Game, FieldState, Command};
use entities::{Beetle, Beetles};
use std::thread;
use std::time::{Duration};
use utils::{SIMULATION_PERIOD_MS, Color};
use rand::{Rng, thread_rng};

pub struct BattleGA<'a> {
    ui: &'a UI,
    game: Game,
}

impl<'a> BattleGA<'a> {
    pub fn new(population: Beetles, ui: &'a UI) -> BattleGA<'a> {

        let mut game = Game::new();
        game.set_population(population);

        BattleGA {
            ui,
            game,
        }
    }
}

impl<'a> GeneticAlgorithm for BattleGA<'a> {

    fn setup(&mut self) {
    }

    fn cleanup(&mut self) {
        for beetle in self.game.field_state.beetles.values_mut() {
            beetle.set_command(Command::Stop);
        }
    }

    fn get_game(&self) -> &Game {
        &self.game
    }

    fn get_ui(&self) -> &UI {
        self.ui
    }

    fn fitness(&self, beetle: &Beetle) -> f32 {
        beetle.damage_inflicted as f32
    }

    fn run_generation(&mut self) {

        let mut rng = thread_rng();

        let population_size = self.game.field_state.beetles.len();

        for beetle in self.game.field_state.beetles.values_mut() {
            beetle.health = beetle.max_health();
            beetle.damage_inflicted = 0;
            beetle.color = Color { r: 213, g: 77, b: 77, a: 255 };
            // put them all on different teams so it's a free for all
            beetle.team_id = beetle.id;

            let rand_x: f32 = rng.gen_range(100.0, 600.0);
            let rand_y: f32 = rng.gen_range(100.0, 600.0);
            beetle.position.x = rand_x;
            beetle.position.y = rand_y;
        }

        {
            let check_done_callback = |state: &FieldState| {
                state.beetles.len() < ((population_size / 2) + 10) as usize
            };

            let mut sim = FightSimulation::new(&mut self.game, check_done_callback);
            // TODO: should be a way to remove this. Currently its only
            // purpose is so the type checker knows what kind of closure to
            // implement above.
            //let ui = self.ui.clone();
            sim.set_tick_callback(move |_state| {
                //ui.update_game_state(&_state);
                //thread::sleep(Duration::from_millis(SIMULATION_PERIOD_MS));
            });
            sim.run();
        }

        while self.game.field_state.beetles.len() < population_size as usize {

            let offspring;

            // scope to satisfy borrow checker 
            {
                let rando_id = self.tournament_select_individual();
                let rando = self.game.field_state.beetles.get(&rando_id).unwrap();
                offspring = self.mutate(&rando);
            }

            self.game.add_beetle(offspring);
        }

        // TODO: I think they're still running and killing a few off after
        // the sim ends
        
        //self.ui.update_game_state(&self.game.field_state);
        //thread::sleep(Duration::from_millis(SIMULATION_PERIOD_MS));
    }

}
