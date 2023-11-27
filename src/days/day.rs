pub struct DayResult {
    pub part1: String,
    pub part2: Option<String>,
}

pub trait Day {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String>;
}
