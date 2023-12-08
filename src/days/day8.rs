use std::{collections::HashMap, mem::swap};

use itertools::Itertools;

use super::day::{Day, DayResult};

pub struct Instance;

enum Instruction {
    Left,
    Right,
}

impl Instruction {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!(),
        }
    }
}

struct Network(HashMap<String, (String, String)>);

impl Network {
    fn new(lines: &[String]) -> Self {
        let mut map = HashMap::new();

        for line in lines {
            let from = line[0..3].to_owned();
            let left = line[7..10].to_owned();
            let right = line[12..15].to_owned();

            map.insert(from, (left, right));
        }

        Network(map)
    }

    fn traverse(&self, instructions: &[Instruction]) -> usize {
        let mut steps = 0;
        let mut current = "AAA";

        let mut instructions = instructions.iter().cycle();

        while current != "ZZZ" {
            let instruction = instructions.next().unwrap();
            current = self.step(current, instruction);

            steps += 1;
        }

        steps
    }

    fn step(&self, current: &str, instruction: &Instruction) -> &str {
        let (left, right) = &self.0[current];
        match instruction {
            Instruction::Left => left,
            Instruction::Right => right,
        }
    }

    fn traverse_ghost(&self, instructions: &[Instruction]) -> usize {
        let mut ghosts: Vec<_> = self
            .0
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|s| s.as_str())
            .collect();
        let mut instructions = instructions.iter().cycle();

        let mut steps = 0;

        let mut pattern = vec![Vec::new(); ghosts.len()];

        while pattern.iter().filter(|g| g.len() >= 3).count() != ghosts.len() {
            let instruction = instructions.next().unwrap();
            steps += 1;

            for (i, ghost) in ghosts.iter_mut().enumerate() {
                *ghost = &mut self.step(&ghost, instruction);

                if ghost.ends_with("Z") {
                    pattern[i].push(steps)
                }
            }
        }

        pattern.iter().for_each(|v| {
            let diffs: Vec<_> = v
                .windows(2)
                .map(|w| match w {
                    [a, b] => b - a,
                    _ => unreachable!(),
                })
                .collect();
            assert_eq!(v[0], diffs[0]);
            assert!(diffs.iter().all_equal());
        });

        // This only works because they cycle back to the start and all have 1 destination
        pattern.iter().map(|v| v[0]).fold(1, |a, b| lcm(a, b))
    }
}

fn gcd(a: usize, b: usize) -> usize {
    let mut r = (a, b);

    while r.0 != 0 {
        let q = r.1 / r.0;
        swap(&mut r.0, &mut r.1);
        r.0 = r.0 - q * r.1;
    }

    r.1
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let instructions: Vec<_> = lines[0].chars().map(Instruction::from_char).collect();

        let network = Network::new(&lines[2..]);

        Ok(DayResult {
            part1: network.traverse(&instructions).to_string(),
            part2: Some(network.traverse_ghost(&instructions).to_string()),
        })
    }
}
