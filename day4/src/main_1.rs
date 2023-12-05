use std::{fs, collections::HashSet};

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Should have a file called input.txt");
    let lines : Vec<&str> = contents.split("\n").collect();
    let mut count = 0;
    for l in lines {
        let line_data :Vec<&str>= l.split(":").collect();
        let line_info :Vec<&str> = line_data[1 as usize].split("|").collect();

        let winning_numbers = line_info[0 as usize];
        let cards = line_info[1 as usize];
 
        let q = qty_winning_numbers(cards, create_hash_set(winning_numbers)); 
        if q == 0 {continue;}
        count += 2_i32.pow(q as u32-1);
    }
    println!("{}", count);
}

fn qty_winning_numbers(cards: &str, hs: HashSet<i32>) -> i32 {
    let mut count: i32 = 0;
    let mut cn = String::new();
    for c in cards.chars() {
        if c == ' ' {
            if hs.contains(&cn.parse::<i32>().unwrap_or(0)) {
                count += 1;
            }
            cn = String::new();
            continue;
        }
        cn.push(c);
    }
    
    if !cn.is_empty() && hs.contains(&cn.parse::<i32>().unwrap_or(0)) { count += 1 } 
    return count
}

fn create_hash_set(winning_numbers: &str) -> HashSet<i32> {
    let mut hs: HashSet<i32> = HashSet::new();
    let mut cn = String::new();
    for w in winning_numbers.chars() {
        if w == ' '{
            if !cn.is_empty() {
                hs.insert(cn.parse::<i32>().unwrap());
            }
            cn = String::new();
            continue;
        }
        cn.push(w);
    }

    return hs;
}
