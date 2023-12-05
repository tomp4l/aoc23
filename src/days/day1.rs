use super::day::*;

pub struct Instance;

fn calibration_value(lines: &[String], scores: &Vec<(String, u32)>) -> u32 {
    lines
        .iter()
        .map(|l| {
            let mut first = None;
            let mut last = None;
            for i in 0..l.len() {
                for (m, s) in scores {
                    if l[i..].starts_with(m) {
                        if first.is_none() {
                            first = Some(*s)
                        }
                        last = Some(*s)
                    }
                }
            }
            first.unwrap_or(0) * 10 + last.unwrap_or(0)
        })
        .sum()
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let mut scores: Vec<_> = (1..=9).map(|i| (i.to_string(), i)).collect();

        let part1: u32 = calibration_value(&lines, &scores);

        let words: Vec<_> = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .iter()
        .enumerate()
        .map(|(i, &w)| (w.to_owned(), (i + 1) as u32))
        .collect();

        scores.extend(words);

        let part2 = calibration_value(&lines, &scores);
        Ok(DayResult {
            part1: part1.to_string(),
            part2: Some(part2.to_string()),
        })
    }
}
