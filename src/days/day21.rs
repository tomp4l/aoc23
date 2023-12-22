use std::collections::HashSet;

use super::day::{Day, DayResult};

pub struct Instance;

#[derive(Debug)]
struct Garden {
    rocks: HashSet<(i32, i32)>,
    x_len: i32,
    y_len: i32,
    start: (i32, i32),
}

const TARGET: i64 = 26501365;

impl Garden {
    fn from_lines(lines: &[String]) -> Self {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut rocks = HashSet::new();
        let mut start = None;
        for (y, l) in lines.iter().enumerate() {
            max_y = max_y.max(y as i32);
            for (x, c) in l.chars().enumerate() {
                max_x = max_x.max(x as i32);
                if c == '#' {
                    rocks.insert((x as i32, y as i32));
                }
                if c == 'S' {
                    start = Some((x as i32, y as i32))
                }
            }
        }
        Garden {
            rocks,
            x_len: max_x + 1,
            y_len: max_y + 1,
            start: start.unwrap(),
        }
    }

    fn steps_64(&self) -> usize {
        let mut possibilities = HashSet::new();
        possibilities.insert(self.start);
        for _ in 0..64 {
            let mut new_possibilities = HashSet::new();

            for p in possibilities {
                let attempts = vec![
                    (p.0 + 1, p.1),
                    (p.0 - 1, p.1),
                    (p.0, p.1 + 1),
                    (p.0, p.1 - 1),
                ];

                for (x, y) in attempts {
                    if x < 0 || y < 0 || x >= self.x_len || y >= self.y_len {
                        continue;
                    }
                    if !self.rocks.contains(&(x, y)) {
                        new_possibilities.insert((x, y));
                    }
                }
            }
            possibilities = new_possibilities;
        }

        possibilities.len()
    }

    fn steps_26501365(&self) -> i64 {
        let mut possibilities = HashSet::new();
        possibilities.insert((self.start.0, self.start.1));

        let diff = TARGET % self.x_len as i64;

        let mut xs = Vec::new();

        for i in 0..=(diff as i32 + 2 * self.x_len) {
            let mut new_possibilities = HashSet::new();

            if i % self.x_len == diff as i32 {
                xs.push(possibilities.len() as i64);
            }

            for p in possibilities {
                let attempts = vec![
                    (p.0 + 1, p.1),
                    (p.0 - 1, p.1),
                    (p.0, p.1 + 1),
                    (p.0, p.1 - 1),
                ];

                for (x, y) in attempts {
                    let x_rock = (x + self.x_len * 1000) % self.x_len;
                    let y_rock = (y + self.y_len * 1000) % self.y_len;

                    if !self.rocks.contains(&(x_rock, y_rock)) {
                        new_possibilities.insert((x, y));
                    }
                }
            }
            possibilities = new_possibilities;
        }

        let x0 = xs[0];
        let x1 = xs[1];
        let x2 = xs[2];

        let d2_0 = x1 - x0;
        let d2_1 = x2 - x1;
        let d2 = (d2_1 - d2_0) / 2;

        let d1 = x1 - d2 - x0;

        let t = TARGET / self.x_len as i64;

        t * t * d2 + t * d1 + x0
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let garden = Garden::from_lines(&lines);

        let part1 = garden.steps_64().to_string();

        let part2 = garden.steps_26501365().to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}
