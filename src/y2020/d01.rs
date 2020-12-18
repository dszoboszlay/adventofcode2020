use std::cmp::Ordering;
use std::rc::Rc;
use crate::{Day, Part};

const SUM: i32 = 2020;
type Input = Rc<Vec<i32>>;

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

pub fn parse(s: String) -> Day {
    let mut input: Vec<i32> = s.lines().map(|x| x.parse().unwrap()).collect();
    input.sort();

    let part1 = Box::new(Part1 { input: Rc::new(input) });
    let part2 = Box::new(Part2 { input: part1.input.clone() });
    Day {
        parts: vec![part1, part2]
    }
}

enum FindRes {
    Exact(usize),
    Closest(usize),
    None
}

fn find_with_upper_bound(input: &Input, l: usize, h: usize, val: i32) -> FindRes {
    if l > h || input[l] > val {
        return FindRes::None
    };

    let mut l = l;
    let mut h = h;
    while l <= h {
        let m = (l + h) / 2;
        match input[m].cmp(&val) {
            Ordering::Less => l = m + 1,
            Ordering::Greater => h = m - 1,
            Ordering::Equal => return FindRes::Exact(m)
        }
    };

    FindRes::Closest(l - 1)
}

fn find_product_by_sum(input: &Input, h: usize, sum: i32) -> Option<i64> {
    let mut l = 0;
    let mut h = h;

    while l <= h {
        match find_with_upper_bound(input, l, h, sum - input[l]) {
            FindRes::Exact(m) => return Option::Some((input[m] * input[l]) as i64),
            FindRes::Closest(m) => { h = m; l += 1 },
            FindRes::None => h -= 1
        }
        if l > h { return Option::None };
        match find_with_upper_bound(input, l, h, sum - input[h]) {
            FindRes::Exact(m) => return Option::Some((input[m] * input[h]) as i64),
            FindRes::Closest(m) => { l = m; h -= 1 },
            FindRes::None => h -= 1
        }
    };

    Option::None
}

fn p01(input: &Input) -> i64 {
    find_product_by_sum(input, input.len() - 1, SUM).unwrap()
}

fn p02(input: &Input) -> i64 {
    let mut i = 2;
    loop {
        match find_product_by_sum(input, i - 1, SUM - input[i]) {
            Option::Some(product) => return product * (input[i] as i64),
            None => i += 1
        }
    }
}