use advent_of_code_data::{Day, Year, get_input};
// use std::fs;
use std::ops::RangeInclusive;
// use std::path;

fn extract_data(
    input: &str,
) -> Result<(Vec<RangeInclusive<usize>>, Vec<usize>), Box<dyn std::error::Error>> {
    let mut ranges: Vec<RangeInclusive<usize>> = Vec::new();
    let mut indices: Vec<usize> = Vec::new();

    // extract ranges
    let mut lines_iter = input.lines();
    while let Some(line) = lines_iter.next() {
        if line.trim().is_empty() {
            break;
        }
        let start_stop: Vec<usize> = line
            .split("-")
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;
        if start_stop.len() != 2 {
            return Err("Invalid range format".into());
        }
        ranges.push(start_stop[0]..=start_stop[1]);
    }
    // extract indices
    for line in lines_iter {
        indices.push(line.parse::<usize>().expect("parsing error"))
    }
    Ok((ranges, indices))
}

fn collect_valid_ids(ranges: &[RangeInclusive<usize>], indices: &[usize]) -> u32 {
    let mut fresh_ids = 0u32;
    for index in indices.iter() {
        for range in ranges.iter() {
            if range.contains(&index) {
                fresh_ids += 1;
                break;
            }
        }
    }
    fresh_ids
}

fn merge_ranges(ranges: &[RangeInclusive<usize>]) -> Vec<RangeInclusive<usize>> {
    let mut sorted_range = ranges.to_vec();
    let mut union_ranges = Vec::<RangeInclusive<usize>>::new();
    sorted_range.sort_by_key(|r| *r.start());
    let mut last_range = sorted_range[0].clone();

    for current_range in sorted_range.iter().skip(1) {
        if *current_range.start() <= *last_range.end() + 1 {
            // overlapping or adjacent ranges
            let start = *last_range.start();
            let end = (*last_range.end()).max(*current_range.end());
            last_range = start..=end;
        } else {
            union_ranges.push(last_range.clone());
            last_range = current_range.clone();
        }
    }
    union_ranges.push(last_range);
    union_ranges
}

fn count_ids_in_ranges(ranges: &[RangeInclusive<usize>]) -> u64 {
    let mut id_count = 0u64;
    for range in ranges {
        id_count += (*range.end() - *range.start() + 1) as u64;
    }
    id_count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let input = fs::read_to_string(path::Path::new("example_data.txt"))?;
    let input = get_input(Day(5), Year(2025))?;

    let (ranges, indices) = extract_data(&input)?;
    let ranges = merge_ranges(&ranges);
    let fresh_ids = collect_valid_ids(&ranges, &indices);
    println!("Fresh IDs: {}", fresh_ids);

    // for part 2 we have to return all elements in the union over the ranges. We can do this in two ways.
    // Computing the union is O(nlogn) over the number of ranges in time and O(n) in space where n is the number of ranges
    // Extracting all elements of all ranges and creating a HashSet is O(l) in time and space where l is the total
    // number of elements in the ranges. It seems like option 2 requires too much memory.
    // let mut all_fresh_ids = HashSet::<usize>::new();
    // for range in ranges{
    //     all_fresh_ids.extend(range);
    // }
    let all_fresh_ids = count_ids_in_ranges(&ranges);
    println!("Number of fresh ids: {}", all_fresh_ids);

    Ok(())
}
