use advent_of_code_data::{Day, Year, get_input};
// use std::fs;
// use std::path;

#[derive(Copy, Clone)]
struct Coord {
    row: usize,
    col: usize,
}

fn build_grid(input: &str, print: bool) -> Result<Vec<Vec<bool>>, Box<dyn std::error::Error>> {
    let grid: Vec<Vec<bool>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '@' => Ok(true),
                    '.' => Ok(false),
                    _ => Err(format!("unexpected character: {c}")),
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    if print {
        grid.iter().for_each(|row| {
            println!(
                "{}",
                row.iter()
                    .map(|&b| if b { '@' } else { '.' })
                    .collect::<String>()
            )
        })
    };
    return Ok(grid);
}

fn is_valid_point(grid: &[Vec<bool>], row_idx: usize, col_idx: usize) -> bool {
    let height = grid.len();
    let width = grid[0].len();
    if !grid[row_idx][col_idx] {
        return false;
    } else {
        let mut neighbors = 0u8;

        // oiginal implementation using simple logic and type conversion to handle signed values
        // let row_idx = row_idx as i32;
        // let col_idx = col_idx as i32;
        // for i in [row_idx - 1, row_idx, row_idx + 1] {
        //     for j in [col_idx - 1, col_idx, col_idx + 1] {
        //         if i < 0 || j < 0 || i as usize >= height || j as usize >= width {
        //             continue;
        //         }
        //         neighbors += grid[i as usize][j as usize] as u8;
        //     }
        // }

        // improved approach using saturating_sub and ranges
        for i in row_idx.saturating_sub(1)..(row_idx + 2).min(height) {
            for j in col_idx.saturating_sub(1)..(col_idx + 2).min(width) {
                neighbors += grid[i][j] as u8;
            }
        }
        return neighbors < 5;
    }
}

fn traverse_grid(grid: &[Vec<bool>]) -> Vec<Coord> {
    // we know this is a square grid, so rosws[0].len() == rows[i].len()
    let mut coords: Vec<Coord> = Vec::new();
    let height = grid.len();
    let width = grid[0].len();

    for i in 0..height {
        for j in 0..width {
            if is_valid_point(&grid, i, j) {
                coords.push(Coord { row: i, col: j })
            }
        }
    }
    return coords;
}

fn update_grid(
    grid: &mut Vec<Vec<bool>>,
    valid_coords: &Vec<Coord>,
) -> Result<(), Box<dyn std::error::Error>> {
    for coord in valid_coords {
        grid[coord.row][coord.col] = false;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let input = fs::read_to_string(path::Path::new("example_data.txt"))?;
    let input = get_input(Day(4), Year(2025))?;
    let mut grid = build_grid(&input, false)?;
    let mut valid_coords = traverse_grid(&grid);
    println!("Number of valid points:");
    println!("Part 1:  {}", valid_coords.len());

    let mut valid_points = valid_coords.len();

    update_grid(&mut grid, &valid_coords)?;
    while !valid_coords.is_empty() {
        valid_coords = traverse_grid(&grid);
        update_grid(&mut grid, &valid_coords)?;
        valid_points += valid_coords.len();
    }
    println!("Part 2:  {}", valid_points);

    Ok(())
}
