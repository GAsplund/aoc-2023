use std::fs::read_to_string;

fn read_file() -> Vec<String> {
    read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect()
}

fn get_first_number(string: impl Iterator<Item = char>, last: bool) -> Option<(usize, char)> {
    if last {
        string
            .enumerate()
            .collect::<Vec<(usize, char)>>()
            .into_iter()
            .rev()
            .find(|c| c.1.is_numeric())
    } else {
        string.enumerate().find(|c| c.1.is_numeric())
    }
}

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_number_string_parsed(number: &str) -> u32 {
    (NUMBERS
        .iter()
        .position(|&r| r == number)
        .unwrap_or(usize::MAX)
        .wrapping_add(1)) as u32
}

fn get_substring_numbers(string: &str) -> Vec<(usize, &str)> {
    let mut nums = NUMBERS
        .iter()
        .flat_map(|n| string.match_indices(n))
        .collect::<Vec<(usize, &str)>>();
    nums.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    nums
}

fn get_first_number_string(string: &str, last: bool) -> Option<(usize, u32)> {
    let res = get_substring_numbers(string);
    let res = if last {
        res.iter().last()
    } else {
        res.iter().next()
    };

    if let Some(r) = res {
        Some((r.0, get_number_string_parsed(r.1)))
    } else {
        None
    }
}

fn get_digit(input: &str) -> u32 {
    let first = get_first_number(input.chars().into_iter(), false);
    let last = get_first_number(input.chars().into_iter(), true);

    let first_digit = first.unwrap_or((0, '0')).1.to_digit(10).unwrap_or(0);
    let second_digit = last.unwrap_or((0, '0')).1.to_digit(10).unwrap_or(0);

    10 * first_digit + second_digit
}

fn get_digit_string(input: &str) -> u32 {
    let start_1 = get_first_number_string(input, false);
    let start_2 = get_first_number(input.chars().into_iter(), false);

    let first_digit = match (start_1, start_2) {
        (Some(first), Some(second)) => {
            if first.0 < second.0 {
                first.1
            } else {
                second.1.to_digit(10).unwrap_or(0)
            }
        }
        (Some(first), None) => first.1,
        (None, Some(second)) => second.1.to_digit(10).unwrap_or(0),
        (None, None) => 0,
    };

    let end_1 = get_first_number_string(input, true);
    let end_2 = get_first_number(input.chars().into_iter(), true);

    let last_digit = match (end_1, end_2) {
        (Some(first), Some(second)) => {
            if first.0 > second.0 {
                first.1
            } else {
                second.1.to_digit(10).unwrap_or(0)
            }
        }
        (Some(first), None) => first.1,
        (None, Some(second)) => second.1.to_digit(10).unwrap_or(0),
        (None, None) => 0,
    };

    10 * first_digit + last_digit
}

fn main() {
    let calibration_values: Vec<u32> = read_file().iter().map(|l| get_digit(l)).collect();
    let calibration_sum: u32 = calibration_values.iter().sum();
    println!("Solution 1: {}", calibration_sum);

    let calibration_strings: Vec<u32> = read_file().iter().map(|l| get_digit_string(l)).collect();
    let calibration_strings_sum: u32 = calibration_strings.iter().sum();
    println!("Solution 2: {}", calibration_strings_sum);
}
