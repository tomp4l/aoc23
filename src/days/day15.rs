use itertools::Itertools;

use super::day::{Day, DayResult};

pub struct Instance;

fn hash(s: &str) -> u8 {
    let mut ret = 0;
    for c in s.as_bytes() {
        let n = ret as u16 + (*c as u16);
        ret = ((n * 17) % 256) as u8;
    }
    ret
}

#[derive(Debug)]
enum Instruction<'a> {
    Add { label: &'a str, focal_length: u8 },
    Remove { label: &'a str },
}

impl<'a> Instruction<'a> {
    fn from_str(s: &'a str) -> Self {
        if s.contains('=') {
            let (label, c) = s.split_once('=').unwrap();
            let focal_length = c.parse::<u8>().unwrap();
            Instruction::Add {
                label,
                focal_length,
            }
        } else {
            Instruction::Remove {
                label: &s[..s.len() - 1],
            }
        }
    }
}

struct HashMap<'a>(Vec<Vec<(&'a str, u8)>>);

impl<'a> HashMap<'a> {
    fn new() -> Self {
        HashMap(vec![Vec::new(); 256])
    }

    fn insert(&mut self, label: &'a str, value: u8) {
        let hash = hash(label) as usize;
        if let Some(v) = self.0[hash].iter_mut().find(|c| c.0 == label) {
            v.1 = value;
        } else {
            self.0[hash].push((label, value));
        }
    }

    fn remove(&mut self, label: &'a str) {
        let hash = hash(label) as usize;
        self.0[hash].retain(|c| c.0 != label);
    }

    fn power(&self) -> u32 {
        let mut power = 0;
        for i in 0..256 {
            let b = &self.0[i];
            for (j, (_, p)) in b.iter().enumerate() {
                power += ((i + 1) as u32) * ((j + 1) as u32) * (*p as u32);
            }
        }
        power
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let line = &lines[0];

        let part1 = line
            .split(',')
            .map(hash)
            .map(|h| h as u32)
            .sum::<u32>()
            .to_string();

        let instructions = line.split(',').map(Instruction::from_str).collect_vec();

        let mut map = HashMap::new();

        for instruction in instructions {
            match instruction {
                Instruction::Add {
                    label,
                    focal_length,
                } => map.insert(label, focal_length),
                Instruction::Remove { label } => map.remove(label),
            }
        }

        Ok(DayResult {
            part1,
            part2: Some(map.power().to_string()),
        })
    }
}
