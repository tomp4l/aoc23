pub trait Day {
    fn run(&self, lines: Vec<String>) -> Result<(), String>;
}
