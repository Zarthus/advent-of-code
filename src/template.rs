use std::io::Write;

pub fn part1(
    writer: &mut std::io::BufWriter<std::io::StdoutLock>,
    input: &[String],
    _bench: &crate::bench::Bench,
) {
}

pub fn part2(
    writer: &mut std::io::BufWriter<std::io::StdoutLock>,
    input: &[String],
    _bench: &crate::bench::Bench,
) {
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_output;
    use std::io::Read;

    const INPUTS: [&str; 1] = ["NOT_IMPLEMENTED"];

    fn inputs() -> Vec<String> {
        INPUTS.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_part1() {
        assert_output!(part1, inputs(), "NOT_IMPLEMENTED");
    }

    #[test]
    fn test_part2() {
        assert_output!(part2, inputs(), "NOT_IMPLEMENTED");
    }
}
