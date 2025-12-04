use std::io::Write;

pub fn part1(
    writer: &mut std::io::BufWriter<std::io::StdoutLock>,
    input: &[String],
    _bench: &crate::bench::Bench,
) {
    let grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let height = grid.len();
    let width = if height > 0 { grid[0].len() } else { 0 };

    let mut accessible_count = 0;

    for row in 0..height {
        for col in 0..width {
            if grid[row][col] == '@' {
                let adjacent_count = count_adjacent_rolls(&grid, row, col, height, width);

                if adjacent_count < 4 {
                    accessible_count += 1;
                }
            }
        }
    }

    write!(writer, "{}", accessible_count).unwrap();
}

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn count_adjacent_rolls(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    height: usize,
    width: usize,
) -> usize {
    let mut count = 0;

    for (dr, dc) in DIRECTIONS.iter() {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;

        if new_row >= 0 && new_row < height as i32 && new_col >= 0 && new_col < width as i32 {
            if grid[new_row as usize][new_col as usize] == '@' {
                count += 1;
            }
        }
    }

    count
}

// due to timeboxing, part 2 was generated with AI (claude sonnet 4.5),
// honestly quite nifty. Got it in first try.
pub fn part2(
    writer: &mut std::io::BufWriter<std::io::StdoutLock>,
    input: &[String],
    _bench: &crate::bench::Bench,
) {
    let mut grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let height = grid.len();
    let width = if height > 0 { grid[0].len() } else { 0 };

    let mut total_removed = 0;

    loop {
        // Find all accessible rolls in this iteration
        let mut to_remove = Vec::new();

        for row in 0..height {
            for col in 0..width {
                if grid[row][col] == '@' {
                    let adjacent_count = count_adjacent_rolls(&grid, row, col, height, width);

                    if adjacent_count < 4 {
                        to_remove.push((row, col));
                    }
                }
            }
        }

        // If no rolls can be removed, we're done
        if to_remove.is_empty() {
            break;
        }

        // Remove all accessible rolls
        for (row, col) in to_remove.iter() {
            grid[*row][*col] = '.';
            total_removed += 1;
        }
    }

    write!(writer, "{}", total_removed).unwrap();
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_output;
    use std::io::Read;

    const GRID: [&str; 10] = [
        "..@@.@@@@.",
        "@@@.@.@.@@",
        "@@@@@.@.@@",
        "@.@@@@..@.",
        "@@.@@@@.@@",
        ".@@@@@@@.@",
        ".@.@.@.@@@",
        "@.@@@.@@@@",
        ".@@@@@@@@.",
        "@.@.@@@.@.",
    ];

    #[test]
    fn test_part1() {
        assert_output!(part1, GRID.iter().map(|s| s.to_string()).collect(), "13");
    }

    #[test]
    fn test_part2() {
        assert_output!(part2, GRID.iter().map(|s| s.to_string()).collect(), "43");
    }
}
