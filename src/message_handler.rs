use std::thread;
use std::time::{Duration};
use utils;
use simulation::GeneticAlgorithm;
use game::{Game, FieldState};
use gen::messages::UiMessage;
use entities::{Beetles, BeetleBuilder};
use simulation::speed_ga::SpeedGA;
use simulation::battle_ga::BattleGA;
use simulation::food_ga::FoodGA;
use simulation::Simulate;
use simulation::fight_simulation::FightSimulation;
use ui::UI;
use cgmath::Vector2;

pub struct MessageHandler {
}

impl MessageHandler {
    pub fn new() -> MessageHandler {
        MessageHandler {}
    }

    pub fn handle_message(
            &mut self, mut game: &mut Game, ui: &UI, message: UiMessage) -> bool {

        let mut done = false;

        if message.has_select_beetle() {
            game.select_beetle(message.get_select_beetle().get_beetle_id());
        }
        else if message.has_select_all_in_area() {
            let x1 = message.get_select_all_in_area().get_x1();
            let y1 = message.get_select_all_in_area().get_y1();
            let x2 = message.get_select_all_in_area().get_x2();
            let y2 = message.get_select_all_in_area().get_y2();

            game.select_all_in_area(x1, y1, x2, y2);
        }
        else if message.has_selected_move_command() {
            game.selected_move_command(
                message.get_selected_move_command().get_x(),
                message.get_selected_move_command().get_y());
        }
        else if message.has_deselect_all_beetles() {
            game.deselect_all_beetles();
        }
        else if message.has_create_beetle() {
            let beetle = BeetleBuilder::new()
                //.speed_units_per_tick(converted_speed)
                //.rotation_radians_per_tick(Rad(converted_rotation))
                .x_pos(message.get_create_beetle().get_x())
                .y_pos(message.get_create_beetle().get_y())
                .build();
            game.add_beetle(beetle);
        }
        else if message.has_selected_interact_command() {
            game.selected_interact_command(
                message.get_selected_interact_command().get_target_id());
        }
        else if message.has_terminate() {
            done = true;
        }
        else if message.has_run_speed_simulation() {

            let mut simulation = SpeedGA::new(game, &ui);
            simulation.run();
        }
        else if message.has_run_battle_simulation() {

            // TODO: so many clones
            // also copypasta from below
            let mut population = Beetles::new();
            let ids = game.field_state.selected_beetles.clone();

            for beetle_id in &ids  {
                if let Some(beetle) = game.field_state.beetles.get(&beetle_id) {
                    population.insert(beetle.id, beetle.clone());
                }
            }

            {
                let mut ga = BattleGA::new(population, &ui);
                ga.run();
                population = ga.get_population().clone();
            }

            for (id, (_, new_beetle)) in ids.iter().zip(population.into_iter()) {
                if let Some(beetle) = game.field_state.beetles.get_mut(&id) {
                    let pos = (*beetle).position;
                    *beetle = new_beetle;
                    (*beetle).id = *id;
                    (*beetle).position = pos;
                    (*beetle).team_id = 0;
                    (*beetle).direction = Vector2::new(1.0, 0.0);
                }
            }
            //let mut ga = BattleGA::new(game.field_state.beetles.clone(), &ui);
            //ga.run();

            //game.set_population(ga.get_population().clone());
        }
        else if message.has_run_food_ga() {

            // TODO: so many clones
            let mut population = Beetles::new();
            let ids = game.field_state.selected_beetles.clone();

            for beetle_id in &ids  {
                if let Some(beetle) = game.field_state.beetles.get(&beetle_id) {
                    population.insert(beetle.id, beetle.clone());
                }
            }

            {
                let mut ga = FoodGA::new(population, &ui);
                ga.run();
                population = ga.get_population().clone();
            }

            for (id, (_, new_beetle)) in ids.iter().zip(population.into_iter()) {
                if let Some(beetle) = game.field_state.beetles.get_mut(&id) {
                    let pos = (*beetle).position;
                    *beetle = new_beetle;
                    (*beetle).id = *id;
                    (*beetle).position = pos;
                    (*beetle).team_id = 1;
                    (*beetle).direction = Vector2::new(-1.0, 0.0);
                }
            }
        }
        else if message.has_run_fight_simulation() {

            let check_done_callback = |state: &FieldState| {
                // whatever the first beetle's team is, make sure there are no
                // enemies left.
                if let Some(beetle) = state.beetles.iter().next() {
                    let team_id = beetle.1.team_id;

                    for other in state.beetles.values() {
                        if other.team_id != team_id {
                            return false;
                        }
                    }

                    true
                }
                else {
                    true
                }
            };
            let mut sim = FightSimulation::new(&mut game, check_done_callback);
            sim.set_tick_callback(|state| {
                ui.update_game_state(&state);
                //println!("{:?}", ui);
                thread::sleep(Duration::from_millis(utils::SIMULATION_PERIOD_MS));
            });
            sim.run();
        }
        else if message.has_create_formation() {
            game.create_formation();
        }

        return done;
    }
}
