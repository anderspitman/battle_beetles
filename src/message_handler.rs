use game::Game;
use gen::messages::UiMessage;
use beetle::BeetleBuilder;

pub struct MessageHandler {
    //game: &'a mut Game,
}

impl MessageHandler {
    //pub fn new(game: &mut Game) -> MessageHandler {
    pub fn new() -> MessageHandler {
        //MessageHandler { game }
        MessageHandler {}
    }

    pub fn handle_message(&mut self, game: &mut Game, message: UiMessage) -> bool {

        let mut done = false;

        if message.has_select_beetle() {
            game.select_beetle(message.get_select_beetle().get_beetle_id());
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
                message.get_selected_interact_command().get_beetle_id());
        }
        else if message.has_terminate() {
            done = true;
        }

        return done;
    }
}
