use cgmath::{Point2, Vector2, InnerSpace, Rotation, Rotation2, Rad, Basis2};
use simulation::{Command, Action};
use std::collections::HashMap;

pub type Id = i32;
pub type Beetles = HashMap<Id, Beetle>;


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
    attack_power: i32,
    health: i32,
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
            attack_power: 10,
            health: 100, 
        }
    }

    pub fn set_command(&mut self, command: Command) {
        self.current_command = command;
    }

    pub fn tick(&mut self, beetles: &Beetles) -> Action {
        let action = match self.current_command {
            Command::Move{ position } => {
                self.move_toward(&position);
                Action::Move
            },
            Command::Interact { target_id } => {
                if let Some(target) = beetles.get(&target_id) {
                    if self.close_enough_to_interact(target.position) {
                        Action::Attack{
                            target_id: target_id,
                            attack_power: self.attack_power,
                        }
                    }
                    else {
                        self.move_toward(&target.position);
                        Action::Move
                    }
                }
                else {
                    Action::Nothing
                }
            },
            Command::Idle => {
                Action::Nothing
            },
        };

        return action;
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

    //fn find_closest_food<'a>(&self, foods: &'a Vec<Food>) -> (&'a Food, i32) {

    //    let mut closest_index = 0;
    //    let mut closest = &foods[closest_index];
    //    let min_vec = closest.position - self.position;
    //    let mut min_dist = min_vec.magnitude();

    //    for (i, food) in foods.iter().enumerate() {
    //        let vector = food.position - self.position;
    //        let dist = vector.magnitude();

    //        if dist < min_dist {
    //            min_dist = dist;
    //            closest = &foods[i];
    //            closest_index = i;
    //        }
    //    }

    //    return (closest, closest_index as i32);
    //}

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

    fn close_enough_to_interact(&self, target_position: Point2<f32>) -> bool {
        let vector = target_position - self.position;
        let dist = vector.magnitude();

        return dist < 20.0;
    }

    pub fn take_damage(&mut self, damage_amount: i32) -> bool {
        self.health -= damage_amount;
        let mut dead = false;
        if self.health <= 0 {
            dead = true;
        }
        return dead;
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

