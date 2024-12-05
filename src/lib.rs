use std::{fs::read_to_string, str::FromStr};
use regex::Regex;

pub fn result_uncorrupt_file(input_path: &str) -> u32 {
    let input = read_file(input_path);

    find_figures_to_multiply(&input).iter()
    .fold(0, |acc, next_value| acc + next_value.0 * next_value.1)
}

pub fn result_uncorrupt_file_with_closures(input_path: &str) -> u32 {
    let input = read_file(input_path);
    
    find_figures_to_multiply_with_closures(&input).iter()
    .fold(0, |acc, next_value| acc + next_value.0 * next_value.1)
}

fn read_file(input_path: &str) -> String {
    read_to_string(input_path).unwrap()
}

fn find_figures_to_multiply(input: &str) -> Vec<ValidNumbers> {
    let regex = Regex::new("mul\\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\\)").unwrap();
    regex.captures_iter(input).map(|value| {
        let first_number = FromStr::from_str(value.name("first").unwrap().as_str()).unwrap();
        let second_number = FromStr::from_str(value.name("second").unwrap().as_str()).unwrap();
        ValidNumbers(first_number, second_number)

    }).collect()
}

fn find_figures_to_multiply_with_closures(input: &str) -> Vec<ValidNumbers> {
    let mut valid_numbers: Vec<ValidNumbers> = Vec::new();

    input.split("do")
        .map(|part| part.to_string())
        .filter(|part| !part.starts_with("n't()"))
        .for_each(|part| {
            valid_numbers.extend(find_figures_to_multiply(&part));
        });
    
    valid_numbers
}

#[derive(PartialEq)]
#[derive(Debug)]
struct ValidNumbers(u32, u32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_numbers_to_multiply_with_closures_return_one_valid_number_when_one_path_one_do_dont_closure() {
        assert_eq!(find_figures_to_multiply_with_closures("mul(5,5)don't()mul(5,6)"), vec![ValidNumbers(5, 5)]);
    }

    #[test]
    fn should_find_numbers_to_multiply_with_closures_return_one_valid_number_when_one_path_no_closure() {
        assert_eq!(find_figures_to_multiply_with_closures("mul(5,5)"), vec![ValidNumbers(5, 5)]);
    }

    #[test]
    fn should_read_file() {
        assert_eq!(read_file("tests/resources/puzzle.txt"), "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
    }

    #[test]
    fn should_find_numbers_to_multiply_return_empty_vec_when_empty_input() {
        assert_eq!(find_figures_to_multiply(""), Vec::new());
    }

    #[test]
    fn should_find_numbers_to_multiply_return_one_valid_number_when_one_path() {
        assert_eq!(find_figures_to_multiply("mul(5,5)"), vec![ValidNumbers(5, 5)]);
    }

    #[test]
    fn should_find_numbers_to_multiply_return_empty_number_when_no_valid_path() {
        assert_eq!(find_figures_to_multiply("mul[5,5)"), Vec::new());
    }

    #[test]
    fn should_find_numbers_to_multiply_return_empty_number_when_no_valid_path_because_of_spaces() {
        assert_eq!(find_figures_to_multiply("mul ( 5 , 5 )"), Vec::new());
    }

    #[test]
    fn should_find_numbers_to_multiply_return_multiple_valid_numbers_number_when_multiple_valid_path() {
        assert_eq!(find_figures_to_multiply("eeddmul(5,5)&&&&mul(1,1)&&&"), vec![ValidNumbers(5, 5), ValidNumbers(1, 1)]);
    }

    #[test]
    fn should_find_numbers_to_multiply_return_multiple_valid_numbers_number_when_multiple_valid_path_two_digits() {
        assert_eq!(find_figures_to_multiply("eeddmul(5,55)&&&&mul(3,44)&&&"), vec![ValidNumbers(5, 55), ValidNumbers(3, 44)]);
    }
}