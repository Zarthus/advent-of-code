use std::time::Instant;

pub struct Bench {
    start: Instant,
}

impl Bench {
    pub fn new() -> Self {
        Bench {
            start: Instant::now(),
        }
    }

    pub fn step(&self, name: &str) {
        println!("{}: {} ms", name, self.elapsed());
    }

    pub fn end(&self, name: &str) {
        println!("Total ({}): {} ms", name, self.elapsed());
    }

    fn elapsed(&self) -> u128 {
        self.start.elapsed().as_millis()
    }
}
