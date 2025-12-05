use std::{
    io::Write,
    ops::{Range, RangeInclusive},
};

pub fn part1(
    writer: &mut std::io::BufWriter<std::io::StdoutLock>,
    input: &[String],
    _bench: &crate::bench::Bench,
) {
    let (ranges, ingredient_ids) = parse_input(input);

    let mut fresh = 0;
    for id in ingredient_ids {
        if ranges.contains(id) {
            fresh += 1;
        }
    }

    write!(writer, "{}", fresh).unwrap();
}

struct IngredientRanges {
    ranges: Vec<RangeInclusive<u64>>,
}

impl IngredientRanges {
    fn contains(&self, value: u64) -> bool {
        for range in &self.ranges {
            if range.contains(&value) {
                return true;
            }
        }
        false
    }
}

fn parse_input(input: &[String]) -> (IngredientRanges, Vec<u64>) {
    let ranges = parse_ingredient_ranges(input);
    let mut ingredient_ids = Vec::new();

    for line in input.iter().skip(ranges.len() + 1) {
        let num: u64 = line.parse().unwrap();
        ingredient_ids.push(num);
    }

    (IngredientRanges { ranges }, ingredient_ids)
}

pub fn part2(
    writer: &mut std::io::BufWriter<std::io::StdoutLock>,
    input: &[String],
    _bench: &crate::bench::Bench,
) {
    let mut ranges = parse_ingredient_ranges(input);
    let mut total = 0;

    ranges.sort_by(|a, b| a.start().cmp(b.start()));
    let mut current_range: RangeInclusive<u64> = ranges.first().cloned().unwrap();
    for range in ranges {
        if range.start() <= &(current_range.end() + 1) {
            let new_end = std::cmp::max(*current_range.end(), *range.end());
            current_range = RangeInclusive::new(*current_range.start(), new_end);
        } else {
            total += current_range.end() - current_range.start() + 1;
            current_range = range;
        }
    }

    total += current_range.end() - current_range.start() + 1;
    write!(writer, "{}", total).unwrap();
}

fn parse_ingredient_ranges(input: &[String]) -> Vec<RangeInclusive<u64>> {
    let mut ranges = Vec::new();

    for line in input {
        if line.is_empty() {
            break;
        }
        let parts: Vec<&str> = line.trim().split('-').collect();
        let start: u64 = parts[0].parse().unwrap();
        let end: u64 = parts[1].parse().unwrap();
        ranges.push(RangeInclusive::new(start, end));
    }

    ranges
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_output;
    use std::io::Read;

    const INGREDIENT_RANGES: [&str; 11] = [
        "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
    ];

    fn inputs() -> Vec<String> {
        INGREDIENT_RANGES.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_part1() {
        assert_output!(part1, inputs(), "3");
    }

    #[test]
    fn test_part2() {
        assert_output!(part2, inputs(), "14");
    }
}
