fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    // Split the input into multiple lines
    let calibration_document = input.lines();

    // Extract the values from each line and sum them up
    let calibration_value: usize = calibration_document.map(extract_value).sum();

    // Return the solution as a string
    calibration_value.to_string()
}

fn find_first_digit(line: &str, digits: &Vec<char>, spelled_digits: &Vec<&str>) -> usize {
    // Find the index of the first occurrence of each digit
    let digit_indices: Vec<usize> = digits
        .iter()
        .map(|digit| match line.find(*digit) {
            Some(digit) => digit,
            None => usize::MAX,
        })
        .collect();

    // Find the index of the first occurrence of each spelled digit
    let spelled_digit_indices: Vec<usize> = spelled_digits
        .iter()
        .map(|digit| match line.find(*digit) {
            Some(digit) => digit,
            None => usize::MAX,
        })
        .collect();

    // Find the lowest digit index
    let mut min_index = line.len();
    let mut min_value = usize::MAX;

    for (index, &value) in digit_indices.iter().enumerate() {
        if value < min_value {
            min_index = index;
            min_value = value;
        }
    }
    for (index, &value) in spelled_digit_indices.iter().enumerate() {
        if value < min_value {
            min_index = index;
            min_value = value;
        }
    }

    min_index + 1
}

fn find_last_digit(line: &str, digits: &Vec<char>, spelled_digits: &Vec<&str>) -> usize {
    // Find the index of the last occurrence of each digit
    let digit_indices: Vec<i32> = digits
        .iter()
        .map(|digit| match line.rfind(*digit) {
            Some(digit) => digit as i32,
            None => -1,
        })
        .collect();

    // Find the index of the last occurrence of each spelled digit
    let spelled_digit_indices: Vec<i32> = spelled_digits
        .iter()
        .map(|digit| match line.rfind(*digit) {
            Some(digit) => digit as i32,
            None => -1,
        })
        .collect();

    // Find the highest digit index
    let mut max_index: i32 = -1;
    let mut max_value: i32 = -1;

    for (index, &value) in digit_indices.iter().enumerate() {
        if value > max_value {
            max_index = index as i32;
            max_value = value;
        }
    }
    for (index, &value) in spelled_digit_indices.iter().enumerate() {
        if value > max_value {
            max_index = index as i32;
            max_value = value;
        }
    }

    max_index as usize + 1
}

fn extract_value(line: &str) -> usize {
    // Store valid digits
    let digits = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let spelled_digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    // Find first and last digits
    find_first_digit(line, &digits, &spelled_digits) * 10
        + find_last_digit(line, &digits, &spelled_digits)
}
