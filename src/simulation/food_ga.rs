use simulation::GeneticAlgorithm;
use ui::UI;
use game::{Game};
use beetle_genome::{BeetleGenome};

pub struct FoodGA<'a> {
    ui: &'a UI,
    game: &'a mut Game,
}

impl<'a> FoodGA<'a> {
    pub fn new(game: &'a mut Game, ui: &'a UI) -> FoodGA<'a> {
        FoodGA {
            ui,
            game,
        }
    }
}

impl<'a> GeneticAlgorithm for FoodGA<'a> {

    fn get_game(&self) -> &Game {
        self.game
    }

    fn get_ui(&self) -> &UI {
        self.ui
    }

    fn run_generation(&mut self) -> (Vec<f32>, Vec<BeetleGenome>) {
        //let population_size = self.game.field_state.beetles.len();
        //

        (Vec::new(), Vec::new())
    }

}
