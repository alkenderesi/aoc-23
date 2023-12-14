use std::iter::zip;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let boat_races = process_input(&input);

    boat_races
        .iter()
        .map(|boat_race| boat_race.record_opportunities())
        .product::<u32>()
        .to_string()
}

struct BoatRace {
    time: u32,
    distance: u32,
}

impl BoatRace {
    fn record_opportunities(&self) -> u32 {
        let mut opportunities = 0;
        let mut travel_time;
        let mut travel_distance;

        for charge_time in 1..self.time {
            travel_time = self.time - charge_time;
            travel_distance = charge_time * travel_time;
            if travel_distance > self.distance {
                opportunities += 1;
            }
        }

        opportunities
    }
}

fn extract_numbers(text: &str) -> Vec<u32> {
    text.split(' ')
        .filter(|substring| !substring.is_empty())
        .map(|substring| match substring.parse::<u32>() {
            Ok(number) => number,
            Err(_e) => panic!("No parsable number in '{substring}' while processing '{text}'"),
        })
        .collect()
}

fn process_input(input: &str) -> Vec<BoatRace> {
    let mut lines = input.lines();

    let (_time_label, time_text) = lines.next().unwrap().split_once(':').unwrap();
    let (_distance_label, distance_text) = lines.next().unwrap().split_once(':').unwrap();

    zip(extract_numbers(time_text), extract_numbers(distance_text))
        .map(|(time, distance)| BoatRace { time, distance })
        .collect()
}
