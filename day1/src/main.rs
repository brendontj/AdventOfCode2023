use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("");
    let contents_lines = contents.split("\n");
    let mut calibration_values_sum: i32 = 0;
    for line in contents_lines {
        let mut left_number: char = '0';
        for l in line.chars() {
            if l.is_numeric() {
                left_number = l;
                break;
            }
        }

        let mut right_number: char = '0';
        for r in line.chars().rev() {
            if r.is_numeric() {
                right_number = r;
                break;
            }
        }

        let mut number_str = String::new();
        number_str.push_str(&left_number.to_string());
        number_str.push_str(&right_number.to_string());

        match number_str.parse::<i32>() {
            Ok(n) => calibration_values_sum += n,
            Err(err) => {
                println!("Error {}", err)
            }
        };
    }
    println!("{}", calibration_values_sum);
}
