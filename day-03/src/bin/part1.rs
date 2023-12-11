use std::iter::zip;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    // Split the input into multiple lines
    let lines: Vec<&str> = input.lines().collect();

    // Parse numbers and symbols
    let numbers = find_numbers(&lines);
    let symbols = find_symbols(&lines, &'.');

    // Filter part numbers, get their value, and sum them up
    let part_number_sum: u32 = numbers
        .iter()
        .filter(|&number| is_part_number(number, &symbols))
        .map(|part_number| part_number.value)
        .sum();

    part_number_sum.to_string()
}

struct Point {
    x: usize,
    y: usize,
}

struct Number {
    start: Point,
    end: Point,
    value: u32,
}

fn find_numbers(lines: &Vec<&str>) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();

    let mut starts: Vec<usize> = Vec::new();
    let mut ends: Vec<usize> = Vec::new();

    let mut curr_digit: bool;
    let mut prev_digit: bool;

    for (x, &line) in lines.iter().enumerate() {
        prev_digit = false;

        for (y, chr) in line.chars().enumerate() {
            curr_digit = chr.is_digit(10);

            if curr_digit && !prev_digit {
                starts.push(y);
            } else if !curr_digit && prev_digit {
                ends.push(y);
            }

            prev_digit = curr_digit;
        }

        if prev_digit {
            ends.push(line.len());
        }

        for (start, end) in zip(&starts, &ends) {
            match line[*start..*end].parse::<u32>() {
                Ok(value) => numbers.push(Number {
                    start: Point { x, y: *start },
                    end: Point { x, y: *end - 1 },
                    value,
                }),
                Err(_e) => panic!("No parsable number in: {}", &line[*start..*end]),
            }
        }

        starts.clear();
        ends.clear();
    }

    numbers
}

fn find_symbols(lines: &Vec<&str>, whitespace: &char) -> Vec<Point> {
    let mut symbols: Vec<Point> = Vec::new();

    for (x, &line) in lines.iter().enumerate() {
        for (y, chr) in line.chars().enumerate() {
            if chr != *whitespace && !chr.is_digit(10) {
                symbols.push(Point { x, y });
            }
        }
    }

    symbols
}

fn is_part_number(number: &Number, symbols: &Vec<Point>) -> bool {
    for symbol in symbols {
        // Check for adjacency
        if number.start.x.abs_diff(symbol.x) <= 1
            && (number.start.y.abs_diff(symbol.y) <= 1
                || number.end.y.abs_diff(symbol.y) <= 1
                || number.start.y < symbol.y && number.end.y > symbol.y)
        {
            return true;
        }
    }

    false
}
