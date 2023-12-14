use std::collections::HashMap;

use itertools::Itertools;

use super::day::{Day, DayResult};

pub struct Instance;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
enum Rock {
    Round,
    Cube,
}

impl Rock {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'O' => Some(Self::Round),
            '#' => Some(Self::Cube),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Platform {
    rocks: HashMap<(usize, usize), Rock>,
    x_len: usize,
    y_len: usize,
}

impl Platform {
    fn from_lines(lines: &[String]) -> Self {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut rocks = HashMap::new();
        for (y, l) in lines.iter().enumerate() {
            max_y = max_y.max(y);
            for (x, c) in l.chars().enumerate() {
                max_x = max_x.max(x);
                if let Some(rock) = Rock::from_char(c) {
                    rocks.insert((x, y), rock);
                }
            }
        }
        Platform {
            rocks,
            x_len: max_x + 1,
            y_len: max_y + 1,
        }
    }

    fn tilt_north(&mut self) {
        for y in 0..self.y_len {
            for x in 0..self.x_len {
                if let Some(Rock::Round) = self.rocks.get(&(x, y)) {
                    self.rocks.remove(&(x, y));
                    let mut y = y;
                    loop {
                        if y > 0 {
                            y -= 1;
                            if self.rocks.get(&(x, y)).is_some() {
                                y += 1;
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    self.rocks.insert((x, y), Rock::Round);
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for y in (0..self.y_len).rev() {
            for x in 0..self.x_len {
                if let Some(Rock::Round) = self.rocks.get(&(x, y)) {
                    self.rocks.remove(&(x, y));
                    let mut y = y;
                    loop {
                        if y < (self.y_len - 1) {
                            y += 1;
                            if self.rocks.get(&(x, y)).is_some() {
                                y -= 1;
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    self.rocks.insert((x, y), Rock::Round);
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for x in (0..self.x_len).rev() {
            for y in 0..self.y_len {
                if let Some(Rock::Round) = self.rocks.get(&(x, y)) {
                    self.rocks.remove(&(x, y));
                    let mut x = x;
                    loop {
                        if x < (self.x_len - 1) {
                            x += 1;
                            if self.rocks.get(&(x, y)).is_some() {
                                x -= 1;
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    self.rocks.insert((x, y), Rock::Round);
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for x in 0..self.x_len {
            for y in 0..self.y_len {
                if let Some(Rock::Round) = self.rocks.get(&(x, y)) {
                    self.rocks.remove(&(x, y));
                    let mut x = x;
                    loop {
                        if x > 0 {
                            x -= 1;
                            if self.rocks.get(&(x, y)).is_some() {
                                x += 1;
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    self.rocks.insert((x, y), Rock::Round);
                }
            }
        }
    }

    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn total_load(&self) -> usize {
        self.rocks
            .iter()
            .filter(|(_, v)| matches!(v, Rock::Round))
            .map(|((_, y), _)| self.y_len - y)
            .sum()
    }
}

const TARGET_CYCLE: usize = 1000000000;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let mut platform = Platform::from_lines(&lines);

        platform.tilt_north();

        let part1 = platform.total_load().to_string();

        let mut cycles = Vec::new();
        let mut cache = HashMap::new();
        let mut cycle_length = 0;

        for i in 0.. {
            platform.cycle();
            cycles.push(platform.total_load());
            let rocks = platform
                .rocks
                .iter()
                .map(|(a, b)| (*a, b.clone()))
                .sorted()
                .collect_vec();

            if let Some(j) = cache.get(&rocks) {
                cycle_length = (i - j) as usize;
                break;
            } else {
                cache.insert(rocks, i);
            }
        }

        let cycle = &cycles[(cycles.len() - cycle_length)..cycles.len()];
        let offset = TARGET_CYCLE % cycle_length;
        let start_offset = cycles.len() % cycle_length + 1;

        let target_offset = (cycle_length - start_offset + offset) % cycle_length;

        Ok(DayResult {
            part1,
            part2: Some(cycle[target_offset].to_string()),
        })
    }
}
