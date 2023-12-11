use std::{
    collections::{HashMap, HashSet},
    iter,
};

use super::day::{Day, DayResult};

pub struct Instance;

#[derive(Debug, Clone)]
enum Pipe {
    Vertical,
    Horizontal,
    BendNe,
    BendNw,
    BendSw,
    BendSe,
    Ground,
    Start,
}

impl Pipe {
    fn next(&self, direction: &Direction) -> Option<Direction> {
        let (f, t) = match self {
            Pipe::Vertical => (Direction::North, Direction::South),
            Pipe::Horizontal => (Direction::East, Direction::West),
            Pipe::BendNe => (Direction::North, Direction::East),
            Pipe::BendNw => (Direction::North, Direction::West),
            Pipe::BendSw => (Direction::South, Direction::West),
            Pipe::BendSe => (Direction::South, Direction::East),
            Pipe::Ground => return None,
            Pipe::Start => return Some(*direction),
        };

        let entry_direction = direction.opposite();

        if f == entry_direction {
            Some(t)
        } else if t == entry_direction {
            Some(f)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn all() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

impl Pipe {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::BendNe,
            'J' => Pipe::BendNw,
            '7' => Pipe::BendSw,
            'F' => Pipe::BendSe,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            c => panic!("unmatched: {}", c),
        }
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Coord(u16, u16);

impl Coord {
    fn next(&self, direction: &Direction) -> Coord {
        match direction {
            Direction::North => Coord(self.0, self.1 - 1),
            Direction::East => Coord(self.0 + 1, self.1),
            Direction::South => Coord(self.0, self.1 + 1),
            Direction::West => Coord(self.0 - 1, self.1),
        }
    }

    fn out_corner(
        &self,
        out: Direction,
        other: Direction,
    ) -> (Vec<(Coord, InsideOutside)>, Direction) {
        (
            [
                self.next(&out),
                self.next(&other),
                self.next(&out).next(&other),
            ]
            .into_iter()
            .zip(iter::repeat(InsideOutside::Outside))
            .collect(),
            out,
        )
    }

    fn in_corner(
        &self,
        in_: Direction,
        other: Direction,
    ) -> (Vec<(Coord, InsideOutside)>, Direction) {
        (
            [
                self.next(&in_),
                self.next(&other),
                self.next(&in_).next(&other),
            ]
            .into_iter()
            .zip(iter::repeat(InsideOutside::Inside))
            .collect(),
            in_.opposite(),
        )
    }

    fn out_line(&self, out: Direction) -> (Vec<(Coord, InsideOutside)>, Direction) {
        (
            (vec![
                (self.next(&out), InsideOutside::Outside),
                (self.next(&out.opposite()), InsideOutside::Inside),
            ]),
            out,
        )
    }
}

#[derive(Clone, Copy)]
enum InsideOutside {
    Inside,
    Outside,
    Either,
}

struct Map(HashMap<Coord, Pipe>);

impl Map {
    fn from_lines(lines: &[String]) -> Self {
        let mut map = HashMap::new();

        lines.iter().zip(1..).for_each(|(s, y)| {
            s.chars().zip(1..).for_each(|(c, x)| {
                map.insert(Coord(x, y), Pipe::from_char(c));
            })
        });
        Map(map)
    }

    fn find_distance(&self) -> usize {
        self.find_loop().len() / 2
    }

    fn find_loop(&self) -> Vec<Coord> {
        let (start, pipe) = self
            .0
            .iter()
            .find(|(_, v)| matches!(v, Pipe::Start))
            .unwrap();

        let mut start_direction = Direction::North;

        loop {
            let mut current_pos = *start;
            let mut current_pipe = pipe;

            let mut current_direction = start_direction;
            let mut found_loop = vec![*start];

            loop {
                if let Some(next_direction) = (current_pipe).next(&current_direction) {
                    let next_pos = (current_pos).next(&next_direction);
                    if let Some(next_pipe) = self.0.get(&next_pos) {
                        current_pipe = next_pipe;
                        current_pos = next_pos;
                        current_direction = next_direction;
                        found_loop.push(current_pos);
                    } else {
                        found_loop = vec![];
                        break;
                    }
                } else {
                    found_loop = vec![];
                    break;
                }

                if &current_pos == start {
                    break;
                }
            }

            if found_loop.is_empty() {
                match start_direction {
                    Direction::North => start_direction = Direction::East,
                    Direction::East => start_direction = Direction::South,
                    Direction::South => start_direction = Direction::West,
                    Direction::West => panic!("no loop"),
                };
            } else {
                return found_loop;
            }
        }
    }

    fn count_inside(&self) -> usize {
        let path = self.find_loop();
        let mut inside_outside = InOutMap::new(&mut self.0.keys(), &self.find_loop());

        let mut outside: Option<(&Coord, Direction)> = None;

        for p in path.iter().skip(1).cycle() {
            let pipe = &self.0[p];

            let pipe = if matches!(pipe, Pipe::Start) {
                let b = path[1];
                let a = path[path.len() - 2];

                match (a.0.cmp(&b.0), a.1.cmp(&b.1)) {
                    (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => &Pipe::BendSw,
                    (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => &Pipe::Horizontal,
                    (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => &Pipe::BendNw,
                    (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => &Pipe::Vertical,
                    (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => unreachable!(),
                    (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => &Pipe::Vertical,
                    (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => &Pipe::BendSe,
                    (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => &Pipe::Horizontal,
                    (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => &Pipe::BendNe,
                }
            } else {
                pipe
            };

            if let Some((prev, out)) = &mut outside {
                let (in_out, new_out) = match pipe {
                    Pipe::Vertical | Pipe::Horizontal => p.out_line(*out),
                    Pipe::BendNe | Pipe::BendNw | Pipe::BendSe | Pipe::BendSw => {
                        let (ns, ew) = match pipe {
                            Pipe::BendNe => (Direction::North, Direction::East),
                            Pipe::BendNw => (Direction::North, Direction::West),
                            Pipe::BendSw => (Direction::South, Direction::West),
                            Pipe::BendSe => (Direction::South, Direction::East),
                            _ => unreachable!(),
                        };

                        let (in_out, other) = match out {
                            Direction::North | Direction::South => (ew, ns),
                            Direction::East | Direction::West => (ns, ew),
                        };
                        if &other == out {
                            p.in_corner(in_out.opposite(), other.opposite())
                        } else {
                            p.out_corner(in_out.opposite(), other.opposite())
                        }
                    }
                    Pipe::Ground | Pipe::Start => unreachable!(),
                };

                for (c, i) in &in_out {
                    if let Some(v) = inside_outside.0.get_mut(c) {
                        match (&v, i) {
                            (InsideOutside::Inside, InsideOutside::Outside)
                            | (InsideOutside::Outside, InsideOutside::Inside)
                            | (_, InsideOutside::Either) => unreachable!(),
                            (InsideOutside::Inside, InsideOutside::Inside)
                            | (InsideOutside::Outside, InsideOutside::Outside) => (),
                            (InsideOutside::Either, i) => {
                                *v = *i;
                                inside_outside.flood()
                            }
                        }
                    }
                }

                *prev = p;
                *out = new_out;

                if inside_outside.is_complete() {
                    break;
                }
            } else {
                for d in Direction::all() {
                    let c = p.next(&d);
                    if let Some(InsideOutside::Outside) = inside_outside.0.get(&c) {
                        if matches!(pipe, Pipe::Vertical | Pipe::Horizontal) {
                            outside = Some((p, d));
                            break;
                        }
                    }
                }
            }
        }

        inside_outside.inside()
    }
}

struct InOutMap<'a>(HashMap<&'a Coord, InsideOutside>);

#[allow(dead_code)]
impl<'a> InOutMap<'a> {
    fn new<I: Iterator<Item = &'a Coord>>(all_coords: &mut I, path: &[Coord]) -> InOutMap<'a> {
        let path_coords: HashSet<Coord> = HashSet::from_iter(path.to_vec());
        let mut inside_outside = HashMap::new();

        let mut max_x = 0;
        let mut max_y = 0;

        for c in all_coords {
            if !path_coords.contains(c) {
                inside_outside.insert(c, InsideOutside::Either);
                max_x = c.0.max(max_x);
                max_y = c.1.max(max_y);
            }
        }

        for x in 1..=(max_x) {
            if let Some(c) = inside_outside.get_mut(&Coord(x, 1)) {
                *c = InsideOutside::Outside;
            }
            if let Some(c) = inside_outside.get_mut(&Coord(x, max_y)) {
                *c = InsideOutside::Outside;
            }
        }

        for y in 1..=(max_y) {
            if let Some(c) = inside_outside.get_mut(&Coord(1, y)) {
                *c = InsideOutside::Outside;
            }
            if let Some(c) = inside_outside.get_mut(&Coord(max_x, y)) {
                *c = InsideOutside::Outside;
            }
        }

        let mut ret = Self(inside_outside);
        ret.flood();
        ret
    }

    fn flood(&mut self) {
        let mut candidates: Vec<_> = self
            .0
            .iter()
            .filter(|(_, m)| !matches!(m, InsideOutside::Either))
            .map(|(a, b)| (**a, *b))
            .collect();
        while let Some((c, i)) = candidates.pop() {
            for direction in Direction::all() {
                let nc = c.next(&direction);
                if let Some(n) = self.0.get_mut(&nc) {
                    if matches!(n, InsideOutside::Either) {
                        *n = i;
                        candidates.push((nc, i))
                    }
                }
            }
        }
    }

    fn is_complete(&self) -> bool {
        self.0
            .values()
            .filter(|p| matches!(p, InsideOutside::Either))
            .count()
            == 0
    }

    fn inside(&self) -> usize {
        self.0
            .values()
            .filter(|v| matches!(v, InsideOutside::Inside))
            .count()
    }

    fn print(&self) {
        let max_x = self.0.keys().map(|c| c.0).max().unwrap();
        let max_y = self.0.keys().map(|c| c.1).max().unwrap();

        for y in 1..=max_y {
            for x in 1..=max_x {
                let c = match self.0.get(&Coord(x, y)) {
                    Some(InsideOutside::Inside) => 'I',
                    Some(InsideOutside::Outside) => 'O',
                    Some(InsideOutside::Either) => 'E',
                    _ => '.',
                };
                print!("{}", c)
            }
            println!()
        }
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let map = Map::from_lines(&lines);

        let part1 = map.find_distance().to_string();
        let part2 = map.count_inside().to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}
