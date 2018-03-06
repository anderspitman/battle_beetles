pub struct Simulation {
    field_state: FieldState,
}

impl Simulation {

    pub fn new() -> Simulation {
        Simulation {
            field_state: FieldState {
                entities: vec![],
            }
        }
    }

    pub fn tick(&mut self) -> &FieldState {
        self.field_state.entities.push(Beetle::new(16.0, 17.0));
        &self.field_state
    }
}

#[derive(Serialize, Debug)]
pub struct FieldState {
    pub entities: Vec<Beetle>,
}

#[derive(Serialize, Debug)]
pub struct Beetle {
    entity_type: String,
    x: f32,
    y: f32,
}

impl Beetle {
    pub fn new(x: f32, y: f32) -> Beetle {
        Beetle{
            entity_type: "Beetle".to_string(),
            x,
            y
        }
    }
}
