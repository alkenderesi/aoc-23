use std::ops::Range;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let (seed_ranges, maps) = process_input(input);

    let mut min_location = u128::MAX;

    // Brute force
    for seed_range in seed_ranges {
        for seed in seed_range {
            let mut value = seed;
            for map in &maps {
                value = evaluate_map(map, &value);
            }
            if value < min_location {
                min_location = value;
            }
        }
    }

    min_location.to_string()
}

struct Conversion {
    destination_range_start: u128,
    source_range_start: u128,
    range_length: u128,
}

fn extract_numbers(text: &str) -> Vec<u128> {
    text.split(' ')
        .map(|substring| match substring.parse::<u128>() {
            Ok(number) => number,
            Err(_e) => panic!("No parsable number in '{substring}' while processing '{text}'"),
        })
        .collect()
}

fn extract_seed_ranges(seed_batch: &Vec<&str>) -> Vec<Range<u128>> {
    let (_label, seed_text) = seed_batch.get(0).unwrap().split_once(": ").unwrap();

    let seed_values = extract_numbers(seed_text);
    let mut seed_value_iterator = seed_values.iter();

    let mut seed_ranges = Vec::new();

    for _ in 0..seed_values.len() / 2 {
        let start = *seed_value_iterator.next().unwrap();
        let end = start + *seed_value_iterator.next().unwrap();
        seed_ranges.push(start..end)
    }

    seed_ranges
}

fn extract_map(map_batch: &Vec<&str>) -> Vec<Conversion> {
    let mut map_iterator = map_batch.iter();

    let _label = map_iterator.next();

    map_iterator
        .map(|&conversion_text| extract_numbers(conversion_text))
        .map(|conversion_values| Conversion {
            destination_range_start: *conversion_values.get(0).unwrap(),
            source_range_start: *conversion_values.get(1).unwrap(),
            range_length: *conversion_values.get(2).unwrap(),
        })
        .collect()
}

fn process_input(input: &str) -> (Vec<Range<u128>>, Vec<Vec<Conversion>>) {
    let batches: Vec<Vec<&str>> = input
        .lines()
        .collect::<Vec<&str>>()
        .split(|&line| line == "")
        .map(|line| line.to_vec())
        .collect();

    let mut batch_iterator = batches.iter();

    let seed_ranges = extract_seed_ranges(batch_iterator.next().unwrap());

    let maps = batch_iterator
        .map(|map_batch| extract_map(map_batch))
        .collect::<Vec<Vec<Conversion>>>();

    (seed_ranges, maps)
}

fn evaluate_map(map: &Vec<Conversion>, input: &u128) -> u128 {
    for conversion in map {
        if input >= &conversion.source_range_start
            && input < &(conversion.source_range_start + conversion.range_length)
        {
            return input - conversion.source_range_start + conversion.destination_range_start;
        }
    }

    *input
}
