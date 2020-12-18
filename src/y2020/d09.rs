use std::rc::Rc;
use std::cmp::Ordering;
use crate::{Day, Part};

type Input = Rc<Vec<i64>>;

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
    let mut lo = 0usize;
    let mut hi = 25usize;

    loop {
        let preamble = &input[lo..hi];
        match preamble.into_iter().filter(|x| {
            let x = **x;
            let y = input[hi] - x;
            y != x && preamble.contains(&y)                   
        }).next() {
            Option::Some(_) =>  {
                lo += 1;
                hi += 1;
            },
            Option::None => return input[hi]
        }
    }
}

fn p02(input: &Input) -> i64 {
    let target = p01(input);
    let mut lo = 0usize;
    let mut hi = 25usize;
    let mut sum: i64  = input[lo..hi].iter().sum();

    loop {
        match sum.cmp(&target) {
            Ordering::Equal => {
                let r = &input[lo..hi];
                return r.iter().min().unwrap() + r.iter().max().unwrap();
            },
            Ordering::Less => {
                sum += input[hi];
                hi += 1;
            },
            Ordering::Greater => {
                sum -= input[lo];
                lo += 1;
                if lo + 1 >= hi {
                    sum += input[hi];
                    hi += 1;
                }
            }
        }
    }
}

pub fn parse(s: String) -> Day {
    let input: Vec<i64> = s.lines().map(|l| l.parse().unwrap() ).collect();

    let part1 = Box::new(Part1 { input: Rc::new(input) });
    let part2 = Box::new(Part2 { input: part1.input.clone() });
    Day {
        parts: vec![part1, part2]
    }
}
