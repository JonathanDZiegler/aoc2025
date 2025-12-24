use std::collections::HashMap;
use advent_of_code_data::{get_input, Day, Year};

fn floor_div(a: i32, b: i32) -> i32 {
    let mut result = a / b;
    if (a % b != 0) && ((a < 0) != (b < 0)) {
        result -= 1;
    }
    result
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    // starting position
    let mut dial_pos:i32 = 50;
    let mut zero_pos:u32 = 0;
    let mut zero_traversals:u32 = 0;


    let dir_map = HashMap::from([
        ('L', -1i32),
        ('R', 1i32),
    ]);

    let input = get_input(Day(1), Year(2025)).expect("Failed to get input data");
    for line in input.lines() {
        // split line into direction (1st char) and value
        let direction: char = line.chars().next().ok_or("Empty line")?;
        let value = line[1..].parse::<i32>()?;
        let dir_value = *dir_map.get(&direction).ok_or("Invalid direction")?;
        
        let previous_pos = dial_pos;
        let tmp_pos = value * dir_value + dial_pos;
        
        // Count circuits traversed in this move (not total from origin)
        let circuits_traversed = (floor_div(tmp_pos, 100) - floor_div(previous_pos, 100)).abs() as u32;
        zero_traversals += circuits_traversed;
        
        // Special handling for left moves and zero positions  
        if direction == 'L' {
            if previous_pos == 0 && circuits_traversed > 0 {
                zero_traversals -= 1;
            }
            if tmp_pos.rem_euclid(100) == 0 {
                zero_traversals += 1;
            }
        }
        
        dial_pos = tmp_pos.rem_euclid(100);
        
        if dial_pos == 0 {
            zero_pos += 1;
        }
        // println!("Counter pos: {}", dial_pos);
        // println!("Zero traversal count: {}", zero_traversals);
    }
    println!("Final position: {}", dial_pos);
    println!("Number of times at zero (part 1): {}", zero_pos);
    println!("Number of times crossed zero (part 2): {}", zero_traversals);

    Ok(())
}
