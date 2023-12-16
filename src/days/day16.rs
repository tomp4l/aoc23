use std::{
    collections::{HashMap, HashSet},
    iter,
};

use itertools::Itertools;

use super::day::{Day, DayResult};

pub struct Instance;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn next(&self, c: &(usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (c.0, c.1 - 1),
            Direction::Down => (c.0, c.1 + 1),
            Direction::Right => (c.0 + 1, c.1),
            Direction::Left => (c.0 - 1, c.1),
        }
    }
}

#[derive(Debug)]
enum Mirror {
    NorthEast,
    NorthWest,
}

impl Mirror {
    fn next(&self, d: &Direction, c: &(usize, usize)) -> (&'static Direction, (usize, usize)) {
        let d = match self {
            Mirror::NorthEast => match d {
                Direction::Up => &Direction::Right,
                Direction::Down => &Direction::Left,
                Direction::Right => &Direction::Up,
                Direction::Left => &Direction::Down,
            },
            Mirror::NorthWest => match d {
                Direction::Up => &Direction::Left,
                Direction::Down => &Direction::Right,
                Direction::Right => &Direction::Down,
                Direction::Left => &Direction::Up,
            },
        };

        (d, d.next(c))
    }
}

#[derive(Debug)]
enum Splitter {
    Horizontal,
    Vertical,
}

impl Splitter {
    fn next(
        &self,
        d: &'static Direction,
        c: &(usize, usize),
    ) -> Vec<(&'static Direction, (usize, usize))> {
        match (self, d) {
            (Splitter::Horizontal, Direction::Right)
            | (Splitter::Horizontal, Direction::Left)
            | (Splitter::Vertical, Direction::Up)
            | (Splitter::Vertical, Direction::Down) => vec![(d, d.next(c))],
            (Splitter::Vertical, Direction::Right) | (Splitter::Vertical, Direction::Left) => {
                vec![
                    (&Direction::Up, Direction::Up.next(c)),
                    (&Direction::Down, Direction::Down.next(c)),
                ]
            }
            (Splitter::Horizontal, Direction::Up) | (Splitter::Horizontal, Direction::Down) => {
                vec![
                    (&Direction::Right, Direction::Right.next(c)),
                    (&Direction::Left, Direction::Left.next(c)),
                ]
            }
        }
    }
}

#[derive(Debug)]
enum GridEntry {
    Mirror(Mirror),
    Splitter(Splitter),
}

impl GridEntry {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '/' => Some(GridEntry::Mirror(Mirror::NorthEast)),
            '\\' => Some(GridEntry::Mirror(Mirror::NorthWest)),
            '|' => Some(GridEntry::Splitter(Splitter::Vertical)),
            '-' => Some(GridEntry::Splitter(Splitter::Horizontal)),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Grid {
    entries: HashMap<(usize, usize), GridEntry>,
    x_len: usize,
    y_len: usize,
}

impl Grid {
    fn from_lines(lines: &[String]) -> Self {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut entries = HashMap::new();
        for (y, l) in lines.iter().enumerate() {
            max_y = max_y.max(y);
            for (x, c) in l.chars().enumerate() {
                max_x = max_x.max(x);
                if let Some(e) = GridEntry::from_char(c) {
                    entries.insert((x + 1, y + 1), e);
                }
            }
        }
        Grid {
            entries,
            x_len: max_x + 1,
            y_len: max_y + 1,
        }
    }

    fn beam(&self, start: &(&'static Direction, (usize, usize))) -> usize {
        let mut visited = HashSet::new();
        visited.insert(start.clone());
        let mut beams = vec![start.clone()];

        while let Some(beam) = beams.pop() {
            if let Some(g) = self.entries.get(&beam.1) {
                let next = match g {
                    GridEntry::Mirror(m) => vec![m.next(beam.0, &beam.1)],
                    GridEntry::Splitter(s) => s.next(beam.0, &beam.1),
                };

                let valid = next
                    .iter()
                    .filter(|c| self.contains(&c.1) && !visited.contains(c))
                    .collect_vec();

                visited.extend(valid.clone());
                beams.extend(valid);
            } else {
                let next_coord = beam.0.next(&beam.1);
                let next = (beam.0, next_coord);

                if self.contains(&next_coord) && !visited.contains(&next) {
                    visited.insert(next.clone());
                    beams.push(next);
                }
            }
        }

        let distinct: HashSet<(usize, usize)> = HashSet::from_iter(visited.iter().map(|c| c.1));

        distinct.len()
    }

    fn beam_all(&self) -> usize {
        let mut starts = Vec::new();

        starts.extend(iter::repeat(&Direction::Down).zip((1..=self.x_len).zip(iter::repeat(1))));
        starts.extend(
            iter::repeat(&Direction::Up).zip((1..=self.x_len).zip(iter::repeat(self.y_len))),
        );
        starts.extend(iter::repeat(&Direction::Right).zip(iter::repeat(1).zip(1..=self.y_len)));
        starts.extend(
            iter::repeat(&Direction::Left).zip(iter::repeat(self.x_len).zip(1..=self.y_len)),
        );

        starts.iter().map(|s| self.beam(s)).max().unwrap()
    }

    fn contains(&self, c: &(usize, usize)) -> bool {
        c.0 > 0 && c.1 > 0 && c.0 <= self.x_len && c.1 <= self.y_len
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let grid = Grid::from_lines(&lines);

        let part1 = grid.beam(&(&Direction::Right, (1, 1))).to_string();
        let part2 = grid.beam_all().to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}
