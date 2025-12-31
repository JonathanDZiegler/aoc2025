use advent_of_code_data::{Day, Year, get_input};
use std::fs;
use std::path;

use std::vec;

fn build_grid(input: &str, print: bool) -> Result<Vec<Vec<bool>>, Box<dyn std::error::Error>> {
    let grid: Vec<Vec<bool>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '@' => true,
                    '.' => false,
                    _ => panic!("unexpected value: {c}",),
                })
                .collect()
        })
        .collect();
    grid.iter().for_each(|row| {
        println!(
            "{}",
            row.iter()
                .map(|&b| if b { '@' } else { '.' })
                .collect::<String>()
        )
    });
    return Ok(grid);
}

fn is_valid_point(
    grid: &Vec<Vec<bool>>,
    row_idx: usize,
    col_idx: usize,
    height: usize,
    width: usize,
) -> bool {
    if !grid[row_idx][col_idx] {
        return false;
    } else {
        let row_idx = row_idx as i32;
        let col_idx = col_idx as i32;
        let mut neighbors = 0u8;
        for i in [row_idx - 1, row_idx, row_idx + 1] {
            for j in [col_idx - 1, col_idx, col_idx + 1] {
                if i < 0 || j < 0 || i as usize >= height || j as usize >= width {
                    continue;
                }
                neighbors += grid[i as usize][j as usize] as u8;
            }
        }
        return neighbors < 5;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let input = fs::read_to_string(path::Path::new("example_data.txt"))?;
    let input = get_input(Day(4), Year(2025))?;

    // we now this is a square grid, so rosws[0].len() == rows[i].len()
    let grid = build_grid(&input, false)?;
    let height = grid.len();
    let width = grid[0].len();
    let mut valid_points = 0u32;

    for i in 0..height {
        for j in 0..width {
            valid_points += is_valid_point(&grid, i, j, height, width) as u32;
        }
    }
    println!("Number of valid points: {}", valid_points);
    Ok(())
}
