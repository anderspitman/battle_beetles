use cgmath::{Point2, Vector2, InnerSpace, Rotation, Rotation2, Rad, Basis2};
use rand::Rng;
use rand;
use std::collections::HashMap;

type Id = i32;

#[derive(Serialize, Debug, Clone)]
enum Command {
    Move {
        position: Point2<f32>,
    },
    Attack {
        target_id: Id,
    },
    Idle,
}

pub struct Simulation {
    pub field_state: FieldState,
}

impl Simulation {

    pub fn new() -> Simulation {
        let mut sim = Simulation {
            field_state: FieldState {
                food: Vec::new(),
                beetles: HashMap::new(),
                selected_beetles: Vec::new(),
                beetle_count: 0,
            }
        };

        return sim;
    }

    pub fn select_beetle(&mut self, beetle_id: Id) {
        self.field_state.selected_beetles.push(beetle_id);
    }

    pub fn deselect_all_beetles(&mut self) {
        self.field_state.selected_beetles.clear();
    }

    pub fn selected_move_command(&mut self, x: f32, y: f32) {
        for id in self.field_state.selected_beetles.iter() {
            if let Some(beetle) = self.field_state.beetles.get_mut(id) {
                beetle.set_command(Command::Move{ position: Point2::new(x, y) });
            }
            else {
            }
        }
    }

    pub fn add_beetle(&mut self, mut beetle: Beetle) -> Id {

        let id = self.field_state.beetle_count;
        beetle.id = id;
        self.field_state.beetles.insert(self.field_state.beetle_count, beetle);

        self.field_state.beetle_count += 1;

        return id;
    }

    pub fn add_food(&mut self, x: f32, y: f32) {
        let mut food = Food::new();
        food.position = Point2::new(x, y);
        self.field_state.food.push(food);
    }

    pub fn get_beetle(&self, beetle_id: Id) -> Option<&Beetle> {
        return self.field_state.beetles.get(&beetle_id);
    }

    pub fn tick(&mut self) -> &FieldState {

        for (id, beetle) in self.field_state.beetles.iter_mut() {
            println!("{:?}", beetle);
            beetle.tick();
        }
        
        &self.field_state
    }

    //pub fn tick(&mut self) -> &FieldState {

    //    let mut new_beetles =
    //        Vec::with_capacity(self.field_state.beetles.len());

    //    for beetle in &self.field_state.beetles {
    //        let new_beetle = 
    //            beetle.tick(&self.field_state.beetles,
    //                        &mut self.field_state.food);

    //        new_beetles.push(new_beetle);
    //    }

    //    self.field_state.beetles = new_beetles;

    //    &self.field_state
    //}

    //pub fn done(&self) -> bool {
    //    self.field_state.food.len() == 0
    //}
}

#[derive(Serialize, Debug)]
pub struct FieldState {
    food: Vec<Food>,
    beetles: HashMap<Id, Beetle>,
    selected_beetles: Vec<Id>,
    beetle_count: i32,
}

#[derive(Serialize, Debug, Clone)]
pub struct Beetle {
    pub id: Id,
    position: Point2<f32>,
    direction: Vector2<f32>,
    angle: Rad<f32>,
    smell_range: i32,
    speed: f32,
    rotation_rads_per_second: Rad<f32>,
    num_eaten: i32,
    current_command: Command,
}


impl Beetle {
    pub fn new() -> Beetle {
        Beetle{
            id: 0,
            position: Point2::new(0.0, 0.0),
            direction: Vector2::new(0.0, 1.0),
            angle: Rad(0.0),
            smell_range: 5,
            speed: 0.5,
            rotation_rads_per_second: Rad(0.02),
            num_eaten: 0,
            current_command: Command::Idle,
        }
    }

    pub fn set_command(&mut self, command: Command) {
        self.current_command = command;
    }

