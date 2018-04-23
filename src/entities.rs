use std::collections::HashMap;
use cgmath::{Point2, InnerSpace};
use utils::Positioned;

pub use beetle::{Id, BeetleBuilder, Beetle, Beetles};

pub type FoodSources = HashMap<Id, FoodSource>;
pub type HomeBases = HashMap<Id, HomeBase>;

pub trait Entity : Positioned {
    fn get_id(&self) -> Id;
    fn set_id(&mut self, id: Id);
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

    pub fn amount(&self) -> i32 {
        self.amount
    }

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
