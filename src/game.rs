use cgmath::{Rad, Point2, InnerSpace};
use entities::{BeetleBuilder, Beetle, Id, Beetles};
use beetle_genome::{BeetleGenome};
use rand::{Rng, thread_rng};
use std::f32;
use entities::{Entity, FoodSource, FoodSources, HomeBase, HomeBases, HasFood};
use utils::Positioned;

// This needs to start at 1 because protobuf doesn't handle
// 0s well. See https://github.com/google/protobuf/issues/1606
pub const STARTING_ID: Id = 1;

#[derive(PartialEq, Serialize, Debug, Clone)]
pub enum State {
    Idle,
    Moving,
    Attacking,
}

#[derive(PartialEq, Serialize, Debug, Clone)]
pub enum Command {
    Move {
        position: Point2<f32>,
    },
    Interact {
        target_id: Id,
    },
    HarvestClosestFood,
    Stop,
}

#[derive(Debug)]
pub enum Action {
    MoveToward {
        beetle_id: Id,
        x: f32,
        y: f32,
    },
    Attack {
        source_id: i32,
        target_id: i32,
        attack_power: i32,
    },
    TakeFood {
        beetle_id: Id,
        food_source_id: Id,
        amount: i32,
    },
    DumpFood {
        beetle_id: Id,
        home_base_id: Id,
        amount: i32,
    },
    Nothing {
        beetle_id: Id,
    },
}

#[derive(Serialize, Debug)]
pub struct FieldState {
    pub food_sources: FoodSources,
    pub beetles: Beetles,
    home_bases: HomeBases,
    selected_beetles: Vec<Id>,
}

impl FieldState {

    pub fn get_food_sources(&self) -> &FoodSources {
        &self.food_sources
    }

    pub fn get_home_bases(&self) -> &HomeBases {
        &self.home_bases
    }
}

pub struct Game {
    pub field_state: FieldState,
    next_id: i32,
}

impl Game {

    pub fn new() -> Game {
        let game = Game {
            field_state: FieldState {
                food_sources: FoodSources::new(),
                beetles: Beetles::new(),
                home_bases: HomeBases::new(),
                selected_beetles: Vec::new(),
            },
            next_id: STARTING_ID,
        };

        return game;
    }

    pub fn get_next_id(&mut self) -> Id {
        let id = self.next_id;
        self.next_id += 1;
        return id;
    }

    pub fn generate_random_population<T: FnMut() -> Id>(
            population_size: i32, max_speed: f32, max_rotation: f32,
            id_generator: &mut T) -> Beetles {

        let mut beetles = Beetles::new();

        let mut rng = thread_rng();

        for _ in 0..population_size {

            let id = id_generator();

            let mut genome = BeetleGenome::new();
                genome.set_random_genome();
            let mut beetle = BeetleBuilder::new()
                .max_speed_units_per_tick(max_speed)
                .rotation_radians_per_tick(Rad(max_rotation))
                .x_pos(rng.gen_range(0.0, 500.0))
                .y_pos(rng.gen_range(0.0, 500.0))
                .genome(genome)
                .build();

            beetle.set_id(id);

            beetles.insert(beetle.get_id(), beetle);
        }

        beetles
    }
            

    pub fn set_population(&mut self, population: Beetles) {
        self.field_state.beetles = population;
    }

