pub fn part1(writer: &mut std::io::BufWriter<std::io::StdoutLock>, input: &[String]) {}

pub fn part2(writer: &mut std::io::BufWriter<std::io::StdoutLock>, input: &[String]) {}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_output;
    use std::io::Read;

    #[test]
    fn test_part1() {
        assert_output!(
            part1,
            vec!["NOT_IMPLEMENTED".to_string()],
            "NOT_IMPLEMENTED"
        );
    }

    #[test]
    fn test_part2() {
        assert_output!(
            part2,
            vec!["NOT_IMPLEMENTED".to_string()],
            "NOT_IMPLEMENTED"
        );
    }
}
