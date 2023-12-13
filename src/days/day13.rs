use std::collections::HashMap;

use itertools::Itertools;

use super::day::{Day, DayResult};

pub struct Instance;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Ground {
    Ash,
    Rocks,
}

impl Ground {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Ash,
            _ => Self::Rocks,
        }
    }
}

#[derive(Debug)]
struct Map {
    map: HashMap<(usize, usize), Ground>,
    x_len: usize,
    y_len: usize,
}

impl Map {
    fn from_lines(lines: &[&String]) -> Self {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut map = HashMap::new();
        for (y, l) in lines.iter().enumerate() {
            max_y = max_y.max(y);
            for (x, c) in l.chars().enumerate() {
                max_x = max_x.max(x);
                map.insert((x, y), Ground::from_char(c));
            }
        }
        Map {
            map,
            x_len: max_x + 1,
            y_len: max_y + 1,
        }
    }

    fn columns(&self) -> Vec<Vec<&Ground>> {
        let mut ret = Vec::new();
        for x in 0..self.x_len {
            let mut current = Vec::new();
            for y in 0..self.y_len {
                current.push(&self.map[&(x, y)]);
            }
            ret.push(current);
        }
        ret
    }

    fn rows(&self) -> Vec<Vec<&Ground>> {
        let mut ret = Vec::new();
        for y in 0..self.y_len {
            let mut current = Vec::new();
            for x in 0..self.x_len {
                current.push(&self.map[&(x, y)]);
            }
            ret.push(current);
        }
        ret
    }

    fn reflect_row(&self, rev: bool) -> Option<usize> {
        let rows = self.rows();

        let i = if rev {
            (1..self.y_len).rev().collect_vec()
        } else {
            (1..self.y_len).collect_vec()
        };

        'outer: for y in i {
            let mut a = y - 1;
            let mut b = y;

            loop {
                if rows[a] != rows[b] {
                    continue 'outer;
                }

                if a == 0 || b == self.y_len - 1 {
                    return Some(y * 100);
                }

                a -= 1;
                b += 1;
            }
        }

        None
    }

    fn reflect_column(&self, rev: bool) -> Option<usize> {
        let columns = self.columns();

        let i = if rev {
            (1..self.x_len).rev().collect_vec()
        } else {
            (1..self.x_len).collect_vec()
        };

        'outer: for x in i {
            let mut a = x - 1;
            let mut b = x;

            loop {
                if columns[a] != columns[b] {
                    continue 'outer;
                }

                if a == 0 || b == self.x_len - 1 {
                    return Some(x);
                }

                a -= 1;
                b += 1;
            }
        }

        None
    }

    fn reflect(&self) -> usize {
        self.reflect_column(false)
            .or(self.reflect_row(false))
            .unwrap()
    }

    fn smudge(&self, x: usize, y: usize) -> Self {
        let mut map = self.map.clone();
        map.entry((x, y)).and_modify(|e| {
            *e = match e {
                Ground::Ash => Ground::Rocks,
                Ground::Rocks => Ground::Ash,
            }
        });
        Map {
            map,
            x_len: self.x_len,
            y_len: self.y_len,
        }
    }

    fn reflect_smudge(&self) -> usize {
        let current = self.reflect();
        for x in 0..self.x_len {
            for y in 0..self.y_len {
                let smudge = self.smudge(x, y);
                if let Some(reflect) = smudge.reflect_column(false) {
                    if reflect != current {
                        return reflect;
                    }
                }
                if let Some(reflect) = smudge.reflect_row(false) {
                    if reflect != current {
                        return reflect;
                    }
                }
                if let Some(reflect) = smudge.reflect_column(true) {
                    if reflect != current {
                        return reflect;
                    }
                }
                if let Some(reflect) = smudge.reflect_row(true) {
                    if reflect != current {
                        return reflect;
                    }
                }
            }
        }

        panic!();
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let maps = lines
            .iter()
            .group_by(|l| l.as_str() != "")
            .into_iter()
            .filter(|s| s.0)
            .map(|s| Map::from_lines(&s.1.collect_vec()))
            .collect_vec();

        let part1: usize = maps.iter().map(|m| m.reflect()).sum();
        let part2: usize = maps.iter().map(|m| m.reflect_smudge()).sum();

        Ok(DayResult {
            part1: part1.to_string(),
            part2: Some(part2.to_string()),
        })
    }
}
