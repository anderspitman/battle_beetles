use simulation::Simulate;
use ui::UI;
use game::Game;
use beetle::{BeetleGenome};

pub struct BattleSimulation<'a> {
    ui: &'a UI,
    game: &'a Game,
}

impl<'a> BattleSimulation<'a> {
    pub fn new(game: &'a Game, ui: &'a UI) -> BattleSimulation<'a> {
        BattleSimulation {
            ui,
            game,
        }
    }
}

impl<'a> Simulate for BattleSimulation<'a> {

    fn get_game(&self) -> &Game {
        self.game
    }

    fn get_ui(&self) -> &UI {
        self.ui
    }

    fn run_generation(&mut self) -> (Vec<f32>, Vec<BeetleGenome>) {
        (Vec::new(), Vec::new())
    }
}
