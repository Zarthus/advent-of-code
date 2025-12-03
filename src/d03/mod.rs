use std::io::Write;

pub fn part1(
    writer: &mut std::io::BufWriter<std::io::StdoutLock>,
    input: &[String],
    _bench: &crate::bench::Bench,
) {
    let mut best = 0u64;

    for line in input {
        let number = find_best_two_joltages(&line);
        best += number;
    }

    write!(writer, "{}", best).unwrap();
}

fn find_best_two_joltages(joltages: &String) -> u64 {
    let numbers = joltages
        .chars()
        .map(|s| s.to_string().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let len = numbers.len();
    let mut highest_num1 = 0;
    let mut highest_num1_pos = 0;
    let mut highest_num2 = 0;

    for i in 0..len - 1 {
        if numbers[i] > highest_num1 {
            highest_num1 = numbers[i];
            highest_num1_pos = i;
        }
    }
    for i in (highest_num1_pos + 1)..len {
        if numbers[i] > highest_num2 {
            highest_num2 = numbers[i];
        }
    }

    format!("{}{}", highest_num1, highest_num2)
        .parse::<u64>()
        .unwrap()
}

pub fn part2(
    writer: &mut std::io::BufWriter<std::io::StdoutLock>,
    input: &[String],
    _bench: &crate::bench::Bench,
) {
    let mut best = 0u64;

    for line in input {
        let number = find_best_joltages(&line, 12);
        best += number;
    }

    write!(writer, "{}", best).unwrap();
}

fn find_best_joltages(joltages: &String, amount: usize) -> u64 {
    let numbers = joltages
        .chars()
        .map(|s| s.to_string().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let len = numbers.len();
    if amount > len {
        return numbers
            .iter()
            .map(|n| n.to_string())
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
    }

    let mut result = String::new();
    let mut start = 0;

    for i in 0..amount {
        let remaining_needed = amount - i;
        let search_end = len - remaining_needed + 1;

        let mut best_digit = 0;
        let mut best_idx = start;

        for j in start..search_end {
            if numbers[j] > best_digit {
                best_digit = numbers[j];
                best_idx = j;
            }
        }

        result.push_str(&best_digit.to_string());
        start = best_idx + 1;
    }

    result.parse::<u64>().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_output;
    use std::io::Read;

    const JOLTAGES: [&str; 4] = [
        "987654321111111",
        "811111111111119",
        "234234234234278",
        "818181911112111",
    ];

    #[test]
    fn test_part1() {
        assert_output!(
            part1,
            JOLTAGES
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            "357"
        );
    }

    #[test]
    fn test_part2() {
        assert_output!(
            part2,
            JOLTAGES
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            "3121910778619"
        );
    }
}
