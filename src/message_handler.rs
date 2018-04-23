use simulation::GeneticAlgorithm;
use game::Game;
use gen::messages::UiMessage;
use beetle::BeetleBuilder;
use simulation::speed_ga::SpeedGA;
use simulation::battle_ga::BattleGA;
use ui::UI;

pub struct MessageHandler {
}

impl MessageHandler {
    pub fn new() -> MessageHandler {
        MessageHandler {}
    }

    pub fn handle_message(
            &mut self, game: &mut Game, ui: &UI, message: UiMessage) -> bool {

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

            let mut simulation = BattleGA::new(game, &ui);
            simulation.run();
        }
        else if message.has_create_formation() {
            game.create_formation();
        }

        return done;
    }
}
