use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("");
    let contents_lines = contents.split("\n");
    let mut calibration_values_sum: i32 = 0;

    let numbers = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    for line in contents_lines {
        let line_without_space = line.trim();
        if line_without_space.len() == 0 {
            continue;
        }
       
        let mut left_number = '0';
        let mut right_number = '0';
        match find_first_number(line_without_space, "asc", &numbers) {
            Some(v) => {
                left_number = v; 
            }
            _ => {
                println!(
                    "Error finding left number, with current line {}",
                    line_without_space
                )
            }
        }

        match find_first_number(line_without_space, "dsc", &numbers) {
            Some(v) => {
                right_number = v; 
            }
            _ => {
                println!(
                    "Error finding right number, with current line {}",
                    line_without_space
                )
            }
        }

        let mut number_str = left_number.to_string();
        number_str.push_str(&right_number.to_string());

        match number_str.parse::<i32>() {
            Ok(n) => calibration_values_sum += n,
            Err(err) => {
                println!("Error {}", err)
            }
        };
    }
    println!("\n{}", calibration_values_sum);
}

fn find_first_number(line: &str, order: &str, numbers: &HashMap<&str, char>) -> Option<char> {
    let mut current_string = String::new();

    let chars;
    if order == "asc" {
        chars = line.chars().collect::<Vec<char>>();
    } else {
        chars = line.chars().rev().collect::<Vec<char>>();
    }

    for c in chars {
        if c.is_numeric() {
            return Some(c);
        }

        current_string.push_str(&c.to_string());

        let iter_str = current_string.clone();
        match check_substr_has_number_word(iter_str, &numbers, order) {
            Some(v) => {
                return Some(v);
            }
            None => continue,
        }
    }

    return None;
}

fn check_substr_has_number_word(
    mut substr: String,
    numbers: &HashMap<&str, char>,
    order: &str,
) -> Option<char> {
    while substr.len() >= 3 {
        if order == "dsc" {
            let revesed_str = &substr.chars().rev().collect::<String>() as &str;
            match numbers.get(&revesed_str) {
                Some(v) => return Some(*v),
                None => {}
            }
            let _ = substr.remove(0);
        } else {
            match numbers.get(substr.as_str()) {
                Some(v) => return Some(*v),
                None => {}
            }
            let _ = substr.remove(0);
        }
    }
    return None;
}
