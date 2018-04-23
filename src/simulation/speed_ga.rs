use simulation::GeneticAlgorithm;
use ui::UI;
use game::{Game, STARTING_ID};
use beetle::{Beetle, Beetles};
use beetle_genome::{BeetleGenome};
use cgmath::{Point2};
use rand::{Rng, thread_rng};
use utils::{Color};

const SELECTION_BIAS: f32 = 0.8;


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

    fn tournament_select_individual(&self) -> i32 {
        let id1 = self.get_random_individual_id();
        let id2 = self.get_random_individual_id();

        let indy1 = self.get_game().field_state.beetles.get(&id1).unwrap();
        let indy2 = self.get_game().field_state.beetles.get(&id2).unwrap();

        let fit1 = SpeedGA::fitness(&indy1);
        let fit2 = SpeedGA::fitness(&indy2);

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

    fn get_random_individual_id(&self) -> i32 {
        self.get_game().get_random_beetle_id()
    }

    fn fitness(beetle: &Beetle) -> f32 {
        beetle.speed()
    }

}

impl<'a> GeneticAlgorithm for SpeedGA<'a> {

    fn get_game(&self) -> &Game {
        self.game
    }

    fn get_ui(&self) -> &UI {
        self.ui
    }

    fn run_generation(&mut self) -> (Vec<f32>, Vec<BeetleGenome>) {

        let mut fitnesses = Vec::new();
        let mut genomes = Vec::new();

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

            fitnesses.push(SpeedGA::fitness(&offspring1));
            fitnesses.push(SpeedGA::fitness(&offspring2));

            genomes.push(offspring1.genome.clone());
            genomes.push(offspring2.genome.clone());

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

        return (fitnesses, genomes);
    }
}


fn random_position() -> Point2<f32> {
    let mut rng = thread_rng();
    let rand_x: f32 = rng.gen_range(0.0, 500.0);
    let rand_y: f32 = rng.gen_range(0.0, 500.0);

    Point2::new(rand_x, rand_y)
}
