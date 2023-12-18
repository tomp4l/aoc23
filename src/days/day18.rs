use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

use super::day::{Day, DayResult};

pub struct Instance;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            u => Err(format!("Bad direction: {}", u))?,
        })
    }
}

impl Direction {
    fn next(&self, c: &(i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (c.0, c.1 - 1),
            Direction::Down => (c.0, c.1 + 1),
            Direction::Right => (c.0 + 1, c.1),
            Direction::Left => (c.0 - 1, c.1),
        }
    }
}

struct Instruction {
    direction: Direction,
    distance: u32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let direction = split.next().unwrap().parse()?;
        let distance = split
            .next()
            .ok_or("missing distance")?
            .parse()
            .map_err(|e| format!("bad distance {}", e))?;

        Ok(Instruction {
            direction,
            distance,
        })
    }
}

struct Grid {
    pos: (i32, i32),
    dug: HashSet<(i32, i32)>,
}

impl Grid {
    fn new() -> Self {
        let mut dug = HashSet::new();
        dug.insert((0, 0));
        Grid { pos: (0, 0), dug }
    }

    fn apply(&mut self, instruction: &Instruction) {
        let mut distance = instruction.distance;
        let mut pos = self.pos;
        while distance > 0 {
            pos = instruction.direction.next(&pos);
            self.dug.insert(pos);
            distance -= 1;
        }
        self.pos = pos;
    }

    fn fill(&mut self) {
        let x_min = self.dug.iter().map(|c: &(i32, i32)| c.0).min().unwrap();
        let x_max = self.dug.iter().map(|c| c.0).max().unwrap();
        let y_min = self.dug.iter().map(|c| c.1).min().unwrap();
        let y_max = self.dug.iter().map(|c| c.1).max().unwrap();

        enum Entry {
            CornerUp,
            CornerDown,
        }

        for y in y_min..=y_max {
            let mut inside = false;
            let mut entry = None;

            for x in x_min..=x_max {
                if self.dug.contains(&(x, y)) {
                    if let Some(ref e) = entry {
                        if !self.dug.contains(&(x + 1, y)) {
                            match e {
                                Entry::CornerUp => {
                                    if self.dug.contains(&(x, y + 1)) {
                                        inside = !inside
                                    }
                                }
                                Entry::CornerDown => {
                                    if self.dug.contains(&(x, y - 1)) {
                                        inside = !inside
                                    }
                                }
                            }
                            entry = None;
                        }
                    } else if !self.dug.contains(&(x, y + 1)) {
                        entry = Some(Entry::CornerUp)
                    } else if !self.dug.contains(&(x, y - 1)) {
                        entry = Some(Entry::CornerDown)
                    } else {
                        inside = !inside
                    }
                } else if inside {
                    self.dug.insert((x, y));
                }
            }
        }
    }

    fn volume(&self) -> usize {
        self.dug.len()
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let instructions: Vec<Instruction> = lines.iter().map(|l| l.parse()).try_collect()?;
        let mut grid = Grid::new();

        for instruction in instructions.iter() {
            grid.apply(instruction);
        }

        grid.fill();

        let part1 = grid.volume().to_string();
        Ok(DayResult { part1, part2: None })
    }
}
