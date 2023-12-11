use std::collections::HashSet;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let scratchcards = parse_scratchcards(&lines);
    let points: u32 = scratchcards.iter().map(|card| card.points()).sum();

    points.to_string()
}

struct Scratchcard {
    winning_numbers: HashSet<u32>,
    your_numbers: HashSet<u32>,
}

impl Scratchcard {
    fn points(&self) -> u32 {
        let match_count = self
            .winning_numbers
            .intersection(&self.your_numbers)
            .count();
        if match_count == 0 {
            return 0;
        }
        2_u32.pow(match_count as u32 - 1)
    }
}

fn parse_scratchcards(lines: &Vec<&str>) -> Vec<Scratchcard> {
    lines
        .iter()
        .map(|line| match line.split_once(':') {
            Some((_card_id, card_numbers)) => match card_numbers.split_once('|') {
                Some((winning_numbers, your_numbers)) => Scratchcard {
                    winning_numbers: parse_numbers(winning_numbers),
                    your_numbers: parse_numbers(your_numbers),
                },
                None => panic!("No vertical bar in: {card_numbers}"),
            },
            None => panic!("No colon in: {line}"),
        })
        .collect()
}

fn parse_numbers(text: &str) -> HashSet<u32> {
    text.split(' ')
        .filter(|num| !num.is_empty())
        .map(|num| match num.parse::<u32>() {
            Ok(num) => num,
            Err(_e) => panic!("No parsable number in: {num}"),
        })
        .collect()
}
