fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {

    // Split the input into multiple lines
    let calibration_document = input.lines();

    // Extract the values from each line and sum them up
    let calibration_value: u32 = calibration_document.map(extract_value).sum();

    // Return the solution as a string
    calibration_value.to_string()
}

fn extract_value(line: &str) -> u32 {
    let mut first_digit: u32 = 0;
    let mut last_digit: u32 = 0;

    // Search forwards
    for chr in line.chars() {
        match chr.to_digit(10) {
            Some(digit) => {
                first_digit = digit;
                break;
            },
            None => {}
        }
    }

    // Search backwards
    for chr in line.chars().rev() {
        match chr.to_digit(10) {
            Some(digit) => {
                last_digit = digit;
                break;
            },
            None => {}
        }
    }

    first_digit * 10 + last_digit
}
