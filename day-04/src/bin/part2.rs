use std::collections::HashSet;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let scratchcards = parse_scratchcards(&lines);
    let mut counter: usize = scratchcards.len();

    for (index, scratchcard) in scratchcards.iter().enumerate() {
        counter += reward_recursion(index, scratchcard, &scratchcards, 0);
    }

    counter.to_string()
}

struct Scratchcard {
    winning_numbers: HashSet<u32>,
    your_numbers: HashSet<u32>,
}

impl Scratchcard {
    fn matches(&self) -> usize {
        self.winning_numbers
            .intersection(&self.your_numbers)
            .count()
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

fn reward_recursion(
    index: usize,
    scratchcard: &Scratchcard,
    scratchcards: &Vec<Scratchcard>,
    mut counter: usize,
) -> usize {
    for i in 1..scratchcard.matches() + 1 {
        let new_index = index + i;
        let new_scratchcard = match scratchcards.get(new_index) {
            Some(scratchcard) => scratchcard,
            None => panic!("Unable to reference scratchcard at index: {new_index}"),
        };
        counter += reward_recursion(new_index, new_scratchcard, scratchcards, 1);
    }

    counter
}