    pub fn set_random_population(
            &mut self, population_size: i32, max_speed: f32,
            max_rotation: f32) {

        let beetles;

        {
            let mut id_generator = || {
                self.get_next_id()
            };


            beetles = Game::generate_random_population(
                population_size, max_speed, max_rotation, &mut id_generator); 
        }

        self.field_state.beetles = beetles;
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

    pub fn select_all_in_area(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let x_low = x1.min(x2);
        let x_high = x1.max(x2);
        let y_low = y1.min(y2);
        let y_high = y1.max(y2);

        self.deselect_all_beetles();

        for (_, beetle) in self.field_state.beetles.iter() {
            if beetle.position.x >= x_low && beetle.position.x <= x_high &&
                    beetle.position.y >= y_low && beetle.position.y <= y_high {
                self.field_state.selected_beetles.push(beetle.id);            
            }
        }

        for beetle_id in self.field_state.selected_beetles.iter() {
            if let Some(beetle) = self.field_state.beetles.get_mut(&beetle_id) {
                beetle.selected = true;
            }
        }
    }

    pub fn deselect_all_beetles(&mut self) {

        for (_, beetle) in self.field_state.beetles.iter_mut() {
            beetle.selected = false;
        }

        self.field_state.selected_beetles.clear();
    }

    pub fn selected_move_command(&mut self, x: f32, y: f32) {
        if self.field_state.selected_beetles.len() == 1 {
            for id in self.field_state.selected_beetles.iter() {
                if let Some(beetle) = self.field_state.beetles.get_mut(id) {
                    beetle.set_command(Command::Move{ position: Point2::new(x, y) });
                }
            }
        }
        else {
            self.move_in_formation(x, y);
        }
    }

    pub fn create_formation(&mut self) {

        // create rectangular formation in upper left corner of bounding box
        let (x1, y1, _, _) = self.calculate_selected_bounding_box();

        let spacing = 80.0;
        let column_width = 8;
        let mut row = -1;

        for (i, id) in self.field_state.selected_beetles.iter().enumerate() {
            if let Some(beetle) = self.field_state.beetles.get_mut(id) {

                if i % column_width == 0 {
                    row += 1;
                }

                let x_offset = (i % column_width) as f32;
                let y_offset = row as f32;
                let x = x1 + x_offset * spacing;
                let y = y1 + y_offset * spacing;

                beetle.set_command(Command::Move{ position: Point2::new(x, y) });
            }
        }
    }

    fn move_in_formation(&mut self, x: f32, y: f32) {
        let (x1, y1, x2, y2) = self.calculate_selected_bounding_box();
        let center_x = ((x2 - x1) / 2.0) + x1;
        let center_y = ((y2 - y1) / 2.0) + y1;
        let vector = Point2::new(x, y) - Point2::new(center_x, center_y);

        for id in self.field_state.selected_beetles.iter() {
            if let Some(beetle) = self.field_state.beetles.get_mut(id) {
                let position = beetle.position;
                beetle.set_command(Command::Move{ position: position + vector });
            }
        }
    }

    fn calculate_selected_bounding_box(&self) -> (f32, f32, f32, f32) {

        let mut x_low = f32::MAX;
        let mut y_low = f32::MAX;
        let mut x_high = f32::MIN;
        let mut y_high = f32::MIN;

        for id in self.field_state.selected_beetles.iter() {
            if let Some(beetle) = self.field_state.beetles.get(id) {
                //beetle.set_command(Command::Move{ position: Point2::new(x, y) });
                if beetle.position.x < x_low {
                    x_low = beetle.position.x;
                }
                if beetle.position.x > x_high {
                    x_high = beetle.position.x;
                }
                if beetle.position.y < y_low {
                    y_low = beetle.position.y;
                }
                if beetle.position.y > y_high {
                    y_high = beetle.position.y;
                }
            }
        }

        return (x_low, y_low, x_high, y_high);
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

        let id = self.get_next_id();
        beetle.id = id;
        self.field_state.beetles.insert(id, beetle);
        return id;
    }

    pub fn add_food_source(&mut self, x: f32, y: f32) -> Id {
        let id = self.get_next_id();
        let mut food_source = FoodSource::new(id);
        food_source.set_position(Point2::new(x, y));
        self.field_state.food_sources.insert(id, food_source);

        id
    }

    pub fn add_home_base(&mut self, x: f32, y: f32) {
        let id = self.get_next_id();
        let mut home_base = HomeBase::new(id);
        home_base.set_position(Point2::new(x, y));
        self.field_state.home_bases.insert(id, home_base);
    }

    pub fn get_random_beetle_id(&self) -> i32 {
        let ids: Vec<Id> = self.field_state.beetles.iter().map(|x| x.1.id).collect();

        // TODO: if ids is empty this panics
        let rand_index = thread_rng().gen_range::<i32>(0, (ids.len() - 1) as i32);
        let rand_id = ids[rand_index as usize];
        return rand_id;
    }

    pub fn find_closest_enemy(&self, beetle: &Beetle) -> Option<Id> {


        let enemies = self.field_state.beetles.values().filter(|other| {
            //other.id != beetle.id
            other.team_id != beetle.team_id
        })
        .map(|other| {
            other.id
        })
        .collect();

        self.find_closest(beetle, enemies)
    }

    fn find_closest(&self, beetle: &Beetle, ids: Vec<Id>) -> Option<Id> {

        let mut closest_id = None;
        let mut closest_distance = f32::MAX;

        for id in ids {

            if let Some(other) = self.field_state.beetles.get(&id) {

                let vector = other.position - beetle.position;
                let distance = vector.magnitude();

                if distance < closest_distance {
                    closest_distance = distance;
                    closest_id = Some(other.id);
                }
            }
        }

        closest_id
    }

    pub fn tick(&mut self) -> &FieldState {

        // TODO: maybe move this to struct level to avoid re-allocating
        //let mut actions: Vec<Action> = Vec::with_capacity(self.field_state.beetles.len());

        let actions: Vec<Action>;
        {
            actions = self.field_state.beetles.values().map(|beetle| {
                let action = beetle.tick(
                        &self.field_state.beetles,
                        &self.field_state.food_sources,
                        &self.field_state.home_bases);
                action
            }).collect();
        }

        for action in actions {
            match action {
                Action::MoveToward{beetle_id, x, y} => {
                    if let Some(beetle) = self.field_state.beetles.get_mut(&beetle_id) {

                        let destination = Point2::new(x, y);
                        beetle.move_toward(&destination);
                    }
                },
                Action::Attack{source_id, target_id, attack_power} => {

                    let mut dead = false;
                    let mut went_through = false;

                    if let Some(target) = self.field_state.beetles.get_mut(&target_id) {
                        went_through = true;
                        dead = target.take_damage(attack_power);
                    }

                    if went_through {
                        if let Some(source) = self.field_state.beetles.get_mut(&source_id) {
                            source.damage_inflicted += attack_power;
                        }
                    }

                    if dead {
                        self.field_state.beetles.remove(&target_id);
                    }
                },
                Action::TakeFood{beetle_id, food_source_id, amount} => {

                    let mut empty = false;
                    if let Some(food_source) = self.field_state.food_sources.get_mut(&food_source_id) {

                        let amount_collected = food_source.reduce_food(amount);

                        if amount_collected > 0 {
                            if let Some(beetle) = self.field_state.beetles.get_mut(&beetle_id) {
                                beetle.add_food(amount_collected);
                            }
                        }
                        else {
                            empty = true;
                        }
                    }

                    if empty {
                        self.field_state.food_sources.remove(&food_source_id);
                    }
                },
                Action::DumpFood{beetle_id, home_base_id, amount} => {

                    if let Some(beetle) = self.field_state.beetles.get_mut(&beetle_id) {
                        if let Some(home_base) = self.field_state.home_bases.get_mut(&home_base_id) {
                            beetle.remove_food(amount);
                            home_base.add_food(amount);

                            beetle.food_collected += amount;
                        }
                    }
                },
                Action::Nothing{beetle_id} => {
                    if let Some(beetle) = self.field_state.beetles.get_mut(&beetle_id) {
                        beetle.current_state = State::Idle;
                    }
                }
            }
        }
        
        &self.field_state
    }
}