    pub fn tick(&mut self) {
        match self.current_command {
            Command::Move{ position } => {
                println!("Move to {}, {}", position.x, position.y);
                self.move_toward(&position);
            },
            Command::Attack{ target_id } => {
                println!("Attack");
            },
            Command::Idle => {
                println!("Stop moving");
            }
        }
    }

    //pub fn tick(
    //        &self, _beetles: &Vec<Beetle>,
    //        food: &mut Vec<Food>) -> Beetle {

    //    let mut new_beetle = self.clone();

    //    let mut food_eat_index = None;

    //    if food.len() > 0 {
    //        let (closest_food, closest_food_index) =
    //            self.find_closest_food(food);

    //        // TODO: there are ways to get around cloning here (as well as
    //        // appending to the new vector in the parent method), but the
    //        // optimizations are pretty hacky so I figure I'll hold off for
    //        // now. See https://stackoverflow.com/q/49143770/943814
    //        new_beetle.move_toward(&closest_food.position);

    //        if new_beetle.close_enough_to_eat(&closest_food) {
    //            food_eat_index = Some(closest_food_index);
    //            // Gain a speed boost for each food eaten
    //            new_beetle.speed += 0.02;
    //            new_beetle.num_eaten += 1;
    //        }
    //    }

    //    // This can't be included in the block above because it needs to
    //    // borrow food mutably, and it's already borrowed immutably in that
    //    // block
    //    match food_eat_index {
    //        Some(n) => {
    //            food.remove(n as usize);
    //        },
    //        None => ()
    //    }

    //    return new_beetle;
    //}

    fn find_closest_food<'a>(&self, foods: &'a Vec<Food>) -> (&'a Food, i32) {

        let mut closest_index = 0;
        let mut closest = &foods[closest_index];
        let min_vec = closest.position - self.position;
        let mut min_dist = min_vec.magnitude();

        for (i, food) in foods.iter().enumerate() {
            let vector = food.position - self.position;
            let dist = vector.magnitude();

            if dist < min_dist {
                min_dist = dist;
                closest = &foods[i];
                closest_index = i;
            }
        }

        return (closest, closest_index as i32);
    }

    fn move_toward(&mut self, a: &Point2<f32>) {

        let rot: Basis2<f32> =
            Rotation2::from_angle(self.rotation_rads_per_second);
        let rot_neg: Basis2<f32> =
            Rotation2::from_angle(-self.rotation_rads_per_second);

        let vector = a - self.position;
        let angle = self.direction.angle(vector);

        let thresh = Rad(0.1);

        if angle < -thresh {
            self.direction = rot_neg.rotate_vector(self.direction);
        }
        else if angle > thresh {
            self.direction = rot.rotate_vector(self.direction);
        }
        else {
            self.position.x += self.direction.x * self.speed;
            self.position.y += self.direction.y * self.speed;
        }

        self.angle = Vector2::new(1.0, 0.0).angle(self.direction);
    }

    fn close_enough_to_eat(&self, food: &Food) -> bool {
        let vector = food.position - self.position;
        let dist = vector.magnitude();

        return dist < 20.0;
    }
}

pub struct BeetleBuilder {
    x: f32,
    y: f32,
}

impl BeetleBuilder {
    
    pub fn new() -> BeetleBuilder {
        BeetleBuilder {
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn x_pos(&mut self, val: f32) -> &mut BeetleBuilder {
        self.x = val;

        return self;
    }

    pub fn y_pos(&mut self, val: f32) -> &mut BeetleBuilder {
        self.y = val;

        return self;
    }

    pub fn build(&self) -> Beetle {
        let mut beetle = Beetle::new();
        beetle.position.x = self.x;
        beetle.position.y = self.y;
        return beetle;
    }
}

#[derive(Serialize, Debug)]
pub struct Food {
    position: Point2<f32>
}

impl Food {
    pub fn new() -> Food {
        Food{
            position: Point2::new(0.0, 0.0)
        }
    }
}
