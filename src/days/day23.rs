use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use super::day::{Day, DayResult};

pub struct Instance;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
enum Square {
    Wall,
    Floor,
    Slope(Direction),
}

impl Square {
    fn from_char(c: char) -> Option<Self> {
        Some(match c {
            '#' => Self::Wall,
            '.' => Self::Floor,
            '^' => Self::Slope(Direction::North),
            '>' => Self::Slope(Direction::East),
            'v' => Self::Slope(Direction::South),
            '<' => Self::Slope(Direction::West),
            _ => None?,
        })
    }
}

#[derive(Debug)]
struct Maze {
    squares: HashMap<(usize, usize), Square>,
    y_len: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl Maze {
    fn from_lines(lines: &[String]) -> Self {
        let mut max_y = 0;
        let mut squares = HashMap::new();
        for (y, l) in lines.iter().enumerate() {
            max_y = max_y.max(y);
            for (x, c) in l.chars().enumerate() {
                if let Some(e) = Square::from_char(c) {
                    squares.insert((x + 1, y + 1), e);
                }
            }
        }

        let start = *squares
            .iter()
            .find(|(c, s)| matches!(s, Square::Floor) && c.1 == 1)
            .unwrap()
            .0;

        let end = *squares
            .iter()
            .find(|(c, s)| matches!(s, Square::Floor) && c.1 == max_y + 1)
            .unwrap()
            .0;

        Maze {
            squares,
            y_len: max_y + 1,
            start,
            end,
        }
    }

    fn path(&self) -> usize {
        #[derive(Debug, PartialEq, Eq, Clone)]
        struct Candidate((usize, usize), HashSet<(usize, usize)>);

        impl std::hash::Hash for Candidate {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
                self.1.iter().take(10).collect_vec().hash(state);
            }
        }

        let mut paths: Vec<Candidate> = vec![Candidate(self.start, HashSet::new())];
        let mut completed_paths = Vec::new();

        while let Some(path) = paths.pop() {
            let last = path.0;
            if last.1 == self.y_len {
                completed_paths.push(path);
                continue;
            }
            let candidates = vec![
                ((last.0 + 1, last.1), Direction::East),
                ((last.0 - 1, last.1), Direction::West),
                ((last.0, last.1 + 1), Direction::South),
                ((last.0, last.1 - 1), Direction::North),
            ];

            for (candidate, direction) in candidates {
                if path.1.contains(&candidate) {
                    continue;
                }
                if let Some(square) = self.squares.get(&candidate) {
                    let pass = match square {
                        Square::Wall => false,
                        Square::Floor => true,
                        Square::Slope(s) => *s == direction,
                    };
                    if pass {
                        let mut new_path = path.clone();
                        new_path.0 = candidate;
                        new_path.1.insert(path.0);

                        paths.push(new_path);
                    }
                }
            }
        }

        completed_paths.iter().map(|p| p.1.len()).max().unwrap()
    }

    fn path_no_slope(&self) -> usize {
        let mut junctions: HashSet<_> = self
            .squares
            .keys()
            .filter(|c| !matches!(self.squares[c], Square::Wall))
            .filter(|c| {
                [
                    ((c.0 + 1, c.1)),
                    ((c.0 - 1, c.1)),
                    ((c.0, c.1 + 1)),
                    ((c.0, c.1 - 1)),
                ]
                .iter()
                .filter_map(|c| self.squares.get(c))
                .filter(|c| !matches!(c, Square::Wall))
                .count()
                    > 2
            })
            .copied()
            .collect();

        junctions.extend([self.start, self.end]);

        let mut paths = Vec::new();

        for j in &junctions {
            let c = [
                ((j.0 + 1, j.1)),
                ((j.0 - 1, j.1)),
                ((j.0, j.1 + 1)),
                ((j.0, j.1 - 1)),
            ];
            for c in c {
                let mut steps = 1;
                match self.squares.get(&c) {
                    Some(Square::Wall) | None => continue,
                    _ => (),
                }
                let mut prev = *j;
                let mut cur = c;
                loop {
                    if junctions.contains(&cur) {
                        paths.push((j, cur, steps));
                        break;
                    }
                    steps += 1;
                    let n = [
                        ((cur.0 + 1, cur.1)),
                        ((cur.0 - 1, cur.1)),
                        ((cur.0, cur.1 + 1)),
                        ((cur.0, cur.1 - 1)),
                    ];
                    for n in n {
                        if n == prev {
                            continue;
                        }
                        match self.squares.get(&n) {
                            Some(Square::Wall) | None => continue,
                            _ => {
                                prev = cur;
                                cur = n;
                                break;
                            }
                        }
                    }
                }
            }
        }

        let mut path_distances = HashMap::new();

        for (f, t, d) in paths {
            path_distances
                .entry(*f)
                .and_modify(|p: &mut Vec<((usize, usize), usize)>| p.push((t, d)))
                .or_insert(vec![(t, d)]);
        }

        let start_candidates: usize = 1;
        let mut candidates = vec![(self.start, 0, start_candidates)];

        let mut junction_vec = junctions.iter().collect_vec();
        junction_vec.sort();

        let mut max_distance = 0;
        while let Some((c, d, v)) = candidates.pop() {
            let ps = &path_distances[&c];

            for ps in ps {
                let z = 1 << junction_vec.binary_search(&&ps.0).unwrap();
                if v & z != 0 {
                    continue;
                }
                if ps.0 == self.end {
                    if d + ps.1 > max_distance {
                        max_distance = d + ps.1;
                    }
                    continue;
                }
                let visited = v | z;

                let total = d + ps.1;
                candidates.push((ps.0, total, visited));
            }
        }

        max_distance
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let maze = Maze::from_lines(&lines);

        let part1 = maze.path().to_string();

        let part2 = maze.path_no_slope().to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}
