fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let data: Vec<Vec<&str>> = input
        .lines()
        .collect::<Vec<&str>>()
        .split(|&line| line == "")
        .map(|line: &[&str]| line.to_vec())
        .collect();

    let mut data_iter = data.iter();

    let seeds = match data_iter.next() {
        Some(seed_data) => find_seeds(seed_data),
        None => panic!("Empty input"),
    };

    let mut maps: Vec<Map> = Vec::new();
    for map_data in data_iter {
        let mut map_data_iter = map_data.iter();
        let _header = map_data_iter.next();
        let mut conversions: Vec<Conversion> = Vec::new();
        for conversion_data in map_data_iter {
            let values = parse_numbers(conversion_data);
            conversions.push(Conversion {
                destination_range_start: *values.get(0).unwrap(),
                source_range_start: *values.get(1).unwrap(),
                range_length: *values.get(2).unwrap(),
            })
        }
        maps.push(Map { conversions })
    }

    let mut min_location = u64::MAX;

    for seed in &seeds {
        let mut value = *seed;
        for map in &maps {
            value = map.convert(&value);
        }
        if value < min_location {
            min_location = value;
        }
    }

    min_location.to_string()
}

struct Conversion {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

struct Map {
    conversions: Vec<Conversion>,
}

impl Map {
    fn convert(&self, value: &u64) -> u64 {
        for conversion in &self.conversions {
            if value >= &conversion.source_range_start
                && value < &(conversion.source_range_start + conversion.range_length)
            {
                return value - conversion.source_range_start + conversion.destination_range_start;
            }
        }

        *value
    }
}

fn parse_numbers(text: &str) -> Vec<u64> {
    text.split(' ')
        .filter(|num| !num.is_empty())
        .map(|num| match num.parse::<u64>() {
            Ok(num) => num,
            Err(_e) => panic!("No parsable number in: {num}"),
        })
        .collect()
}

fn find_seeds(seed_data: &Vec<&str>) -> Vec<u64> {
    match seed_data.get(0) {
        Some(&first_line) => match first_line.split_once(": ") {
            Some((_label, seeds)) => parse_numbers(seeds),
            None => panic!("No colon in: {first_line}"),
        },
        None => panic!("No seed data found"),
    }
}
