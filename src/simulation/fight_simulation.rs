use simulation::Simulate;
use game::{Game, FieldState};

// Represents a single fight, without generations.
pub struct FightSimulation<'a, T: Fn(&FieldState), U: Fn(&FieldState) -> bool> {
    game: &'a mut Game,
    tick_callback: Option<T>,
    check_done_callback: U,
}

impl<'a, T: Fn(&FieldState), U: Fn(&FieldState) -> bool> FightSimulation<'a, T, U> {
    pub fn new(game: &'a mut Game, check_done_callback: U) -> FightSimulation<'a, T, U> {
        FightSimulation {
            game,
            tick_callback: None,
            check_done_callback: check_done_callback,
        }
    }

    pub fn set_tick_callback(&mut self, tick_callback: T) {
        self.tick_callback = Some(tick_callback);

        if let Some(ref cb) = self.tick_callback {
            cb(&self.game.field_state);
        }
    }
}

impl<'a, T: Fn(&FieldState), U: Fn(&FieldState) -> bool> Simulate<T> for FightSimulation<'a, T, U> {

    fn run(&mut self) {

        //let population_size = self.game.field_state.beetles.len();

        // TODO: get rid of clone somehow
        let beetles = self.game.field_state.beetles.clone();

        // TODO: remove the need for +10
        // the +10 is because sometimes they gang up on each other and less than
        // half get killed, which leads to an infinite loop.
        //while self.game.field_state.beetles.len() > ((population_size / 2) + 5) as usize {
        while !(self.check_done_callback)(&self.game.field_state) {

            for (_, beetle) in beetles.iter() {
                if let Some(closest_beetle_id) = self.game.find_closest_enemy(&beetle) {
                    self.game.select_beetle(beetle.id);
                    self.game.selected_interact_command(closest_beetle_id);
                    self.game.deselect_all_beetles();
                }
                else {
                    println!("no enemies for {}", beetle.id);
                }
            }

            self.game.tick();
            if let Some(tick_callback) = self.get_tick_callback() {
                tick_callback(&self.game.field_state);
            }
        }

    }

    fn get_tick_callback(&self) -> Option<&T> {
        match self.tick_callback {
            Some(ref cb) => Some(&cb),
            None => None
        }
    }
}
