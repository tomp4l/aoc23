use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;
use lazy_static::lazy_static;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;

use super::day::{Day, DayResult};

pub struct Instance;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Coord {
    x: u16,
    y: u16,
    z: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Block {
    start: Coord,
    end: Coord,
}

lazy_static! {
    static ref BLOCK_REGEX: Regex = Regex::new(r"^(.*),(.*),(.*)~(.*),(.*),(.*)$").unwrap();
}

impl FromStr for Block {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cs: Vec<_> = BLOCK_REGEX
            .captures(s)
            .ok_or("Bad format")?
            .extract::<6>()
            .1
            .iter()
            .map(|l| l.parse::<u16>())
            .try_collect()
            .map_err(|e| format!("{}", e))?;

        Ok(Block {
            start: Coord {
                x: cs[0],
                y: cs[1],
                z: cs[2],
            },
            end: Coord {
                x: cs[3],
                y: cs[4],
                z: cs[5],
            },
        })
    }
}

impl Block {
    fn coords(&self) -> Vec<Coord> {
        let mut ret = Vec::new();
        for x in self.start.x..=self.end.x {
            for y in self.start.y..=self.end.y {
                for z in self.start.z..=self.end.z {
                    ret.push(Coord { x, y, z });
                }
            }
        }
        ret
    }

    fn bottom_coords(&self) -> Vec<Coord> {
        if self.start.z == self.end.z {
            self.coords()
        } else {
            vec![self.start.clone()]
        }
    }

    fn below(&self) -> Block {
        let mut b = self.clone();

        b.start.z -= 1;
        b.end.z -= 1;

        b
    }

    fn at_bottom(&self) -> bool {
        self.start.z <= 1
    }
}

#[derive(Debug, Clone)]
struct Posititions(HashMap<(u16, u16), Vec<bool>>);

impl Posititions {
    fn new(blocks: &[Block]) -> Self {
        let max_z = blocks.iter().map(|c| c.end.z).max().unwrap() + 1;
        let mut positions = HashMap::new();

        for b in blocks {
            for c in b.coords() {
                let xy = (c.x, c.y);

                let zs = positions.entry(xy).or_insert(vec![false; max_z as usize]);

                zs[c.z as usize] = true;
            }
        }

        Posititions(positions)
    }

    fn contains(&self, coord: &Coord) -> bool {
        self.0[&(coord.x, coord.y)][coord.z as usize]
    }

    fn replace(&mut self, old_coords: Vec<Coord>, new_coords: Vec<Coord>) {
        self.remove(old_coords);
        for coord in new_coords {
            self.0.get_mut(&(coord.x, coord.y)).unwrap()[coord.z as usize] = true;
        }
    }

    fn remove(&mut self, old_coords: Vec<Coord>) {
        for coord in old_coords {
            self.0.get_mut(&(coord.x, coord.y)).unwrap()[coord.z as usize] = false;
        }
    }
}

#[derive(Debug, Clone)]
struct Blocks(Vec<Block>, Posititions);

impl Blocks {
    fn new(mut blocks: Vec<Block>) -> Self {
        let positions = Posititions::new(&blocks);
        blocks.sort_by_key(|c| c.end.z);
        Blocks(blocks, positions)
    }

    fn try_drop_one(&mut self) -> bool {
        let mut moved = 0;

        for b in self.0.iter_mut() {
            let current_coords = b.coords();
            let mut next = b.clone();
            while !next.at_bottom()
                && next
                    .below()
                    .bottom_coords()
                    .iter()
                    .all(|c| !self.1.contains(c))
            {
                moved += 1;

                next = next.below();
            }
            self.1.replace(current_coords, next.coords());
            *b = next;
        }

        moved > 0
    }

    fn to_bottom(&mut self) -> bool {
        let mut moved = false;
        while self.try_drop_one() {
            moved = true;
        }
        moved
    }

    fn try_disintegrate(&mut self) -> (usize, usize) {
        self.to_bottom();

        self.0
            .clone()
            .into_par_iter()
            .map(|b| {
                let mut copy = self.clone();
                copy.0.retain(|b2| &b != b2);
                copy.1.remove(b.coords());
                if !copy.to_bottom() {
                    (1, 0)
                } else {
                    let mut old_blocks = self.0.clone();
                    old_blocks.retain(|b2| &b != b2);

                    (
                        0,
                        old_blocks
                            .into_iter()
                            .zip(copy.0)
                            .filter(|(a, b)| a != b)
                            .count(),
                    )
                }
            })
            .reduce(|| (0, 0), |(a, b), (c, d)| (a + c, b + d))
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let blocks: Vec<_> = lines.iter().map(|l| l.parse::<Block>()).try_collect()?;
        let mut blocks = Blocks::new(blocks);

        let (part1, part2) = blocks.try_disintegrate();
        Ok(DayResult {
            part1: part1.to_string(),
            part2: Some(part2.to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drop_single_horizontal_block() {
        let block = Block {
            start: Coord { x: 0, y: 0, z: 10 },
            end: Coord { x: 1, y: 0, z: 10 },
        };

        assert!(block.coords().len() == 2);
        assert!(block.coords().contains(&Coord { x: 0, y: 0, z: 10 }));
        assert!(block.coords().contains(&Coord { x: 1, y: 0, z: 10 }));

        let mut blocks = Blocks::new(vec![block]);
        blocks.to_bottom();

        assert_eq!(
            blocks.0[0],
            Block {
                start: Coord { x: 0, y: 0, z: 1 },
                end: Coord { x: 1, y: 0, z: 1 },
            }
        );
    }

    #[test]
    fn drop_single_vertical_block() {
        let block = Block {
            start: Coord { x: 0, y: 0, z: 5 },
            end: Coord { x: 0, y: 0, z: 10 },
        };

        assert!(block.coords().len() == 6);

        let mut blocks = Blocks::new(vec![block]);
        blocks.to_bottom();

        assert_eq!(
            blocks.0[0],
            Block {
                start: Coord { x: 0, y: 0, z: 1 },
                end: Coord { x: 0, y: 0, z: 6 },
            }
        );
    }
}
