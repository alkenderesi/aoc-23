use std::cmp;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    // Split the input into multiple lines
    let games = input.lines();

    // Parse the games, calculate their cube power, and sum them up
    let power_sum: u32 = games
        .map(game_parser)
        .map(|game| calculate_cube_power(&game))
        .sum();

    power_sum.to_string()
}

struct Game {
    rounds: Vec<HashMap<String, u32>>,
}

fn game_parser(text: &str) -> Game {
    // Split text in half at the colon
    let Some((_id, rounds)) = text.split_once(':') else {
        panic!("No colon found in: {text}");
    };

    // Parse second half to create a game
    Game {
        rounds: rounds.split(';').map(round_parser).collect(),
    }
}

fn round_parser(text: &str) -> HashMap<String, u32> {
    let mut round: HashMap<String, u32> = HashMap::new();

    // Split round into cube types and iterate over them
    for cube in text.split(',') {
        // Split cube into a cube count and a cube color
        let (count, color) = match cube[1..].split_once(' ') {
            Some((count, color)) => (
                // Parse the cube count
                match count.parse::<u32>() {
                    Ok(count) => count,
                    Err(_e) => panic!("No parsable cube count found in: {cube}"),
                },
                // Convert the cube color
                color.to_string(),
            ),
            None => panic!("No space found in: {cube}"),
        };

        // Store cube data in the map
        round.insert(color, count);
    }

    round
}

fn calculate_cube_power(game: &Game) -> u32 {
    let mut cube_counter: HashMap<&String, &u32> = HashMap::new();

    for round in &game.rounds {
        for (color, count) in round {
            // Update value at key,
            // or insert key and value if key doesn't exist
            cube_counter
                .entry(color)
                .and_modify(|max_count| *max_count = cmp::max(*max_count, count))
                .or_insert(count);
        }
    }

    cube_counter.values().map(|count| *count).product()
}
