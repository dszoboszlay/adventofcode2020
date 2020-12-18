use std::rc::Rc;
use crate::{Day, Part};

type Input = Rc<Vec<usize>>;

struct Part1 {
    input: Input
}

impl Part for Part1 {
    fn solve(&self) -> i64 { p01(&self.input) }
}

struct Part2 {
    input: Input
}

impl Part for Part2 {
    fn solve(&self) -> i64 { p02(&self.input) }
}

fn p01(input: &Input) -> i64 {
    let mut deltas = [0, 0, 0, 1];
    let mut last = 0usize;

    for jolt in input.iter() {
        deltas[jolt - last] += 1;
        last = *jolt;
    }

    return (deltas[1] * deltas[3]) as i64
}

fn p02(input: &Input) -> i64 {
    let mut last = 0usize;
    let mut options = [1i64, 1i64, 1i64, 1i64, 0i64, 0i64];

    for jolt in input.iter() {
        let step = jolt - last;
        if step > 0 {
            options[0] = options[step];
            options[1] = options[step + 1];
            options[2] = options[step + 2];
            options[3] = 0i64;
        }
        options[1] += options[0];
        options[2] += options[0];
        options[3] += options[0];

        last = *jolt;
    }
    
    options[3]
}

pub fn parse(s: String) -> Day {
    let mut input: Vec<usize> = s.lines().map(|l| l.parse().unwrap() ).collect();
    input.sort();

    let part1 = Box::new(Part1 { input: Rc::new(input) });
    let part2 = Box::new(Part2 { input: part1.input.clone() });
    Day {
        parts: vec![part1, part2]
    }
}
