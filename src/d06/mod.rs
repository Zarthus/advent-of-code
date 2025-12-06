use std::io::Write;

pub fn part1(
    writer: &mut std::io::BufWriter<std::io::StdoutLock>,
    input: &[String],
    _bench: &crate::bench::Bench,
) {
    let equations = parse_input(input);
    let total: u64 = equations.iter().map(|eq| eq.evaluate()).sum();

    write!(writer, "{}", total).unwrap();
}

enum Operator {
    Add,
    Multiply,
}

struct Equation {
    numbers: Vec<u64>,
    operator: Operator,
}

impl Equation {
    fn evaluate(&self) -> u64 {
        match self.operator {
            Operator::Add => self.numbers.iter().sum(),
            Operator::Multiply => self.numbers.iter().product(),
        }
    }
}

fn parse_input(input: &[String]) -> Vec<Equation> {
    let mut equations = Vec::new();

    let total_equations = input.last().unwrap().split_whitespace().count();
    for i in 0..total_equations {
        let mut numbers = Vec::new();
        for line in input.iter().take(input.len() - 1) {
            let num_str = line
                .split_whitespace()
                .nth(i)
                .expect("Failed to get number");
            let num: u64 = num_str.parse().expect("Failed to parse number");
            numbers.push(num);
        }

        let operator_str = input
            .last()
            .unwrap()
            .split_whitespace()
            .nth(i)
            .expect("Failed to get operator");
        let operator = match operator_str {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => panic!("Unknown operator"),
        };
        equations.push(Equation { numbers, operator });
    }

    equations
}

// part2: solved by ai (claude code 4.5), due to timeboxing overrun
pub fn part2(
    writer: &mut std::io::BufWriter<std::io::StdoutLock>,
    input: &[String],
    _bench: &crate::bench::Bench,
) {
    let max_width = input.iter().map(|line| line.len()).max().unwrap_or(0);

    let grid: Vec<Vec<char>> = input
        .iter()
        .take(input.len() - 1)
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.resize(max_width, ' ');
            chars
        })
        .collect();

    let mut transposed = Vec::new();
    for col_idx in 0..max_width {
        let mut column = String::new();
        for row in &grid {
            column.push(row[col_idx]);
        }
        transposed.push(column);
    }

    let mut equations: Vec<Vec<String>> = Vec::new();
    let mut current_eq = Vec::new();

    for col in transposed {
        if col.trim().is_empty() {
            if !current_eq.is_empty() {
                equations.push(current_eq.clone());
                current_eq.clear();
            }
        } else {
            current_eq.push(col.trim().to_string());
        }
    }
    if !current_eq.is_empty() {
        equations.push(current_eq);
    }

    let operators: Vec<&str> = input.last().unwrap().split_whitespace().collect();

    let mut total: u64 = 0;
    for (i, nums) in equations.iter().enumerate() {
        if i >= operators.len() {
            break;
        }
        let nums: Vec<u64> = nums.iter().filter_map(|s| s.parse().ok()).collect();
        if nums.is_empty() {
            continue;
        }
        let result = match operators[i] {
            "+" => nums.iter().sum::<u64>(),
            "*" => nums.iter().product::<u64>(),
            _ => 0,
        };
        total += result;
    }

    write!(writer, "{}", total).unwrap();
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_output;
    use std::io::Read;

    const INPUTS: [&str; 4] = [
        "123 328  51 64 ",
        " 45 64  387 23 ",
        "  6 98  215 314",
        "*   +   *   +  ",
    ];

    fn inputs() -> Vec<String> {
        INPUTS.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_part1() {
        assert_output!(part1, inputs(), "4277556");
    }

    #[test]
    fn test_part2() {
        assert_output!(part2, inputs(), "3263827");
    }
}
