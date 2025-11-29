#![allow(dead_code, unused_imports, unused_variables)]

mod bench;
mod d01;

struct Solver {
    day: u8,
    inputs: (Vec<String>, Vec<String>),
    part1: fn(&[String]),
    part2: fn(&[String]),
}

fn main() {
    let (day, part) = parse_inputs();

    println!("Day {}", day);
    let solver = match day {
        1 => (d01::part1, d01::part2),
        _ => {
            eprintln!("Day {} is not yet implemented.", day);
            std::process::exit(1);
        }
    };
    let inputs = load_inputs(day);

    solve(
        Solver {
            day,
            inputs,
            part1: solver.0,
            part2: solver.1,
        },
        part,
    );
}

fn solve(solver: Solver, part: u8) {
    match part {
        1 => {
            let b = bench::Bench::new();
            (solver.part1)(&solver.inputs.0);
            b.end(format!("D{:2}-1", solver.day).as_str());
        }
        2 => {
            let b = bench::Bench::new();
            (solver.part2)(&solver.inputs.1);
            b.end(format!("D{:2}-2", solver.day).as_str());
        }
        0 => {
            let b = bench::Bench::new();
            (solver.part1)(&solver.inputs.0);
            b.end(format!("D{:2}-1", solver.day).as_str());
            let b = bench::Bench::new();
            (solver.part2)(&solver.inputs.1);
            b.end(format!("D{:2}-2", solver.day).as_str());
        }
        _ => {
            eprintln!("Part must be 1, 2, or omitted (for both)");
            std::process::exit(1);
        }
    }
}

fn parse_inputs() -> (u8, u8) {
    let day: u8 = std::env::args()
        .nth(1)
        .unwrap_or_else(|| {
            eprintln!("Usage: ./program <day> [part = both]");
            std::process::exit(1);
        })
        .parse()
        .unwrap_or_else(|e| {
            eprintln!("Usage: ./program <day> [part = both]");
            eprintln!("{:?}", e);
            std::process::exit(1);
        });
    let part: u8 = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "0".to_string())
        .parse()
        .unwrap_or(0);

    (day, part)
}

fn load_inputs(day: u8) -> (Vec<String>, Vec<String>) {
    let p1 = format!("inputs/day{:02}-1.txt", day);
    let p2 = format!("inputs/day{:02}-2.txt", day);

    let input_p1 = std::fs::read_to_string(&p1)
        .unwrap_or_else(|_| {
            eprintln!("Failed to read input file: {}", p1);
            std::process::exit(1);
        })
        .lines()
        .map(|line| line.to_string())
        .collect();

    let input_p2 = match std::fs::read_to_string(&p2) {
        Ok(content) => content.lines().map(|line| line.to_string()).collect(),
        Err(_) => vec!["File 2 does not exist.".to_string()],
    };

    (input_p1, input_p2)
}
