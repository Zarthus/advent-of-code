use std::io::Write;

enum Rotation {
    Left(i64),
    Right(i64),
}
impl std::fmt::Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rotation::Left(num) => write!(f, "L{}", num),
            Rotation::Right(num) => write!(f, "R{}", num),
        }
    }
}
const START: u64 = 50;

pub fn part1(writer: &mut std::io::BufWriter<std::io::StdoutLock>, input: &[String]) {
    let rotations = parse_input(input);

    let mut position: i64 = START as i64;
    let mut counter: u32 = 0;
    for rotation in rotations {
        match rotation {
            Rotation::Left(num) => {
                position -= num;
                while position < 0 {
                    position += 100;
                }
            }
            Rotation::Right(num) => {
                position += num;
                while position >= 100 {
                    position -= 100;
                }
            }
        }

        if position == 0 {
            #[cfg(feature = "debug")]
            println!("CLICK at {}, {}, {}", rotation, counter, position);
            counter += 1;
        }
    }

    write!(writer, "{}", counter).unwrap();
}

pub fn part2(writer: &mut std::io::BufWriter<std::io::StdoutLock>, input: &[String]) {
    let rotations = parse_input(input);

    let mut position: i64 = START as i64;
    let mut counter: u32 = 0;
    for rotation in rotations {
        match rotation {
            Rotation::Left(num) => {
                for _i in 0..num {
                    position -= 1;
                    if position == 0 {
                        counter += 1;
                    }
                    if position < 0 {
                        position = 99;
                    }
                }
            }
            Rotation::Right(num) => {
                for _i in 0..num {
                    position += 1;
                    if position > 99 {
                        position = 0;
                        counter += 1;
                    }
                }
            }
        }
    }

    write!(writer, "{}", counter).unwrap();
}

fn parse_input(input: &[String]) -> Vec<Rotation> {
    input
        .iter()
        .map(|s| parse_rotation(s))
        .collect::<Vec<Rotation>>()
}

fn parse_rotation(s: &str) -> Rotation {
    let (letter, num_str) = s.split_at(1);
    let num: i64 = num_str.parse().unwrap();
    match letter {
        "L" => Rotation::Left(num),
        "R" => Rotation::Right(num),
        _ => panic!("Invalid rotation direction: {}", letter),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_output;
    use std::io::Read;

    const ROTATIONS: [&str; 10] = [
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    ];

    #[test]
    fn test_part1() {
        assert_output!(
            part1,
            ROTATIONS.iter().map(|i| i.to_string()).collect(),
            "3"
        );
    }

    #[test]
    fn test_part2() {
        assert_output!(
            part2,
            ROTATIONS.iter().map(|i| i.to_string()).collect(),
            "6"
        );
    }
}
