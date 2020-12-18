use std::rc::Rc;
use std::result::Result;
use std::str::FromStr;
use crate::{Day, Part};

#[derive(Debug, PartialEq)]
struct PasswordPolicy {
    min: u32,
    max: u32,
    c: char,
    pwd: String,
}

impl FromStr for PasswordPolicy {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 1-3 a: abcde
        //  ^ ^   ^
        //  | |   |
        //  | |   +-- p2 + 4
        //  | +-- p2
        //  +-- p1
        let p1 = s.find('-').unwrap();
        let p2 = s.find(' ').unwrap();
        
        match s[..p1].parse::<u32>() {
            Result::Ok(min) => {
                match s[(p1 + 1)..p2].parse::<u32>() {
                    Result::Ok(max) => Result::Ok(PasswordPolicy {
                        min: min,
                        max: max,
                        c: s[(p2 + 1)..(p2 + 2)].parse::<char>().unwrap(),
                        pwd: s[(p2 + 4)..].to_string()
                    }),
                    Result::Err(e) => Result::Err(e)
                }
            },
            Result::Err(e) => Result::Err(e)
        }
    }
}

type Input = Rc<Vec<PasswordPolicy>>;

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
    input.iter().filter(|&pwd| {
        let cnt = pwd.pwd.chars().filter(|c| *c == pwd.c).count() as u32;
        cnt >= pwd.min && cnt <= pwd.max
    }).count() as i64
}

fn p02(input: &Input) -> i64 {
    input.iter().filter(|&pwd| {
        let s = pwd.pwd.as_bytes();
        let c = pwd.c as u8;
        (s[pwd.min as usize - 1] == c) ^ (s[pwd.max as usize - 1] == c)
    }).count() as i64
}


pub fn parse(s: String) -> Day {
    let input: Vec<PasswordPolicy> = s.lines().map(|s| s.parse().unwrap()).collect();
    
    let part1 = Box::new(Part1 { input: Rc::new(input) });
    let part2 = Box::new(Part2 { input: part1.input.clone() });
    Day {
        parts: vec![part1, part2]
    }
}
