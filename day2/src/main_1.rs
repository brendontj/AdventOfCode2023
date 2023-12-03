use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

struct Subset {
    blue: i32,
    red: i32,
    green: i32,
}

impl FromStr for Subset {
    fn from_str(mut subset: &str) -> Result<Self, Self::Err> {
        let mut colors: HashSet<String> = HashSet::new();
        colors.insert("green".to_string());
        colors.insert("blue".to_string());
        colors.insert("red".to_string());

        subset = subset.trim();

        let subset_chars: Vec<char> = subset.chars().collect();
        let mut current_substr = String::new();
        let mut i: i32 = subset_chars.len() as i32 - 1;
        let mut g: i32 = 0;
        let mut b: i32 = 0;
        let mut r: i32 = 0;
       
        while i > 0 { 
            if subset_chars[i as usize] == ' ' || subset_chars[i as usize] == ','  { 
                i -= 1;
                continue;
            } 

            current_substr.push(subset_chars[i as usize]); 
            if colors.contains(&current_substr.as_str().chars().rev().collect::<String>()) {
                let mut next_idx = i - 2;
                let mut number_reverse = String::new();
                while next_idx >= 0 && subset_chars[next_idx as usize].is_numeric() { 
                    number_reverse.push(subset_chars[next_idx as usize]);
                    next_idx -= 1;
                }
                
                let number_as_string : String = number_reverse.as_str().chars().rev().collect();
                let correct_token =  &current_substr.as_str().chars().rev().collect::<String>();

                if correct_token == "green" {
                    g = number_as_string.as_str().parse::<i32>().unwrap();
               } else if correct_token == "red" {
                    r =  number_as_string.as_str().parse::<i32>().unwrap();
               } else if correct_token == "blue" {
                   b = number_as_string.as_str().parse::<i32>().unwrap();
               }

               i = next_idx;
               current_substr = String::new();
               continue;
            }
            i -= 1;
        }
        Ok(Subset { blue: b, red: r, green: g})
    }

    type Err = std::num::ParseIntError;
}

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Shoud have a file called input.txt");
    let content_lines = contents.split("\n"); 
    let mut sum_games_id: i32 = 0;
    for line in content_lines {
        let line_split: Vec<&str> = line.split(':').collect();

        let mut game_id_str: String = String::new();
        for c in line_split[0].chars().rev() {
            if c.is_numeric() {
                game_id_str.push(c);
                continue;
            }
            break;
        }
        let game_id_reversed: String = game_id_str.chars().rev().collect(); 

        let subsets = line_split[1].split(";");

        let mut valid_subset = true;
        for s in subsets {
            match Subset::from_str(s) {
                Ok(subset) => { if subset.blue > 14 || subset.green > 13 || subset.red > 12 {  valid_subset = false; break; }}
                Err(_) => { println!("{} is not a valid subset", s)}
            }
        }

        if valid_subset { 
            sum_games_id += game_id_reversed.parse::<i32>().unwrap();
        }
    }
    println!("{}", sum_games_id);
}
