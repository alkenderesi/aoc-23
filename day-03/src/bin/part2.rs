use std::iter::zip;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    // Split the input into multiple lines
    let lines: Vec<&str> = input.lines().collect();

    // Parse numbers and symbols
    let numbers = find_numbers(&lines);
    let symbols = find_symbols(&lines, &'.');

    // Filter gears and multiply their adjacent part numbers in one step, then sum these up
    let gear_product_sum: u32 = symbols
        .iter()
        .map(|symbol| calculate_gear_product(symbol, &numbers))
        .sum();

    gear_product_sum.to_string()
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

struct Symbol {
    position: Point,
    value: char,
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

fn find_symbols(lines: &Vec<&str>, whitespace: &char) -> Vec<Symbol> {
    let mut symbols: Vec<Symbol> = Vec::new();

    for (x, &line) in lines.iter().enumerate() {
        for (y, chr) in line.chars().enumerate() {
            if chr != *whitespace && !chr.is_digit(10) {
                symbols.push(Symbol {
                    position: Point { x, y },
                    value: chr,
                });
            }
        }
    }

    symbols
}

fn calculate_gear_product(symbol: &Symbol, numbers: &Vec<Number>) -> u32 {
    if symbol.value != '*' {
        return 0;
    }

    let mut adjacent_numbers: Vec<&Number> = Vec::new();

    for number in numbers {
        // Check for adjacency
        if number.start.x.abs_diff(symbol.position.x) <= 1
            && (number.start.y.abs_diff(symbol.position.y) <= 1
                || number.end.y.abs_diff(symbol.position.y) <= 1
                || number.start.y < symbol.position.y && number.end.y > symbol.position.y)
        {
            adjacent_numbers.push(number);
        }
    }

    if adjacent_numbers.len() == 2 {
        return adjacent_numbers
            .iter()
            .map(|&number| number.value)
            .product();
    }

    0
}
