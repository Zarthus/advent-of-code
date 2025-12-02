use std::io::Write;

struct Id {
    start: u64,
    end: u64,
}

pub fn part1(
    writer: &mut std::io::BufWriter<std::io::StdoutLock>,
    input: &[String],
    _bench: &crate::bench::Bench,
) {
    let ids = parse_input(input);

    let mut total: u64 = 0;
    for id in ids {
        for num in id.start..=id.end {
            let s = num.to_string();
            let len = s.len();
            if len % 2 != 0 {
                continue;
            }
            let half = len / 2;
            let (first, second) = s.split_at(half);
            if first == second {
                total += num;
            }
        }
    }

    write!(writer, "{}", total).unwrap();
}

pub fn part2(
    writer: &mut std::io::BufWriter<std::io::StdoutLock>,
    input: &[String],
    _bench: &crate::bench::Bench,
) {
    let ids = parse_input(input);

    let mut total: u64 = 0;
    for id in ids {
        for num in id.start..=id.end {
            let s = num.to_string();
            let len = s.len();
            for pattern_len in 1..=len / 2 {
                if len % pattern_len != 0 {
                    continue;
                }
                let pattern = &s[0..pattern_len];
                if s.chars()
                    .collect::<Vec<_>>()
                    .chunks(pattern_len)
                    .all(|chunk| chunk.iter().collect::<String>() == pattern)
                {
                    total += num;
                    break;
                }
            }
        }
    }
    write!(writer, "{}", total).unwrap();
}

fn parse_input(input: &[String]) -> Vec<Id> {
    input
        .join("")
        .split(",")
        .map(str::trim)
        .map(parse_id)
        .collect()
}

fn parse_id(s: &str) -> Id {
    let parts: Vec<&str> = s.split('-').collect();
    let start = parts[0].parse::<u64>().unwrap();
    let end = parts[1].parse::<u64>().unwrap();
    Id { start, end }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_output;
    use std::io::Read;

    fn ids() -> Vec<String> {
        ["11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn test_part1() {
        assert_output!(part1, ids(), "1227775554");
    }

    #[test]
    fn test_part2() {
        assert_output!(part2, ids(), "4174379265");
    }
}
