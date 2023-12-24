use std::{mem::swap, str::FromStr};

use itertools::Itertools;

use super::day::{Day, DayResult};

pub struct Instance;

#[derive(Debug, Clone)]
struct Coord {
    x: i128,
    y: i128,
    z: i128,
}

impl FromStr for Coord {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s.split(", ").collect_tuple().ok_or("bad coord format")?;

        let x = x.parse().map_err(|e| format!("x {e} {x}"))?;
        let y = y.parse().map_err(|e| format!("y {e} {y}"))?;
        let z = z.parse().map_err(|e| format!("z {e} {z}"))?;

        Ok(Coord { x, y, z })
    }
}

#[derive(Debug, Clone)]
struct Hailstone {
    position: Coord,
    velocity: Coord,
}

impl FromStr for Hailstone {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split_once(" @ ").ok_or("bad hailstone format")?;
        let position = p.parse()?;
        let velocity = v.parse()?;

        Ok(Hailstone { position, velocity })
    }
}

impl Hailstone {
    fn cross_xy(&self, other: &Hailstone) -> Option<(i128, i128)> {
        let x1 = self.position.x;
        let x2 = self.position.x + self.velocity.x;
        let x3 = other.position.x;
        let x4 = other.position.x + other.velocity.x;

        let y1 = self.position.y;
        let y2 = self.position.y + self.velocity.y;
        let y3 = other.position.y;
        let y4 = other.position.y + other.velocity.y;

        let denom_x = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        let denom_y = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if denom_x != 0 && denom_y != 0 {
            let px = ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4)) / denom_x;
            let py = ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4)) / denom_y;

            let t1 = self.velocity.x == 0 || ((px - x1) / (self.velocity.x)) > 0;
            let t2 = other.velocity.x == 0 || ((px - x3) / (other.velocity.x)) > 0;

            if t1 && t2 {
                Some((px, py))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let hailstones: Vec<Hailstone> = lines.iter().map(|l| l.parse()).try_collect()?;

        let pairs = hailstones.iter().tuple_combinations();

        let mut crossed = 0;
        for (a, b) in pairs {
            if let Some((x, y)) = a.cross_xy(b) {
                if (200000000000000..400000000000000).contains(&x)
                    && (200000000000000..400000000000000).contains(&y)
                {
                    crossed += 1;
                }
            }
        }

        let mut part2 = None;
        'outer: for xv in -300..300 {
            for yv in -300..300 {
                let mut new_stones = hailstones.clone();

                new_stones.iter_mut().for_each(|h| {
                    h.velocity.x += xv;
                    h.velocity.y += yv;
                });

                if let Some(v1) = new_stones[0].cross_xy(&new_stones[1]) {
                    if new_stones.iter().tuple_combinations().all(|(a, b)| {
                        if let Some(c) = a.cross_xy(b) {
                            c == v1
                        } else {
                            true
                        }
                    }) {
                        for zv in -300..300 {
                            let mut new_stones = new_stones.clone();
                            new_stones.iter_mut().for_each(|h| {
                                h.velocity.z += zv;
                                swap(&mut h.velocity.y, &mut h.velocity.z);
                                swap(&mut h.position.y, &mut h.position.z);
                            });
                            if let Some(v2) = new_stones[0].cross_xy(&new_stones[1]) {
                                if new_stones.iter().tuple_combinations().all(|(a, b)| {
                                    if let Some(c) = a.cross_xy(b) {
                                        c == v2
                                    } else {
                                        true
                                    }
                                }) {
                                    part2 = Some(v1.0 + v1.1 + v2.1);
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(DayResult {
            part1: crossed.to_string(),
            part2: part2.map(|i| i.to_string()),
        })
    }
}
