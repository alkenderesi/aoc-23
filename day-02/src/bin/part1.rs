use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    // Split the input into multiple lines
    let games = input.lines();

    // Elf bag cube config
    let cube_config: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    // Parse the games, filter them based on the cube config, and sum their IDs
    let valid_game_id_sum: u32 = games
        .map(game_parser)
        .filter(|game| is_valid_game(game, &cube_config))
        .map(|game| game.id)
        .sum();

    valid_game_id_sum.to_string()
}

struct Game {
    id: u32,
    rounds: Vec<HashMap<String, u32>>,
}

fn game_parser(text: &str) -> Game {
    // Split text in half at the colon
    let Some((id, rounds)) = text.split_once(':') else {
        panic!("No colon found in: {text}");
    };

    // Parse each half to create a game
    Game {
        id: id_parser(id),
        rounds: rounds.split(';').map(round_parser).collect(),
    }
}

fn id_parser(text: &str) -> u32 {
    // Find the index of the space character
    let Some(space_index) = text.find(' ') else {
        panic!("No space found in: {text}");
    };

    // Parse the ID starting after the space
    match text[space_index + 1..].parse::<u32>() {
        Ok(id) => id,
        Err(_e) => panic!("No parsable ID in: {text}"),
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

fn is_valid_game(game: &Game, cube_config: &HashMap<&str, u32>) -> bool {
    for round in &game.rounds {
        for (color, count) in round {
            let Some(limit) = cube_config.get(color.as_str()) else {
                // Invalid cube color
                return false;
            };
            // More cubes than allowed
            if count > limit {
                return false;
            }
        }
    }
    true
}
