use advent_of_code_data::{Day, Year, get_input};
// use std::fs;
// use std::path;

fn max_num(line: &str, n_digits: u8) -> Result<u64, Box<dyn std::error::Error>> {
    let mut reversed_chars = line.chars().rev();
    let mut digits = Vec::<u8>::with_capacity(n_digits as usize);
    // step one: populate the digits vector
    for _ in 0..n_digits {
        digits.push(
            reversed_chars
                .next()
                .ok_or("Iterator error")?
                .to_digit(10)
                .ok_or("Not a digit")? as u8,
        );
    }

    // reverse digits to be in the right order
    digits.reverse();

    // iterate through remaining characters right to left
    for c in reversed_chars {
        // grab character
        let mut num: u8 = c.to_digit(10).ok_or("Not a digit")? as u8;
        // go through the individual characters in the digits vector left to right
        for i in 0..digits.len() {
            // assuming the current number is larger than the current position in digits
            if num >= digits[i] {
                // we replace the current digit with the new one and hold the current digit in a buffer
                let tmp = digits[i];
                digits[i] = num;
                num = tmp;
            }
            else{
                break
            }
        }
    }
    let mut result: u64 = 0;
    // construct final number
    for (exponent, digit) in digits.iter().rev().enumerate() {
        result += *digit as u64 * 10u64.pow(exponent as u32);
    }
    Ok(result)
}

fn run(input: &String, digits: u8)->Result<(), Box<dyn std::error::Error>> {
    let mut max_nums = Vec::<u64>::new();
    for line in input.lines() {
        let curr_num = max_num(line, digits)?;
        max_nums.push(curr_num as u64);
    }
    println!("Result: {}", max_nums.into_iter().sum::<u64>());
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let input = fs::read_to_string(path::Path::new("example_data.txt"))?;
    let input = get_input(Day(3), Year(2025))?;
    println!("Part one:");
    run(&input, 2)?;
    println!("Part two:");
    run(&input, 12)?;
    Ok(())
}

#[test]
fn test_max_num() {
    assert!(max_num("987654321111111", 2).unwrap() == 98);
    assert!(max_num("811111111111119", 2).unwrap() == 89);
    assert!(max_num("234234234234278", 2).unwrap() == 78);
    assert!(max_num("818181911112111", 2).unwrap() == 92);
    assert!(max_num("987654321111111", 12).unwrap() == 987654321111);
    assert!(max_num("811111111111119", 12).unwrap() == 811111111119);
    assert!(max_num("234234234234278", 12).unwrap() == 434234234278);
}
