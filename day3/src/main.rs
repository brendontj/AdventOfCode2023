use std::{collections::hash_set, fs};

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Should have a file called input.txt");
    let lines: Vec<&str> = contents.split("\n").collect();

    let rows_size = lines.len();
    println!("{}", rows_size);

    let columns_size = lines[0].len();
    println!("{}", columns_size);

    let mut symbols = hash_set::HashSet::new();
    symbols.insert('$');
    symbols.insert('/');
    symbols.insert('+');
    symbols.insert('#');
    symbols.insert('%');
    symbols.insert('*');
    symbols.insert('@');
    symbols.insert('%');
    symbols.insert('=');
    symbols.insert('-');
    symbols.insert('!');
    symbols.insert('^');
    symbols.insert('&'); 
    
    let mut total_sum = 0;
    for r in 0..rows_size {
        // maybe rows_size -1
        for c in 0..columns_size {
            //maybe columns_size -1
            let character = lines[r].chars().nth(c).unwrap();
            if symbols.contains(&character) {
                let adjacent_numbers = find_adjacent_number(&lines, r, c, rows_size, columns_size);
                total_sum += adjacent_numbers.iter().map(|&x| x as i32).sum::<i32>();
            }
        }
    }
    println!("{}", total_sum);
}

fn find_adjacent_number(
    lines: &Vec<&str>,
    c_r: usize,
    c_c: usize,
    m_r: usize,
    m_c: usize,
) -> Vec<i32> {
    let mut should_check_tc = true;
    let mut should_check_tl = true;
    let mut should_check_bc = true;
    let mut should_check_bl = true;

    let mut adjacent_numbers: Vec<i32> = vec![];
    let mut i: i32 = 0;

    //check bottom-right
    while c_r + 1 <= m_r
        && c_c + 1 - (i as usize) < m_c
        && lines[c_r + 1]
            .chars()
            .nth(c_c + 1 - (i as usize))
            .unwrap()
            .is_numeric()
    {
        i += 1;
    }
    i -= 1;

    if i < 0 {
        i = 0;
    } else if i == 1 {
        should_check_bc = false;
    } else if i > 1 {
        should_check_bc = false;
        should_check_bl = false;
    }

    // starting to record the number of bottom-right
    let mut bottom_right_number = String::new();
    while c_r + 1 <= m_r
        && (c_c as i32 + 1 - i) < m_c as i32
        && lines[c_r + 1]
            .chars()
            .nth((c_c as i32 + 1 - i) as usize)
            .unwrap()
            .is_numeric()
    {
        bottom_right_number.push(lines[c_r + 1].chars().nth((c_c as i32 + 1 - i) as usize).unwrap());
        i -= 1;
    }

    match bottom_right_number.parse::<i32>() {
        Ok(n) => adjacent_numbers.push(n),
        Err(_) => adjacent_numbers.push(0),
    }

    i = 0;

    //check middle-right
    let mut middle_right_number = String::new();
    while c_c as i32 + 1 + i < m_c as i32
        && lines[c_r]
            .chars()
            .nth((c_c as i32 + 1 + i) as usize)
            .unwrap()
            .is_numeric()
    {
        middle_right_number.push(lines[c_r].chars().nth((c_c as i32 + 1 + i) as usize).unwrap());
        i += 1;
    }
    match middle_right_number.parse::<i32>() {
        Ok(n) => adjacent_numbers.push(n),
        Err(_) => adjacent_numbers.push(0),
    }

    i = 0;

    //check top-right
    while c_r as i32 - 1 >= 0
        && c_c as i32 + 1 - i >= 0
        && c_c as i32 + 1 - i <= m_c as i32
        && lines[c_r - 1]
            .chars()
            .nth((c_c as i32 + 1 - i) as usize)
            .unwrap()
            .is_numeric()
    {
        i += 1;
    }

    i -= 1;

    if i < 0 {
        i = 0;
    } else if i == 1 {
        should_check_tc = false;
    } else if i > 1 {
        should_check_tc = false;
        should_check_tl = false;
    }

    //starting to record the number of top-right
    let mut top_right_number = String::new();
    while c_r as i32 - 1 >= 0
        && c_c as i32 + 1 - i >= 0
        && c_c as i32 + 1 - i < m_c as i32
        && lines[c_r - 1]
            .chars()
            .nth((c_c as i32 + 1 - i) as usize)
            .unwrap()
            .is_numeric()
    {
        top_right_number.push(lines[c_r - 1].chars().nth((c_c as i32 + 1 - i) as usize).unwrap());
        i -= 1;
    }
    match top_right_number.parse::<i32>() {
        Ok(n) => adjacent_numbers.push(n),
        Err(_) => adjacent_numbers.push(0),
    }

    //check top-center
    i = 0;
    if should_check_tc {
        //find init of top center number
        while c_r as i32 - 1 >= 0
            && c_c as i32 - i >= 0
            && lines[c_r - 1]
                .chars()
                .nth((c_c as i32 - i) as usize)
                .unwrap()
                .is_numeric()
        {
            i += 1;
        }
        i -= 1;
        if i < 0 {
            i = 0;
        } else if i >= 1 {
            should_check_tl = false;
        }

        //starting to record the number of top-center
        let mut top_center_number = String::new();
        while c_r as i32 - 1 >= 0
            && c_c as i32 - i >= 0
            && c_c as i32 - i < m_c as i32
            && lines[c_r - 1]
                .chars()
                .nth((c_c as i32 - i) as usize)
                .unwrap()
                .is_numeric()
        {
            top_center_number.push(
                lines[c_r - 1]
                    .chars()
                    .nth((c_c as i32 - i) as usize)
                    .unwrap(),
            );
            i -= 1;
        }
        match top_center_number.parse::<i32>() {
            Ok(n) => adjacent_numbers.push(n),
            Err(_) => adjacent_numbers.push(0),
        }
    }

    i = 0;

    if should_check_tl {
        //find init of top left number
        while c_r as i32 - 1 >= 0
            && c_c as i32 - 1 - i >= 0
            && lines[c_r - 1]
                .chars()
                .nth((c_c as i32 - 1 - i) as usize)
                .unwrap()
                .is_numeric()
        {
            i += 1;
        }
        i -= 1;
        if i < 0 {
            i = 0;
        }

        //starting to record the number of top-left
        let mut top_left_number = String::new();
        while c_r as i32 - 1 >= 0
            && c_c as i32 - 1 - i >= 0
            && lines[c_r - 1]
                .chars()
                .nth((c_c as i32 - 1 - i) as usize)
                .unwrap()
                .is_numeric()
        {
            top_left_number.push(lines[c_r - 1].chars().nth((c_c as i32 - 1 - i) as usize).unwrap());
            i -= 1;
        }
        match top_left_number.parse::<i32>() {
            Ok(n) => adjacent_numbers.push(n),
            Err(_) => adjacent_numbers.push(0),
        }
    }

    i = 0;

    //check middle-left

    //find init of middle-left
    while c_c as i32 - 1 - i >= 0
        && lines[c_r]
            .chars()
            .nth((c_c as i32 - 1 - i) as usize)
            .unwrap()
            .is_numeric()
    {
        i += 1;
    }
    i -= 1;
    if i < 0 {
        i = 0;
    }
    let mut middle_left_number = String::new();

    while c_c as i32 - 1 - i < m_c as i32
        && lines[c_r]
            .chars()
            .nth((c_c as i32 - 1 - i) as usize)
            .unwrap()
            .is_numeric()
    {
        middle_left_number.push(
            lines[c_r]
                .chars()
                .nth((c_c as i32 - 1 - i) as usize)
                .unwrap(),
        );
        i -= 1;
    }
    match middle_left_number.parse::<i32>() {
        Ok(n) => adjacent_numbers.push(n),
        Err(_) => adjacent_numbers.push(0),
    }

    i = 0;

    //check bottom-center
    if should_check_bc {
        //find init of bottom-center
        while c_r as i32 + 1 < m_r as i32
            && c_c as i32 - i >= 0
            && lines[c_r + 1]
                .chars()
                .nth((c_c as i32 - i) as usize)
                .unwrap()
                .is_numeric()
        {
            i += 1;
        }

        i -= 1;
        if i < 0 {
            i = 0;
        } else if i >= 1 {
            should_check_bl = false
        }

        let mut bottom_center_number = String::new();
        while c_r as i32 + 1 < m_r as i32
            && c_c as i32 - i >= 0
            && lines[c_r + 1]
                .chars()
                .nth((c_c as i32 - i) as usize)
                .unwrap()
                .is_numeric()
        {
            bottom_center_number.push(
                lines[c_r + 1]
                    .chars()
                    .nth((c_c as i32 - i) as usize)
                    .unwrap(),
            );
            i -= 1;
        }
        match bottom_center_number.parse::<i32>() {
            Ok(n) => adjacent_numbers.push(n),
            Err(_) => adjacent_numbers.push(0),
        }
    }

    i = 0;

    if should_check_bl {
        //find init bottom left number
        while c_r as i32 + 1 < m_r as i32
            && c_c as i32 - 1 - i >= 0
            && lines[c_r + 1]
                .chars()
                .nth((c_c as i32 - 1 - i) as usize)
                .unwrap()
                .is_numeric()
        {
            i += 1;
        }
        i -= 1;
        if i < 0 {
            i = 0;
        }

        //starting to record the number of bottom left
        let mut bottom_left_number = String::new();
        while c_r as i32 + 1 < m_r as i32
            && c_c as i32 - 1 - i >= 0
            && lines[c_r + 1]
                .chars()
                .nth((c_c as i32 - 1 - i) as usize)
                .unwrap()
                .is_numeric()
        {
            bottom_left_number.push(
                lines[c_r + 1]
                    .chars()
                    .nth((c_c as i32 - 1 - i) as usize)
                    .unwrap(),
            );
            i -= 1;
        }
        match bottom_left_number.parse::<i32>() {
            Ok(n) => adjacent_numbers.push(n),
            Err(_) => adjacent_numbers.push(0),
        }
    }

    return adjacent_numbers;
}
