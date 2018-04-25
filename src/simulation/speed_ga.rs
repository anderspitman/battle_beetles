use simulation::GeneticAlgorithm;
use ui::UI;
use game::{Game, STARTING_ID};
use entities::{Beetle, Beetles};
use cgmath::{Point2};
use rand::{Rng, thread_rng};
use utils::{Color};


pub struct SpeedGA<'a> {
    ui: &'a UI,
    game: &'a mut Game,
}

impl<'a> SpeedGA<'a> {
    pub fn new(game: &'a mut Game, ui: &'a UI) -> SpeedGA<'a> {
        SpeedGA {
            ui,
            game,
        }
    }
}

impl<'a> GeneticAlgorithm for SpeedGA<'a> {

    fn get_game(&self) -> &Game {
        self.game
    }

    fn get_ui(&self) -> &UI {
        self.ui
    }

    fn run_generation(&mut self) {

        let mut new_population = Beetles::new();

        let mut id = STARTING_ID;
        while new_population.len() < self.game.field_state.beetles.len() {

            let parent1_id = self.tournament_select_individual();
            let parent2_id = self.tournament_select_individual();

            let parent1 = self.game.field_state.beetles.get(&parent1_id).unwrap();
            let parent2 = self.game.field_state.beetles.get(&parent2_id).unwrap();

            let mut offspring1;
            let mut offspring2;

            offspring1 = self.mutate(&parent1);
            offspring2 = self.mutate(&parent2);

            let color = Color { r: 144, g: 153, b: 212, a: 255 };
            offspring1.id = id;
            offspring1.team_id = id;
            offspring1.color = color;
            offspring1.position = random_position();
            new_population.insert(id, offspring1);
            id += 1;
            offspring2.id = id;
            offspring2.team_id = id;
            offspring2.color = color;
            offspring2.position = random_position();
            new_population.insert(id, offspring2);
            id += 1;
        }

        self.game.field_state.beetles = new_population;
    }

    fn fitness(&self, beetle: &Beetle) -> f32 {
        beetle.speed()
    }
}


fn random_position() -> Point2<f32> {
    let mut rng = thread_rng();
    let rand_x: f32 = rng.gen_range(0.0, 500.0);
    let rand_y: f32 = rng.gen_range(0.0, 500.0);

    Point2::new(rand_x, rand_y)
}
