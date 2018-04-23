use std::collections::HashMap;
use cgmath::{Point2, InnerSpace};
use utils::Positioned;
use std::f32;

pub use beetle::{Id, BeetleBuilder, Beetle, Beetles};

pub type FoodSources = HashMap<Id, FoodSource>;
pub type HomeBases = HashMap<Id, HomeBase>;

pub trait Entity : Positioned {
    fn get_id(&self) -> Id;
    fn set_id(&mut self, id: Id);
}

pub trait HasFood {
    fn add_food(&mut self, amount: i32) -> i32;
    fn remove_food(&mut self, amount: i32) -> i32;
}

#[derive(Serialize, Debug, Clone)]
pub struct HomeBase {
    id: Id,
    food_stored_amount: i32,
    position: Point2<f32>,
}

impl HomeBase {
    pub fn new(id: Id) -> HomeBase {
        HomeBase {
            id: id,
            food_stored_amount: 0,
            position: Point2::new(0.0, 0.0),
        }
    }

    pub fn get_food_stored_amount(&self) -> i32 {
        self.food_stored_amount
    }
}

impl Entity for HomeBase {
    fn get_id(&self) -> Id {
        self.id
    }

    fn set_id(&mut self, id: Id) {
        self.id = id;
    }
}

impl HasFood for HomeBase {

    fn add_food(&mut self, amount: i32) -> i32 {
        self.food_stored_amount += amount;
        amount
    }

    fn remove_food(&mut self, amount: i32) -> i32 {
        self.food_stored_amount -= amount;
        amount
    }
}

impl Positioned for HomeBase {
    fn get_position(&self) -> Point2<f32> {
        self.position
    }
    fn set_position(&mut self, position: Point2<f32>) {
        self.position = position;
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct FoodSource {
    id: Id,
    amount: i32,
    position: Point2<f32>,
}

impl FoodSource {
    pub fn new(id: i32) -> FoodSource {
        FoodSource {
            id: id,
            amount: 100,
            position: Point2::new(0.0, 0.0),
        }
    }

    // TODO: impl HasFood instead
    pub fn reduce_food(&mut self, amount: i32) -> i32 {
        if self.amount > amount {
            self.amount -= amount;
            return amount;
        }
        else {
            let remaining = self.amount;
            self.amount = 0;
            return remaining;
        }
    }

    // TODO: impl HasFood instead
    pub fn increase_food(&mut self, amount: i32) -> i32 {
        self.amount += amount;
        self.amount
    }

    pub fn amount(&self) -> i32 {
        self.amount
    }
}

impl Entity for FoodSource {
    fn get_id(&self) -> Id {
        self.id
    }

    fn set_id(&mut self, id: Id) {
        self.id = id;
    }
}

impl Positioned for FoodSource {
    fn get_position(&self) -> Point2<f32> {
        self.position
    }
    fn set_position(&mut self, position: Point2<f32>) {
        self.position = position;
    }
}

pub fn find_closest<'a, T: Entity, U: Entity>(entity: &T, collection: &'a HashMap<Id, U>) -> Option<&'a U> {
    
    let mut closest_distance = f32::MAX;
    let mut closest = None;

    for other in collection.values() {
        let vector = other.get_position() - entity.get_position();
        let distance = vector.magnitude();

        if distance < closest_distance {
            closest_distance = distance;
            closest = Some(other);
        }
    }

    closest
}
