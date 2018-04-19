use cgmath::{InnerSpace};
use simulation::Simulate;
use ui::UI;
use game::{Game, STARTING_BEETLE_ID};
use beetle::{Id, Beetle, BeetleGenome};
use std::thread;
use std::time::{Duration};
use utils::{SIMULATION_PERIOD_MS, POPULATION_SIZE};
use rand::{Rng, thread_rng};

pub struct BattleSimulation<'a> {
    ui: &'a UI,
    game: &'a mut Game,
}

impl<'a> BattleSimulation<'a> {
    pub fn new(game: &'a mut Game, ui: &'a UI) -> BattleSimulation<'a> {
        BattleSimulation {
            ui,
            game,
        }
    }

    fn find_closest_beetle(&self, beetle: &Beetle) -> Id {

        let mut closest_id = STARTING_BEETLE_ID;
        let mut closest_distance = 1000000.0;

        for (_, other_beetle) in &self.game.field_state.beetles {
            if beetle.id == other_beetle.id {
                continue;
            }

            let vector = other_beetle.position - beetle.position;
            let distance = vector.magnitude();

            if distance < closest_distance {
                closest_distance = distance;
                closest_id = other_beetle.id;
            }
        }

        return closest_id;
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

        println!("Run generation");

        let mut genomes = Vec::new();

        // TODO: get rid of clone somehow
        let beetles = self.game.field_state.beetles.clone();

        for (_, beetle) in &beetles {
            let closest_beetle_id = self.find_closest_beetle(&beetle);
            self.game.select_beetle(beetle.id);
            self.game.selected_interact_command(closest_beetle_id);
            self.game.deselect_all_beetles();
        }

        // the +10 is because sometimes they gang up on each other and less than
        // half get killed
        while self.game.field_state.beetles.len() > ((POPULATION_SIZE / 2) + 10) as usize {
            //println!("loop {}", self.game.field_state.beetles.len());
            self.game.tick();
            //self.ui.update_game_state(self.game.tick());
            //thread::sleep(Duration::from_millis(SIMULATION_PERIOD_MS));
        }

        //println!("Beetles left: {}", self.game.field_state.beetles.len());

        while self.game.field_state.beetles.len() < POPULATION_SIZE as usize {
            let mut offspring;

            {
                let rando_id = self.get_random_individual_id();
                let rando = self.game.field_state.beetles.get(&rando_id).unwrap();
                offspring = self.mutate(&rando);
                let mut rng = thread_rng();
                let rand_x: f32 = rng.gen_range(0.0, 500.0);
                let rand_y: f32 = rng.gen_range(0.0, 500.0);
                offspring.position.x = rand_x;
                offspring.position.y = rand_y;
            }

            self.game.add_beetle(offspring);
        }

        for (_, beetle) in self.game.field_state.beetles.iter_mut() {
            beetle.health = beetle.max_health();
            genomes.push(beetle.genome.clone());
        }

        self.ui.update_game_state(self.game.tick());
        thread::sleep(Duration::from_millis(SIMULATION_PERIOD_MS));

        println!("Beetles after breeding: {}", self.game.field_state.beetles.len());

        (Vec::new(), genomes)
    }

}
