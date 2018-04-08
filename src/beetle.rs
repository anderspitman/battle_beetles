use cgmath::{Point2, Vector2, InnerSpace, Rotation, Rotation2, Rad, Basis2};
use game::{Command, Action};
use std::collections::HashMap;

//const MAX_QUICKNESS: f32 = 10.0;
//const MAX_STRENGTH: f32 = 10.0;
const MAX_SIZE_UNITS: f32 = 40.0;
const MIN_SIZE_UNITS: f32 = 10.0;
//const MAX_CARAPACE_DENSITY: f32 = 10.0;
//const MAX_MASS: f32 = MAX_SIZE * MAX_CARAPACE_DENSITY;

pub type Id = i32;
pub type Beetles = HashMap<Id, Beetle>;

#[derive(Serialize, Debug, Clone)]
pub struct Beetle {
    pub id: Id,
    pub position: Point2<f32>,
    direction: Vector2<f32>,
    pub angle: Rad<f32>,
    smell_range: i32,
    max_speed_units_per_tick: f32,
    rotation_radians_per_tick: Rad<f32>,
    num_eaten: i32,
    current_command: Command,
    attack_power: i32,
    pub health: i32,
    pub selected: bool,
    pub genome: BeetleGenome,
}

#[derive(Serialize, Debug, Clone)]
pub struct BeetleGenome {
    genome: Vec<BeetleGene>,
}

impl BeetleGenome {
    pub fn new() -> BeetleGenome {
        BeetleGenome {
            genome: vec![
                BeetleGene::Size(0.5),
                BeetleGene::CarapaceDensity(0.5),
                BeetleGene::Strength(0.5),
                BeetleGene::Quickness(0.5),
            ],
        }
    }

    pub fn size(&self) -> f32 {
        match self.genome[0] {
            BeetleGene::Size(value) => value,
            _ => panic!() 
        }
    }
    pub fn set_size(&mut self, value: f32) {
        self.genome[0] = BeetleGene::Size(value);
    }

    pub fn carapace_density(&self) -> f32 {
        match self.genome[1] {
            BeetleGene::CarapaceDensity(value) => value,
            _ => panic!() 
        }
    }
    pub fn set_carapace_density(&mut self, carapace_density: f32) {
        self.genome[1] = BeetleGene::CarapaceDensity(carapace_density);
    }

    pub fn strength(&self) -> f32 {
        match self.genome[2] {
            BeetleGene::Strength(value) => value,
            _ => panic!() 
        }
    }
    pub fn set_strength(&mut self, strength: f32) {
        self.genome[2] = BeetleGene::Strength(strength);
    }
    
    pub fn quickness(&self) -> f32 {
        match self.genome[3] {
            BeetleGene::Quickness(value) => value,
            _ => panic!() 
        }
    }
    pub fn set_quickness(&mut self, quickness: f32) {
        self.genome[3] = BeetleGene::Quickness(quickness);
    }
}

// TODO: properly implement a Ratio type to use with these instead of f32
// should limit the values to 0.0-1.0
#[derive(Serialize, Debug, Clone)]
enum BeetleGene {
    Size(f32),
    CarapaceDensity(f32),
    Strength(f32),
    Quickness(f32),
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
            attack_power: 10,
            health: 100, 
            selected: false,
            genome: BeetleGenome::new(),
        }
    }

    pub fn speed(&self) -> f32 {
        let speed_ratio =
            self.genome.quickness() * 0.25 +
            self.genome.strength() * 0.25 +
            (1.0 - self.genome.size()) * 0.25 + 
            (1.0 - self.genome.carapace_density()) * 0.25;

        let speed = speed_ratio * self.max_speed_units_per_tick;
        return speed;
    }

    pub fn size(&self) -> f32 {
        (self.genome.size() * (MAX_SIZE_UNITS - MIN_SIZE_UNITS)) + MIN_SIZE_UNITS
    }

    //pub fn mass(&self) -> f32 {
    //    ((self.genome.size() * MAX_SIZE) *
    //    (self.genome.carapace_density() * MAX_CARAPACE_DENSITY)) /
    //    MAX_MASS
    //}

    pub fn set_command(&mut self, command: Command) {
        self.current_command = command;
    }

    pub fn tick(&mut self, beetles: &Beetles) -> Action {
        let action = match self.current_command {
            Command::Move{ position } => {
                self.move_toward(&position);

                if self.basically_here(position) {
                    self.current_command = Command::Idle;
                }

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

    fn move_toward(&mut self, a: &Point2<f32>) {

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

        return dist < 20.0;
    }

    fn basically_here(&self, position: Point2<f32>) -> bool {
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

