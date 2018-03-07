pub struct Simulation {
    field_state: FieldState,
}

impl Simulation {

    pub fn new() -> Simulation {
        let mut sim = Simulation {
            field_state: FieldState {
                beetles: vec![],
                food: vec![],
            }
        };

        sim.field_state.beetles.push(Beetle::new());

        let mut food = Food::new();
        food.position.x = 100.0;
        food.position.y = 10.0;
        sim.field_state.food.push(food);

        food = Food::new();
        food.position.x = 200.0;
        food.position.y = 200.0;
        sim.field_state.food.push(food);

        food = Food::new();
        food.position.x = 10.0;
        food.position.y = 275.0;
        sim.field_state.food.push(food);

        return sim;
    }

    pub fn tick(&mut self) -> &FieldState {

        let mut new_beetles =
            Vec::with_capacity(self.field_state.beetles.len());

        for beetle in &self.field_state.beetles {
            let result = 
                beetle.tick(&self.field_state.beetles, &self.field_state.food);

            new_beetles.push(result.new_beetle);

            if result.food_eaten_index >= 0 {

                // TODO: this is expensive
                self.field_state.food.remove(result.food_eaten_index as usize);
            }
        }

        self.field_state.beetles = new_beetles;

        &self.field_state
    }

    pub fn done(&self) -> bool {
        self.field_state.food.len() == 0
    }
}

#[derive(Serialize, Debug)]
pub struct FieldState {
    pub beetles: Vec<Beetle>,
    pub food: Vec<Food>,
}

#[derive(Serialize, Clone, Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new() -> Point {
        Point{x: 0.0, y: 0.0}
    }
}

#[derive(Serialize, Debug)]
struct Vector {
    x: f32,
    y: f32,
}

#[derive(Serialize, Debug, Clone)]
pub struct Beetle {
    position: Point,
    smell_range: i32,
    speed: f32,
}

impl Beetle {
    pub fn new() -> Beetle {
        Beetle{
            smell_range: 5,
            speed: 0.5,
            position: Point::new()
        }
    }

    pub fn tick(
            &self, _beetles: &Vec<Beetle>,
            food: &Vec<Food>) -> BeetleTickResult {

        let closest_food_index = self.find_closest_food(food);
        let closest_food = &food[closest_food_index as usize];

        let mut new_beetle = self.clone();
        new_beetle.move_toward(&closest_food.position);

        let mut food_eaten_index = -1;
        if new_beetle.close_enough_to_eat(&closest_food) {
            food_eaten_index = closest_food_index as i32;
        }

        return BeetleTickResult{
            new_beetle: new_beetle,
            food_eaten_index: food_eaten_index,
        };
    }

    fn find_closest_food(&self, foods: &Vec<Food>) -> i32 {

        let mut closest_index = 0;
        let closest = &foods[closest_index];
        let min_vec = vector_from_to(&self.position, &closest.position);
        let mut min_dist = vector_length(&min_vec);

        for (i, food) in foods.iter().enumerate() {
            let vector = vector_from_to(&self.position, &food.position);
            let dist = vector_length(&vector);

            if dist < min_dist {
                min_dist = dist;
                closest_index = i;
            }
        }

        return closest_index as i32;
    }

    fn move_toward(&mut self, a: &Point) {
        let vector = vector_from_to(&self.position, &a);
        let unit = unit_vector(&vector);

        self.position.x += unit.x * self.speed;
        self.position.y += unit.y * self.speed;
    }

    fn close_enough_to_eat(&self, food: &Food) -> bool {
        let vector = vector_from_to(&self.position, &food.position);
        let dist = vector_length(&vector);

        return dist < 1.0;
    }
}

#[derive(Debug)]
pub struct BeetleTickResult {
    new_beetle: Beetle,
    food_eaten_index: i32,
}

fn vector_from_to(a: &Point, b: &Point) -> Vector {
    Vector{
        x: b.x - a.x,
        y: b.y - a.y,
    }
}

fn vector_length(a: &Vector) -> f32 {
    ((a.x*a.x) + (a.y*a.y)).sqrt()
}

fn unit_vector(a: &Vector) -> Vector {
    let length = vector_length(a);
    Vector{
        x: a.x / length,
        y: a.y / length,
    }
}

#[derive(Serialize, Debug)]
pub struct Food {
    position: Point 
}

impl Food {
    pub fn new() -> Food {
        Food{
            position: Point{ x: 200.0, y: 200.0 }
        }
    }
}
