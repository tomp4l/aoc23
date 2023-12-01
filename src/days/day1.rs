use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let part1 = lines
            .iter()
            .map(|l| {
                l.chars()
                    .filter(|c| ('1'..='9').contains(c))
                    .collect::<Vec<_>>()
            })
            .map(|n| {
                vec![n.first().unwrap_or(&'0'), n.last().unwrap_or(&'0')]
                    .into_iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap()
            })
            .sum::<u32>();

        let replacements = vec![
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9"),
        ];

        let part2 = lines
            .iter()
            .map(|l| {
                let mut r = l.to_owned();
                let mut found = true;
                while found {
                    found = false;
                    for (f, t) in &replacements {
                        if let Some(i) = r.find(f) {
                            found = true;
                            r = r[..i + 1].to_owned() + t + &r[i + 1..];
                        }
                    }
                }
                r
            })
            .map(|l| {
                l.chars()
                    .filter(|c| ('1'..='9').contains(c))
                    .collect::<Vec<_>>()
            })
            .map(|n| {
                vec![n.first().unwrap(), n.last().unwrap()]
                    .into_iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap()
            })
            .sum::<u32>();
        Ok(DayResult {
            part1: part1.to_string(),
            part2: Some(part2.to_string()),
        })
    }
}
