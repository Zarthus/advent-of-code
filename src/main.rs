mod d01;

fn main() {
    let day: u16 = std::env::args()
        .nth(1)
        .expect("Usage: ./program <day> <part>")
        .parse()
        .expect("day: not a digit");

    println!("Day {}", day);
    match day {
        1 => d01::solve(),
        _ => println!("Usage: ./program <day> <part>"),
    }
}
