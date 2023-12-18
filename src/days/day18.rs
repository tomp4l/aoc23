use std::str::FromStr;

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
    fn line(&self, c: &(i32, i32), distance: u32) -> (i32, i32) {
        let distance: i32 = distance.try_into().unwrap();
        match self {
            Direction::Up => (c.0, c.1 - distance),
            Direction::Down => (c.0, c.1 + distance),
            Direction::Right => (c.0 + distance, c.1),
            Direction::Left => (c.0 - distance, c.1),
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

struct HackedInstruction(Instruction);

impl FromStr for HackedInstruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = &s[s.len() - 7..s.len() - 1];
        let direction = match &hex[5..6] {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            e => Err(format!("bad direction: {}", e))?,
        };

        let distance = u32::from_str_radix(&hex[..5], 16).unwrap();

        Ok(HackedInstruction(Instruction {
            direction,
            distance,
        }))
    }
}

struct SparseGrid {
    pos: (i32, i32),
    hor_lines: Vec<(i32, i32, i32)>,
    ver_lines: Vec<(i32, i32, i32)>,
}

impl SparseGrid {
    fn new() -> Self {
        SparseGrid {
            pos: (0, 0),
            hor_lines: Vec::new(),
            ver_lines: Vec::new(),
        }
    }

    fn apply(&mut self, instruction: &Instruction) {
        let next = instruction.direction.line(&self.pos, instruction.distance);
        match instruction.direction {
            Direction::Up => self.ver_lines.push((self.pos.0, next.1, self.pos.1)),
            Direction::Down => self.ver_lines.push((self.pos.0, self.pos.1, next.1)),
            Direction::Left => self.hor_lines.push((self.pos.1, next.0, self.pos.0)),
            Direction::Right => self.hor_lines.push((self.pos.1, self.pos.0, next.0)),
        }
        self.pos = next;
    }

    fn volume(&mut self) -> usize {
        #[derive(Debug)]
        enum Corner {
            CornerUp,
            CornerDown,
        }

        fn width(y: i32, ver_lines: &[(i32, i32, i32)], hor_lines: &[(i32, i32, i32)]) -> usize {
            let mut width = 0;
            let mut inside = false;
            let mut x_last = 0;
            let mut corner = None;
            for (x, y_min, y_max) in ver_lines.iter().copied() {
                if y_min == y || y_max == y {
                    let hor_line = hor_lines
                        .iter()
                        .find(|l| l.0 == y && (l.1 == x || l.2 == x))
                        .unwrap();
                    if hor_line.1 == x {
                        if y_min == y {
                            corner = Some(Corner::CornerDown);
                        } else {
                            corner = Some(Corner::CornerUp);
                        }
                        if inside {
                            width += (x - x_last) as usize;
                        }

                        x_last = hor_line.2;

                        width += (hor_line.2 - hor_line.1) as usize;
                    } else {
                        match corner.unwrap() {
                            Corner::CornerDown => {
                                if y_max == y {
                                    inside = !inside
                                }
                            }
                            Corner::CornerUp => {
                                if y_min == y {
                                    inside = !inside
                                }
                            }
                        }

                        if inside {
                            x_last = x;
                        } else {
                            width += 1;
                        }

                        corner = None;
                    }
                } else if (y_min..=y_max).contains(&y) {
                    inside = !inside;
                    if inside {
                        x_last = x;
                    } else {
                        width += (x - x_last + 1) as usize;
                    }
                }
            }
            width
        }

        self.ver_lines.sort();
        self.hor_lines.sort();
        let ys = self.hor_lines.iter().map(|c| c.0).dedup().collect_vec();
        let mut last_y = 0;
        let mut last_width = 0;
        let mut volume = 0;
        for y in ys {
            volume += width(y, &self.ver_lines, &self.hor_lines);
            if last_width > 0 {
                volume += last_width * (y - last_y - 1) as usize;
            }
            last_width = width(y + 1, &self.ver_lines, &self.hor_lines);
            last_y = y;
        }

        volume
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let instructions: Vec<Instruction> = lines.iter().map(|l| l.parse()).try_collect()?;
        let hacked_instructions: Vec<HackedInstruction> =
            lines.iter().map(|l| l.parse()).try_collect()?;

        let mut sparse_grid = SparseGrid::new();
        for instruction in instructions.iter() {
            sparse_grid.apply(instruction);
        }
        let part1 = sparse_grid.volume().to_string();

        let mut sparse_grid = SparseGrid::new();
        for HackedInstruction(instruction) in hacked_instructions.iter() {
            sparse_grid.apply(instruction);
        }
        let part2 = sparse_grid.volume().to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}
