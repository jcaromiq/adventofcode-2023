use std::fs;
use adventOfCode2023::read_from_file;

fn main() {
    let calibrations = read_from_file("src/day1/input.txt");
    let value = calibration_sum(calibrations);
    println!("{value}");
}

fn calibration_sum(calibrations: Vec<String>) -> usize {
    calibrations
        .iter()
        .map(|value| calibration_value(value))
        .sum()
}

fn calibration_value(value: &str) -> usize {
    let normalized: Vec<char> = normalize(value);

    let only_numbers: Vec<&char> = normalized.iter().filter(|v| v.is_numeric()).collect();

    let first = only_numbers.first().unwrap_or(&&'0');
    let last = only_numbers.last().unwrap_or(&&'0');

    format!("{}{}", first, last).parse().unwrap()
}

fn normalize(value: &str) -> Vec<char> {
    let mut value = value.to_lowercase();

    insert_into(&mut value, "one", '1');
    insert_into(&mut value, "two", '2');
    insert_into(&mut value, "three", '3');
    insert_into(&mut value, "four", '4');
    insert_into(&mut value, "five", '5');
    insert_into(&mut value, "six", '6');
    insert_into(&mut value, "seven", '7');
    insert_into(&mut value, "eight", '8');
    insert_into(&mut value, "nine", '9');

    value.chars().collect()
}

fn insert_into(value: &mut String, find_pattern: &str, insert: char) {
    while value.contains(find_pattern) {
        match value.find(find_pattern) {
            None => {}
            Some(start) => {
                value.insert(start + 1, insert);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::{calibration_sum, calibration_value};

    #[test]
    fn get_calibration_value_from_input_with_only_numbers() {
        let value = calibration_value("1234556");
        assert_eq!(value, 16);
    }

    #[test]
    fn get_calibration_value_from_input_with_only_one_numbers() {
        let value = calibration_value("6");
        assert_eq!(value, 66);
    }

    #[test]
    fn get_calibration_value_from_input_with_numbers_spelled_out_with_letters() {
        let value = calibration_value("eightwothree");
        assert_eq!(value, 83);
    }

    #[test]
    fn get_calibration_value_from_input_with_letters() {
        let value = calibration_value("a234556j");
        assert_eq!(value, 26);
    }

    #[test]
    fn get_calibration_value_from_input_with_repeated_letters() {
        let value = calibration_value("two1ninetwo");
        assert_eq!(value, 22);
    }

    #[test]
    fn get_calibration_sum_of_statement() {
        let calibrations = vec![
            String::from("1abc2"),
            String::from("pqr3stu8vwx"),
            String::from("a1b2c3d4e5f"),
            String::from("treb7uchet"),
        ];


        let value = calibration_sum(calibrations);

        assert_eq!(value, 142);
    }

    #[test]
    fn get_calibration_sum_of_statement_spelled_out_with_letters() {
        let calibrations = vec![
            String::from("two1ninetwo"),
            String::from("eightwothree"),
            String::from("abcone2threexyz"),
            String::from("xtwone3four"),
            String::from("4nineeightseven2"),
            String::from("zoneight234"),
            String::from("7pqrstsixteen"),
        ];

        let value = calibration_sum(calibrations);

        assert_eq!(value, 274);
    }
}
