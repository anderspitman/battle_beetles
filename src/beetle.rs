use cgmath::{Point2, Vector2, InnerSpace, Rotation, Rotation2, Rad, Basis2};
use game::{Command, Action};
use entities::{FoodSource, FoodSources, Entity};
use std::collections::HashMap;
use beetle_genome::{BeetleGenome};
use beetle_state_machine::{BeetleStateMachine};
use utils::{
    convert_value_for_sim_period, MIN_SPEED_UNITS_PER_SECOND, Color, Positioned
};

//const MAX_QUICKNESS: f32 = 10.0;
//const MAX_STRENGTH: f32 = 10.0;
const MAX_SIZE_UNITS: f32 = 40.0;
const MIN_SIZE_UNITS: f32 = 10.0;
const MAX_HEALTH: f32 = 200.0;
const MIN_HEALTH: f32 = 10.0;
const MAX_ATTACK: f32 = 50.0;
const MIN_ATTACK: f32 = 1.0;
//const MAX_CARAPACE_DENSITY: f32 = 10.0;
//const MAX_MASS: f32 = MAX_SIZE * MAX_CARAPACE_DENSITY;

pub type Id = i32;
pub type Beetles = HashMap<Id, Beetle>;

#[derive(Serialize, Debug, Clone)]
pub struct Beetle {
    pub id: Id,
    pub position: Point2<f32>,
    pub direction: Vector2<f32>,
    pub angle: Rad<f32>,
    smell_range: i32,
    max_speed_units_per_tick: f32,
    rotation_radians_per_tick: Rad<f32>,
    num_eaten: i32,
    pub current_command: Command,
    pub health: i32,
    pub selected: bool,
    pub genome: BeetleGenome,
    pub color: Color,
    pub team_id: Id,
    food_collected: i32,
    state_machine: BeetleStateMachine,
}

impl Beetle {
    pub fn new() -> Beetle {
        
        Beetle {
            id: 0,
            position: Point2::new(0.0, 0.0),
            direction: Vector2::new(0.0, 1.0),
            angle: Rad(0.0),
            smell_range: 5,
            max_speed_units_per_tick: 0.0,
            rotation_radians_per_tick: Rad(0.10),
            num_eaten: 0,
            current_command: Command::Idle,
            health: 100, 
            selected: false,
            genome: BeetleGenome::new(),
            color: Color::new(),
            team_id: 0,
            food_collected: 0,
            state_machine: BeetleStateMachine::new(),
        }
    }

    pub fn speed(&self) -> f32 {
        let speed_ratio =
            self.genome.quickness() * 0.25 +
            self.genome.strength() * 0.25 +
            (1.0 - self.genome.size()) * 0.25 + 
            (1.0 - self.genome.carapace_density()) * 0.25;

        let min_speed = convert_value_for_sim_period(
                MIN_SPEED_UNITS_PER_SECOND);
        let speed = (speed_ratio * (self.max_speed_units_per_tick - min_speed)) + min_speed;
        return speed;
    }

    pub fn size(&self) -> f32 {
        (self.genome.size() * (MAX_SIZE_UNITS - MIN_SIZE_UNITS)) + MIN_SIZE_UNITS
    }

    pub fn max_health(&self) -> i32 {
        let health_ratio = 
            self.genome.carapace_density() * 0.50 +
            self.genome.size() * 0.30 +
            self.genome.strength() * 0.20;
        return ((health_ratio * (MAX_HEALTH - MIN_HEALTH)) + MIN_HEALTH) as i32;
    }

    pub fn attack_power(&self) -> i32 {
        let attack_ratio =
            self.genome.mandible_sharpness() * 0.30 +
            self.genome.venomosity() * 0.30 +
            self.genome.strength() * 0.20 +
            self.genome.size() * 0.10 +
            self.genome.quickness() * 0.10;

        return ((attack_ratio * (MAX_ATTACK - MIN_ATTACK)) + MIN_ATTACK) as i32;
    }

    //pub fn mass(&self) -> f32 {
    //    ((self.genome.size() * MAX_SIZE) *
    //    (self.genome.carapace_density() * MAX_CARAPACE_DENSITY)) /
    //    MAX_MASS
    //}

    pub fn set_command(&mut self, command: Command) {
        self.current_command = command;
    }

