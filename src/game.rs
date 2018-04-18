use cgmath::{Rad, Point2};

use beetle::{BeetleBuilder, BeetleGenome, Beetle, Id, Beetles};
use rand::{Rng, thread_rng};

// This needs to start at 1 because protobuf doesn't handle
// 0s well. See https://github.com/google/protobuf/issues/1606
const STARTING_BEETLE_ID: i32 = 1;

#[derive(Serialize, Debug, Clone)]
pub enum Command {
    Move {
        position: Point2<f32>,
    },
    Interact {
        target_id: Id,
    },
    Idle,
}

#[derive(Debug)]
pub enum Action {
    Move,
    Attack {
        target_id: i32,
        attack_power: i32,
    },
    Nothing,
}

#[derive(Serialize, Debug)]
pub struct FieldState {
    //food: Vec<Food>,
    pub beetles: Beetles,
    selected_beetles: Vec<Id>,
    next_beetle_id: i32,
}

pub struct Game {
    pub field_state: FieldState,
}

impl Game {

    pub fn new() -> Game {
        let game = Game {
            field_state: FieldState {
                //food: Vec::new(),
                beetles: Beetles::new(),
                selected_beetles: Vec::new(),
                next_beetle_id: STARTING_BEETLE_ID,
            },
        };

        return game;
    }

    pub fn set_random_population(
            &mut self, population_size: i32, max_speed: f32,
            max_rotation: f32) {

        let mut rng = thread_rng();

        for _ in 0..population_size {

            let mut genome = BeetleGenome::new();
                genome.set_size(rng.gen());
                genome.set_carapace_density(rng.gen());
                genome.set_strength(rng.gen());
                genome.set_quickness(rng.gen());
            let mut beetle = BeetleBuilder::new()
                .max_speed_units_per_tick(max_speed)
                .rotation_radians_per_tick(Rad(max_rotation))
                .x_pos(rng.gen_range(0.0, 500.0))
                .y_pos(rng.gen_range(0.0, 500.0))
                .genome(genome)
                .build();
            self.add_beetle(beetle);
        }

    }

    pub fn select_beetle(&mut self, beetle_id: Id) {

        let mut found = false;
        if let Some(beetle) = self.field_state.beetles.get_mut(&beetle_id) {
            found = true;

            beetle.selected = true;
        }

        if found {
            self.field_state.selected_beetles.push(beetle_id);
        }
    }

    pub fn deselect_all_beetles(&mut self) {

        for (_, beetle) in self.field_state.beetles.iter_mut() {
            beetle.selected = false;
        }

        self.field_state.selected_beetles.clear();
    }

    pub fn selected_move_command(&mut self, x: f32, y: f32) {
        for id in self.field_state.selected_beetles.iter() {
            if let Some(beetle) = self.field_state.beetles.get_mut(id) {
                beetle.set_command(Command::Move{ position: Point2::new(x, y) });
            }
        }
    }

    pub fn selected_interact_command(&mut self, target_id: Id) {
        for id in self.field_state.selected_beetles.iter() {
            if let Some(beetle) = self.field_state.beetles.get_mut(id) {
                beetle.set_command(Command::Interact{ target_id: target_id });
            }
        }
    }

    //pub fn selected_idle_command(&mut self) {
    //    for id in self.field_state.selected_beetles.iter() {
    //        if let Some(beetle) = self.field_state.beetles.get_mut(id) {
    //            beetle.set_command(Command::Idle);
    //        }
    //    }
    //}

    pub fn add_beetle(&mut self, mut beetle: Beetle) -> Id {

        let id = self.field_state.next_beetle_id;
        beetle.id = id;
        self.field_state.beetles.insert(self.field_state.next_beetle_id, beetle);

        self.field_state.next_beetle_id += 1;

        return id;
    }

    pub fn get_random_beetle_id(&self) -> i32 {
        thread_rng().gen_range::<i32>(
            STARTING_BEETLE_ID,
            (self.field_state.next_beetle_id - 1) as i32)
    }

    pub fn tick(&mut self) -> &FieldState {

        // TODO: figure out how to not need to clone here
        let cloned_beetles = self.field_state.beetles.clone();
        // TODO: maybe move this to struct level to avoid re-allocating
        let mut actions: Vec<Action> = Vec::with_capacity(self.field_state.beetles.len());

        for (_, beetle) in self.field_state.beetles.iter_mut() {
            let action = beetle.tick(&cloned_beetles);
            actions.push(action);
        }

        for action in actions {
            match action {
                Action::Attack{target_id, attack_power} => {
                    let mut dead = false;

                    if let Some(target) = self.field_state.beetles.get_mut(&target_id) {
                        dead = target.take_damage(attack_power);
                    }

                    if dead {
                        self.field_state.beetles.remove(&target_id);
                    }
                },
                _ => {
                }
            }
        }
        
        &self.field_state
    }
}
