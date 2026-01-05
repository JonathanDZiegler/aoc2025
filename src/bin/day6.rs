use advent_of_code_data::{Day, Year, get_input};
// use std::fs;
// use std::path;
use std::iter::zip;

fn parse_input(raw_input: &str) -> (Vec<i32>, Vec<char>, usize) {
    let lines: Vec<&str> = raw_input.lines().collect();
    let n_lines: usize = lines.len().saturating_sub(1);
    let mut number_cols: Vec<i32> = Vec::new();
    for line in &lines[..lines.len().saturating_sub(1)] {
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|tok| tok.parse::<i32>().expect("Parsing error"))
            .collect();
        number_cols.extend(row);
    }
    let operators: Vec<char> = lines[lines.len().saturating_sub(1)]
        .split_whitespace()
        .map(|s| s.chars())
        .flatten()
        .collect();
    (number_cols, operators, n_lines)
}

fn part_1(input: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let (number_cols, operators, n_lines) = parse_input(&input);

    let n_cols: usize = number_cols.len() / n_lines;
    let mut result = 0i64;
    for c in 0..n_cols {
        let indices: Vec<usize> = (c..number_cols.len()).step_by(n_cols).collect();
        let collected_nums: Vec<i64> = indices.iter().map(|i| number_cols[*i] as i64).collect();
        match operators[c] {
            '+' => {
                let col_sum: i64 = collected_nums.iter().sum();
                result += col_sum as i64;
            }
            '*' => {
                let col_prod: i64 = collected_nums.iter().product();
                result += col_prod as i64;
            }
            _ => return Err("Unexpected operator".into()),
        }
    }
    Ok(result)
}

fn part_2(input: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let mut sep_cols: Vec<i32> = Vec::new();
    let line_len = input.lines().next().expect("no data?").len();
    let n_lines: usize = input.lines().count();
    for pos in 0..line_len {
        let is_sep_col: bool = input
            .lines()
            .map(|s| s.chars().nth(pos) == Some(' '))
            .all(|b| b);
        if is_sep_col {
            sep_cols.push(pos as i32);
        }
    }
    let numbers: Vec<Vec<char>> = input
        .lines()
        .take(n_lines - 1)
        .map(|l| l.chars().collect())
        .collect();

    let mut slice_pos: Vec<i32> = Vec::with_capacity(sep_cols.len() + 2);
    slice_pos.push(-1);
    slice_pos.extend_from_slice(&sep_cols);
    slice_pos.push(line_len as i32);

    let n_lines: usize = input.lines().count() - 1;
    let mut processed_nums: Vec<Vec<i64>> = Vec::with_capacity(n_lines);

    // iterate over columns to build the collections of numbers to be joined with the operators
    for i in (0..(sep_cols.len() + 1)).rev() {
        // grab all elements of the current column right to left and build the int representation
        let mut curr_col: Vec<i64> = Vec::with_capacity(n_lines);
        for char_idx in (slice_pos[i] + 1..slice_pos[i + 1]).rev() {
            let number_string: String = numbers.iter().map(|l| l[char_idx as usize]).collect();
            let number_int: i64 = number_string.trim().parse().expect("parsing error");
            curr_col.push(number_int);
        }
        processed_nums.push(curr_col);
    }
    let operators: Vec<char> = input
        .lines()
        .last()
        .expect("no data?")
        .chars()
        .filter(|&c| c == '+' || c == '*')
        .rev()
        .collect();

    let mut final_res = 0i64;
    for (nums, op) in zip(processed_nums.into_iter(), operators.into_iter()) {
        match op {
            '+' => {
                let col_sum: i64 = nums.iter().sum();
                final_res += col_sum as i64;
            }
            '*' => {
                let col_prod: i64 = nums.iter().product();
                final_res += col_prod as i64;
            }
            _ => return Err("Unexpected operator".into()),
        }
    }
    Ok(final_res)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let input = fs::read_to_string(path::Path::new("example_data.txt"))?;
    let input = get_input(Day(6), Year(2025))?;
    let res_part_1 = part_1(&input)?;
    println!("Result is {}", res_part_1);

    let res_part_2 = part_2(&input)?;
    println!("Result is {}", res_part_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_number_from_column() {
        let col = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let result = part_2(col);
        assert_eq!(result.unwrap(), 3263827i64);
    }
}
