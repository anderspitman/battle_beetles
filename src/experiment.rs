//use std::collections::HashMap;
use std::thread;
use std::time::{Duration};
use std::io::prelude::*;
use std::fs::File;

use cgmath::{Vector2, InnerSpace};
//use serde_json;

use game::{Game, FieldState, Command};
use ui::UI;
use entities::{Id, Beetles};
use utils::{
    convert_value_for_sim_period, POPULATION_SIZE,
    MAX_SPEED_UNITS_PER_SECOND, ROTATION_RADIANS_PER_SECOND,
    SIMULATION_PERIOD_MS
};
use simulation::Simulate;
use simulation::fight_simulation::FightSimulation;
use simulation::GeneticAlgorithm;
use simulation::battle_ga::BattleGA;
use simulation::food_ga::FoodGA;

const NUM_ITERATIONS: usize = 1024;
const FORMATION_SPACING: f32 = 50.0;
const FORMATION_COLUMN_WIDTH: usize = 8;
const TEAM0_START_X: f32 = 50.0;
const TEAM0_START_Y: f32 = 50.0;
const TEAM1_START_X: f32 = 600.0;
const TEAM1_START_Y: f32 = 50.0;
const TEAM_SIZE: i32 = POPULATION_SIZE / 2;


#[derive(Debug)]
struct SimulationResult {
    battle_result: BattleResult,
    food_result: FoodResult,
}

#[derive(Debug)]
struct BattleResult {
    winning_team_id: Id,
    surviving_population_ratio: f32,
}

#[derive(Debug)]
struct FoodResult {
    winning_team_id: Id,
    victory_ratio: f32,
}

pub fn run_experiment(ui: &UI) {

    let mut team0_battle_file = File::create("team0_battle_victories.txt").unwrap();
    let mut team1_battle_file = File::create("team1_battle_victories.txt").unwrap();
    let mut team0_food_file = File::create("team0_food_victories.txt").unwrap();
    let mut team1_food_file = File::create("team1_food_victories.txt").unwrap();

    for i in 0..NUM_ITERATIONS {

        println!("Iter {}", i);
        let result = run_iteration(ui);

        println!("{:?}", result);

        if result.battle_result.winning_team_id == 0 {
            write!(team0_battle_file, "{}\n", result.battle_result.surviving_population_ratio).unwrap();
        }
        else if result.battle_result.winning_team_id == 1 {
            write!(team1_battle_file, "{}\n", result.battle_result.surviving_population_ratio).unwrap();
        }
        else {
            panic!("Invalid battle team id {}", result.battle_result.winning_team_id);
        }

        if result.food_result.winning_team_id == 0 {
            write!(team0_food_file, "{}\n", result.food_result.victory_ratio).unwrap();
        }
        else if result.food_result.winning_team_id == 1 {
            write!(team1_food_file, "{}\n", result.food_result.victory_ratio).unwrap();
        }
        else {
            panic!("Invalid food team id {}", result.food_result.winning_team_id);
        }
    }

    //out_file.write_all(serde_json::to_string(&experiment_result).unwrap().as_bytes()).unwrap();
}

fn run_iteration(ui: &UI) -> SimulationResult {

    let mut next_id = 0;
    let mut id_generator = || {
        next_id += 1;
        next_id
    };

    let mut population = Beetles::new();

    evolve_battle_population(&mut population, &ui, &mut id_generator);
    evolve_food_population(&mut population, &ui, &mut id_generator);

    let battle_result = run_battle_simulation(population.clone(), ui);
    let food_result = run_food_simulation(population.clone(), ui);

    SimulationResult {
        battle_result,
        food_result,
    }
}

fn run_food_simulation(population: Beetles, ui: &UI) -> FoodResult {

    let mut game = Game::new();
    game.set_population(population);

    game.add_home_base(TEAM0_START_X + 100.0, 500.0);
    game.add_home_base(TEAM1_START_X + 100.0, 500.0);

    game.add_food_source(TEAM0_START_X + 100.0, 350.0);
    game.add_food_source(TEAM1_START_X + 100.0, 350.0);

    for food_source in game.field_state.food_sources.values_mut() {
        food_source.increase_food(1_000_000);
    }

    for beetle in game.field_state.beetles.values_mut() {
        // point all beetles down
        beetle.direction = Vector2::new(0.0, 1.0);
        beetle.angle = Vector2::new(1.0, 0.0).angle(beetle.direction);
        beetle.set_command(Command::HarvestClosestFood);
    }

    for _ in 0..1000 {
        game.tick();
        ui.update_game_state(&game.field_state);
        thread::sleep(Duration::from_millis(SIMULATION_PERIOD_MS));
    }

    let mut team0_sum = 0;
    let mut team1_sum = 0;

    for beetle in game.field_state.beetles.values() {
        if beetle.team_id == 0 {
            team0_sum += beetle.food_collected;
        }
        else if beetle.team_id == 1 {
            team1_sum += beetle.food_collected;
        }
    }

    let winning_team_id;
    let victory_ratio;

    if team0_sum > team1_sum {
        winning_team_id = 0;
        victory_ratio = (team0_sum as f32) / (team1_sum as f32);
    }
    else {
        winning_team_id = 1;
        victory_ratio = (team1_sum as f32) / (team0_sum as f32);
    }

    FoodResult {
        winning_team_id,
        victory_ratio,
    }
}

