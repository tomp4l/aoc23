use super::day::{Day, DayResult};

pub struct Instance;

#[derive(Debug)]
struct History(Vec<i64>);

impl History {
    fn predict_forward(&self) -> i64 {
        self.predict(true)
    }

    fn predict_backward(&self) -> i64 {
        self.predict(false)
    }

    fn predict(&self, forward: bool) -> i64 {
        let mut deltas = Vec::new();
        let mut current = self.0.clone();
        loop {
            let diffs: Vec<_> = current
                .windows(2)
                .map(|d| match d {
                    [a, b] => b - a,
                    v => panic!("{:?}", v),
                })
                .collect();

            if diffs.iter().all(|&v| v == 0) {
                break;
            }

            if forward {
                deltas.push(*diffs.last().unwrap());
            } else {
                deltas.push(*diffs.first().unwrap());
            }
            current = diffs;
        }

        let delta: i64 = deltas
            .iter()
            .rev()
            .fold(0, |a, b| if forward { b + a } else { b - a });
        if forward {
            self.0.last().unwrap() + delta
        } else {
            self.0.first().unwrap() - delta
        }
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let histories: Vec<_> = lines
            .iter()
            .map(|l| History(l.split(' ').map(|i| i.parse::<i64>().unwrap()).collect()))
            .collect();

        let part1 = histories
            .iter()
            .map(|v| v.predict_forward())
            .sum::<i64>()
            .to_string();

        let part2 = histories
            .iter()
            .map(|v| v.predict_backward())
            .sum::<i64>()
            .to_string();
        Ok(DayResult {
            part1,
            part2: Some(part2.to_string()),
        })
    }
}
