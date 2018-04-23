use std::collections::HashMap;
use cgmath::{Point2, InnerSpace};
use utils::Positioned;

pub use beetle::{Id, BeetleBuilder, Beetle, Beetles};

pub type FoodSources = HashMap<Id, FoodSource>;
pub type HomeBases = HashMap<Id, HomeBase>;

#[derive(Serialize, Debug, Clone)]
pub struct HomeBase {
    id: Id,
    amount: i32,
    position: Point2<f32>,
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

    pub fn id(&self) -> Id {
        self.id
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

impl Positioned for FoodSource {
    fn get_position(&self) -> Point2<f32> {
        self.position
    }
    fn set_position(&mut self, position: Point2<f32>) {
        self.position = position;
    }
}