fn run_battle_simulation(population: Beetles, ui: &UI) -> BattleResult {

    let mut game = Game::new();
    game.set_population(population);

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

    {
        let mut sim = FightSimulation::new(&mut game, check_done_callback);
        sim.set_tick_callback(|state| {
            ui.update_game_state(&state);
            thread::sleep(Duration::from_millis(SIMULATION_PERIOD_MS));
        });
        sim.run();
    }

    let mut winning_team_id = -1;
    if let Some(beetle) = game.field_state.beetles.iter().next() {
        winning_team_id = beetle.1.team_id;
    }

    let surviving_population_ratio = (game.field_state.beetles.len() as f32) / (TEAM_SIZE as f32);

    BattleResult {
        winning_team_id,
        surviving_population_ratio,
    }

}

fn evolve_battle_population<T: FnMut() -> Id>(
        population: &mut Beetles, ui: &UI, id_generator: &mut T) {

    let mut row = -1;

    let max_speed =
        convert_value_for_sim_period(MAX_SPEED_UNITS_PER_SECOND);

    let max_rotation =
        convert_value_for_sim_period(ROTATION_RADIANS_PER_SECOND);

    let mut battle_beetles = Game::generate_random_population(
            TEAM_SIZE, max_speed, max_rotation, id_generator);

    {
        let mut ga = BattleGA::new(battle_beetles, &ui);
        ga.run();
        battle_beetles = ga.get_population().clone();
    }

    for (i, (_, mut beetle)) in battle_beetles.into_iter().enumerate() {

        if i % FORMATION_COLUMN_WIDTH == 0 {
            row += 1;
        }

        let x_offset = (i % FORMATION_COLUMN_WIDTH) as f32;
        let y_offset = row as f32;
        let x = TEAM0_START_X + x_offset * FORMATION_SPACING;
        let y = TEAM0_START_Y + y_offset * FORMATION_SPACING;

        beetle.id = id_generator();
        beetle.position.x = x;
        beetle.position.y = y;
        beetle.direction = Vector2::new(1.0, 0.0);
        beetle.angle = Vector2::new(1.0, 0.0).angle(beetle.direction);
        beetle.team_id = 0;
        beetle.damage_inflicted = 0;
        beetle.health = beetle.max_health();
        population.insert(beetle.id, beetle);
    }
}

fn evolve_food_population<T: FnMut() -> Id>(
        population: &mut Beetles, ui: &UI, id_generator: &mut T) {

    let mut row = -1;

    let max_speed =
        convert_value_for_sim_period(MAX_SPEED_UNITS_PER_SECOND);

    let max_rotation =
        convert_value_for_sim_period(ROTATION_RADIANS_PER_SECOND);

    let mut food_beetles = Game::generate_random_population(
            TEAM_SIZE, max_speed, max_rotation, id_generator);

    {
        let mut ga = FoodGA::new(food_beetles, &ui);
        ga.run();
        food_beetles = ga.get_population().clone();
    }

    for (i, (_, mut beetle)) in food_beetles.into_iter().enumerate() {

        if i % FORMATION_COLUMN_WIDTH == 0 {
            row += 1;
        }

        let x_offset = (i % FORMATION_COLUMN_WIDTH) as f32;
        let y_offset = row as f32;
        let x = TEAM1_START_X + x_offset * FORMATION_SPACING;
        let y = TEAM1_START_Y + y_offset * FORMATION_SPACING;

        beetle.id = id_generator();
        beetle.position.x = x;
        beetle.position.y = y;
        beetle.team_id = 0;
        beetle.direction = Vector2::new(-1.0, 0.0);
        beetle.angle = Vector2::new(1.0, 0.0).angle(beetle.direction);
        beetle.team_id = 1;
        beetle.food_collected = 0;
        beetle.food_carrying = 0;
        population.insert(beetle.id, beetle);
    }
}
