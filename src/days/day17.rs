use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use super::day::{Day, DayResult};

pub struct Instance;

#[derive(Debug)]
struct LavaPool {
    cells: HashMap<(usize, usize), u32>,
    x_len: usize,
    y_len: usize,
}

impl LavaPool {
    fn from_lines(lines: &[String]) -> Self {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut cells = HashMap::new();
        for (y, l) in lines.iter().enumerate() {
            max_y = max_y.max(y);
            for (x, c) in l.chars().enumerate() {
                max_x = max_x.max(x);
                if let Some(cell) = c.to_digit(10) {
                    cells.insert((x + 1, y + 1), cell);
                }
            }
        }
        LavaPool {
            cells,
            x_len: max_x + 1,
            y_len: max_y + 1,
        }
    }
}

impl LavaPool {
    fn min_heat(&self, min: u8, max: u8) -> u32 {
        let start = (1, 1);

        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        enum Direction {
            Up,
            Down,
            Left,
            Right,
        }
        use Direction::*;

        impl Direction {
            fn turn(&self) -> (&'static Self, &'static Self) {
                match self {
                    Up | Down => (&Left, &Right),
                    Left | Right => (&Up, &Down),
                }
            }

            fn next(&self, c: &(usize, usize)) -> (usize, usize) {
                match self {
                    Up => (c.0, c.1 - 1),
                    Down => (c.0, c.1 + 1),
                    Right => (c.0 + 1, c.1),
                    Left => (c.0 - 1, c.1),
                }
            }
        }

        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
        struct State(u32, u8, (usize, usize), &'static Direction);

        let mut heap: BinaryHeap<Reverse<State>> = BinaryHeap::new();
        let mut seen = HashSet::new();

        heap.push(Reverse(State(0, 0, start, &Right)));
        heap.push(Reverse(State(0, 0, start, &Down)));

        while let Some(Reverse(State(heat, steps, coord, direction))) = heap.pop() {
            if coord == (self.x_len, self.y_len) {
                return heat;
            }

            if steps + 1 < max {
                let next = direction.next(&coord);
                if let Some(next_heat) = self.cells.get(&next) {
                    let state = State(heat + next_heat, steps + 1, next, direction);
                    if seen.insert((state.1, state.2, state.3)) {
                        heap.push(Reverse(state));
                    }
                }
            }

            if steps + 1 >= min {
                let (a, b) = direction.turn();

                let next = a.next(&coord);
                if let Some(next_heat) = self.cells.get(&next) {
                    let state = State(heat + next_heat, 0, next, a);
                    if seen.insert((state.1, state.2, state.3)) {
                        heap.push(Reverse(state));
                    }
                }

                let next = b.next(&coord);
                if let Some(next_heat) = self.cells.get(&next) {
                    let state = State(heat + next_heat, 0, next, b);
                    if seen.insert((state.1, state.2, state.3)) {
                        heap.push(Reverse(state));
                    }
                }
            }
        }

        panic!("Didn't find path")
    }

    fn min_heat_basic(&self) -> u32 {
        self.min_heat(0, 3)
    }

    fn min_heat_ultra(&self) -> u32 {
        self.min_heat(4, 10)
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let pool = LavaPool::from_lines(&lines);

        let part1: String = pool.min_heat_basic().to_string();
        let part2: String = pool.min_heat_ultra().to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}
