use advent_of_code_data::{Day, Year, get_input};

fn compute_block_sizes(id_len: usize, exhaustive: bool) -> Vec<i64> {
    // part one only needs the 2-split chunks
    // part 2 needs all chunks
    let mut block_sizes = Vec::<i64>::new();

    match exhaustive {
        true => {
            for i in 1..=((id_len as i64) / 2) {
                if id_len as i64 % i == 0 {
                    block_sizes.push(i);
                }
            }
        }
        false => {
            if id_len % 2 == 0 {
                block_sizes.push((id_len / 2) as i64);
            }
        }
    }
    return block_sizes;
}

fn chunk_string(s: &str, chunk_size: usize) -> Vec<&str> {
    let bytes = s.as_bytes();
    bytes
        .chunks(chunk_size)
        .map(|c| std::str::from_utf8(c).unwrap())
        .collect()
}

fn is_invalid_id(id: &str, exhaustive: bool) -> bool {
    // we compute the possible chunk sizes in which we can evenly split the id
    let block_size = compute_block_sizes(id.len(), exhaustive);
    for b in block_size {
        let chunks = chunk_string(id, b as usize);
        if chunks.iter().all(|item| item == &chunks[0]) {
            return true;
        }
    }
    return false;
}

fn run(exhaustive: bool) -> Result<(), Box<dyn std::error::Error>> {
    let input = get_input(Day(2), Year(2025))?;
    let mut invalid_count: Vec<u64> = Vec::new();

    for range in input.split(",") {
        let parts = range.split("-").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(format!("Invalid line format: {}", range).into());
        }
        let start: u64 = parts[0]
            .parse()
            .expect(&format!("not a number {}", parts[0]));
        let end: u64 = parts[1]
            .trim()
            .parse()
            .expect(&format!("not a number {}", parts[1]));
        for id_num in start..=end {
            let id_str = id_num.to_string();
            if is_invalid_id(&id_str, exhaustive) {
                invalid_count.push(id_num);
            }
        }
    }
    println!("Number of invalid IDs: {}", invalid_count.len());
    println!(
        "Result {}: {}",
        if exhaustive { "part 2" } else { "part 1" },
        invalid_count.iter().sum::<u64>()
    );
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // part 1
    let _part1 = run(false)?;
    let _part2 = run(true)?;
    Ok(())
}

#[test]
fn test_block_sizes() {
    assert_eq!(compute_block_sizes(6, true), vec![1, 2, 3]);
    assert_eq!(compute_block_sizes(6, false), vec![3]);
    assert_eq!(compute_block_sizes(8, true), vec![1, 2, 4]);
    assert_eq!(compute_block_sizes(8, false), vec![4]);
    assert_eq!(compute_block_sizes(5, true), vec![1]);
    assert_eq!(compute_block_sizes(5, false), Vec::<i64>::new());
}

#[test]
fn test_chunk_string() {
    assert_eq!(chunk_string("123456", 2), vec!["12", "34", "56"]);
    assert_eq!(chunk_string("123456", 3), vec!["123", "456"]);
    assert_eq!(
        chunk_string("123456", 1),
        vec!["1", "2", "3", "4", "5", "6"]
    );
}

#[test]
fn test_is_invalid_id() {
    assert!(is_invalid_id("121212", true));
    assert!(is_invalid_id("11", false));
    assert!(is_invalid_id("12341234", false));
    assert!(!is_invalid_id("123456", false));
    assert!(is_invalid_id("111", true));
    assert!(!is_invalid_id("111", false));
}
