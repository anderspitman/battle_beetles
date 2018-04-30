use std::time::{Duration};
use std::thread;

use simulation::GeneticAlgorithm;
use ui::UI;
use game::{Game, Command};
use utils::{SIMULATION_PERIOD_MS, Color};
use entities::{Entity, Beetle, Beetles};

pub struct FoodGA<'a> {
    ui: &'a UI,
    game: Game,
}

impl<'a> FoodGA<'a> {
    pub fn new(population: Beetles, ui: &'a UI) -> FoodGA<'a> {

        let mut game = Game::new();
        game.set_population(population);

        FoodGA {
            ui,
            game,
        }
    }
}

impl<'a> GeneticAlgorithm for FoodGA<'a> {

    fn setup(&mut self) {

        self.game.add_home_base(128.0, 128.0);

        let food_source_id = self.game.add_food_source(256.0, 256.0);

        if let Some(food_source) = self.game.field_state.food_sources.get_mut(&food_source_id) {
            food_source.increase_food(1_000_000);
        }

        for beetle in self.game.field_state.beetles.values_mut() {
            let color = Color { r: 144, g: 153, b: 212, a: 255 };
            beetle.color = color;
            beetle.set_command(Command::HarvestClosestFood);
        }
    }

    fn get_game(&self) -> &Game {
        &self.game
    }

    fn get_ui(&self) -> &UI {
        self.ui
    }

    fn run_generation(&mut self) {

        for beetle in self.game.field_state.beetles.values_mut() {
            beetle.food_collected = 0;
        }

        //while self.game.field_state.get_food_sources().len() > 0 {
        for _ in 0..2000 {
            self.game.tick();
            //self.ui.update_game_state(self.game.tick());
            //thread::sleep(Duration::from_millis(SIMULATION_PERIOD_MS));
        }

        // TODO: change to with_capacity. make sure loop below still works
        let mut new_population = Beetles::new();

        while new_population.len() < self.game.field_state.beetles.len() {

            let parent1_id = self.tournament_select_individual();
            let parent2_id = self.tournament_select_individual();

            let new_id1 = self.game.get_next_id();
            let new_id2 = self.game.get_next_id();

            let parent1 = self.game.field_state.beetles.get(&parent1_id).unwrap();
            let parent2 = self.game.field_state.beetles.get(&parent2_id).unwrap();

            let mut offspring1;
            let mut offspring2;

            offspring1 = self.mutate(&parent1);
            offspring1.set_id(new_id1);
            offspring2 = self.mutate(&parent2);
            offspring2.set_id(new_id2);

            new_population.insert(offspring1.get_id(), offspring1);
            new_population.insert(offspring2.get_id(), offspring2);
        }

        self.game.field_state.beetles = new_population;

        //self.ui.update_game_state(&self.game.field_state);
        //thread::sleep(Duration::from_millis(SIMULATION_PERIOD_MS));
    }

    fn fitness(&self, beetle: &Beetle) -> f32 {
        beetle.food_collected as f32
    }
}