    pub fn tick(
            &self, beetles: &Beetles,
            food_sources: &FoodSources) -> Action {

        let action = match self.current_command {
            Command::Move{ position } => {

                if self.basically_here(position) {
                    Action::Nothing
                }
                else {
                    Action::MoveToward {
                        beetle_id: self.id,
                        x: position.x,
                        y: position.y,
                    }
                }
            },
            Command::Interact { target_id } => {
                if let Some(target) = beetles.get(&target_id) {
                    if self.close_enough_to_interact(target.position) {
                        if target.team_id != self.team_id {
                            Action::Attack{
                                target_id: target_id,
                                attack_power: self.attack_power(),
                            }
                        }
                        else {
                            Action::Nothing
                        }
                    }
                    else {
                        Action::MoveToward {
                            beetle_id: self.id,
                            x: target.position.x,
                            y: target.position.y,
                        }
                    }
                }
                else if let Some(food_source) = food_sources.get(&target_id) {
                    if self.close_enough_to_interact(food_source.get_position()) {
                        Action::TakeFood {
                            beetle_id: self.id,
                            food_source_id: target_id,
                            amount: 1,
                        }
                    }
                    else {
                        Action::MoveToward {
                            beetle_id: self.id,
                            x: food_source.get_position().x,
                            y: food_source.get_position().y,
                        }
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

    pub fn move_toward(&mut self, a: &Point2<f32>) {

        let rot: Basis2<f32> =
            Rotation2::from_angle(self.rotation_radians_per_tick);
        let rot_neg: Basis2<f32> =
            Rotation2::from_angle(-self.rotation_radians_per_tick);

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
            self.position.x += self.direction.x * self.speed();
            self.position.y += self.direction.y * self.speed();
        }

        self.angle = Vector2::new(1.0, 0.0).angle(self.direction);
    }

    fn close_enough_to_interact(&self, target_position: Point2<f32>) -> bool {
        let vector = target_position - self.position;
        let dist = vector.magnitude();

        return dist < 5.0;
    }

    pub fn basically_here(&self, position: Point2<f32>) -> bool {
        let vector = position - self.position;
        let dist = vector.magnitude();
        return dist < 5.0;
    }

    pub fn take_damage(&mut self, damage_amount: i32) -> bool {
        self.health -= damage_amount;
        let mut dead = false;
        if self.health <= 0 {
            dead = true;
        }
        return dead;
    }

    pub fn add_food(&mut self, amount: i32) {
        self.food_collected += amount;
    }
}

pub struct BeetleBuilder {
    x: f32,
    y: f32,
    max_speed_units_per_tick: f32,
    rotation_radians_per_tick: Rad<f32>,
    genome: BeetleGenome,
}

impl BeetleBuilder {
    
    pub fn new() -> BeetleBuilder {
        BeetleBuilder {
            x: 0.0,
            y: 0.0,
            max_speed_units_per_tick: 1.0,
            rotation_radians_per_tick: Rad(0.01),
            genome: BeetleGenome::new(),
        }
    }

    pub fn max_speed_units_per_tick(&mut self, val: f32) -> &mut BeetleBuilder {
        self.max_speed_units_per_tick = val;
        return self;
    }

    pub fn rotation_radians_per_tick(
            &mut self, val: Rad<f32>) -> &mut BeetleBuilder {
        self.rotation_radians_per_tick = val;
        return self;
    }

    pub fn x_pos(&mut self, val: f32) -> &mut BeetleBuilder {
        self.x = val;
        return self;
    }

    pub fn y_pos(&mut self, val: f32) -> &mut BeetleBuilder {
        self.y = val;
        return self;
    }

    pub fn genome(&mut self, val: BeetleGenome) -> &mut BeetleBuilder {
        self.genome = val;
        return self;
    }

    pub fn build(&self) -> Beetle {
        let mut beetle = Beetle::new();
        beetle.position.x = self.x;
        beetle.position.y = self.y;
        beetle.max_speed_units_per_tick = self.max_speed_units_per_tick;
        beetle.rotation_radians_per_tick = self.rotation_radians_per_tick;
        // TODO: figure out how to move self.genome rather than cloning it
        beetle.genome = self.genome.clone();
        return beetle;
    }
}

impl Entity for Beetle {
    fn get_id(&self) -> Id {
        self.id
    }

    fn set_id(&mut self, id: Id) {
        self.id = id;
    }
}

impl Positioned for Beetle {
    fn get_position(&self) -> Point2<f32> {
        self.position
    }
    fn set_position(&mut self, position: Point2<f32>) {
        self.position = position;
    }
}
