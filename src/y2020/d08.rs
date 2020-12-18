use std::rc::Rc;
use std::str::FromStr;
use crate::{Day, Part};

enum Op {
    Nop(i64),
    Jmp(i64),
    Acc(i64)
}

impl FromStr for Op {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s.chars().next().unwrap();
        match s[4..].parse::<i64>() {
            Result::Ok(i) => if c == 'j' {
                Result::Ok(Op::Jmp(i))
            } else if c == 'a' {
                Result::Ok(Op::Acc(i))
            } else {
                Result::Ok(Op::Nop(i))
            },
            Result::Err(err) => Result::Err(err)
        }
    }
}

type Program = Vec<Op>;
type Input = Rc<Program>;

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
    let mut acc = 0i64;
    let mut ip = 0usize;
    let mut first_time = vec![true; input.len()];
    
    while first_time[ip] {
        first_time[ip] = false;
        match input[ip] {
            Op::Nop(_) => ip += 1,
            Op::Acc(n) => { acc += n; ip += 1 },
            Op::Jmp(n) => ip = (ip as i64 + n) as usize
        }
    };

    acc
}

fn p02(input: &Input) -> i64 {
    fn exec_branch(acc: i64, ip: i64, input: &Input, loop_check: &mut Vec<u16>, loop_cnt: u16) -> Option<i64> {
        if ip < 0 || ip > input.len() as i64 + 1 { return Option::None }

        let mut acc = acc;
        let mut ip = ip as usize;
        loop {
            if ip == input.len() { return Option::Some(acc) }
            if loop_check[ip] >= loop_cnt { return Option::None }
            loop_check[ip] = loop_cnt;

            match input[ip] {
                Op::Acc(n) => { acc += n; ip += 1 },
                Op::Nop(_) => ip += 1,
                Op::Jmp(n) => ip = (ip as i64 + n) as usize
            }
        }
    }

    let mut acc = 0i64;
    let mut ip = 0usize;
    let mut loop_check = vec![0u16; input.len()];
    let mut loop_cnt = 0u16;

    loop {
        loop_check[ip] = u16::MAX;
        match input[ip] {
            Op::Acc(n) => { acc += n; ip += 1 },
            Op::Nop(0) => ip += 1,
            Op::Nop(n) => {
                loop_cnt += 1;
                match exec_branch(acc, ip as i64 + n, input, &mut loop_check, loop_cnt) {
                    Option::Some(res) => return res,
                    Option::None => ip += 1
                }
            },
            Op::Jmp(n) => {
                loop_cnt += 1;
                match exec_branch(acc, ip as i64 + 1, input, &mut loop_check, loop_cnt) {
                    Option::Some(res) => return res,
                    Option::None => ip = (ip as i64 + n) as usize
                }
            }
        }
    }
}

pub fn parse(s: String) -> Day {
    let input: Program = s.lines().map(|l| l.parse().unwrap() ).collect();

    let part1 = Box::new(Part1 { input: Rc::new(input) });
    let part2 = Box::new(Part2 { input: part1.input.clone() });
    Day {
        parts: vec![part1, part2]
    }
}
