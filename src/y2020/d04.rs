use std::rc::Rc;
use std::collections::HashMap;
use crate::{Day, Part};

#[derive(Debug, PartialEq)]
struct Passport {
    fields: HashMap<String, String>
}

impl Passport {
    fn is_valid(&self) -> bool {
        let req_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        req_fields.iter().all(|k| self.fields.contains_key(&k.to_string()))
    }
    
    fn is_strict_valid(&self) -> bool {
        let is_field = |name: &str, test: &dyn Fn(&str) -> bool|{
            match self.fields.get(name) {
                Option::Some(s) => test(s),
                Option::None => false
            }
        };
        let is_int_field = |name, min, max| {
            is_field(name, &|s| match s.parse::<u16>() {
                Result::Ok(n) => n >= min && n <= max,
                _ => false
            })
        };
        
        is_int_field("byr", 1920, 2002) &&
        is_int_field("iyr", 2010, 2020) &&
        is_int_field("eyr", 2020, 2030) &&
        is_field("hgt", &|s| {
            s.len() >= 4 &&
            match s[..s.len() - 2].parse::<u8>() {
                Result::Ok(h) => {
                    let unit = &s[s.len() - 2..];
                    if unit == "in" {
                        h >= 59 && h <= 76
                    } else {
                        unit == "cm" && h >= 150 && h <= 193
                    }
                },
                _ => false
            }
        }) &&
        is_field("hcl", &|s| {
            s.len() == 7 &&
            s.chars().next().unwrap() == '#' &&
            s[1..].chars().all(|c| (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f'))
        }) &&
        is_field("ecl", &|s| {
            s == "amb" ||
            s == "blu" ||
            s == "brn" ||
            s == "gry" ||
            s == "grn" ||
            s == "hzl" ||
            s == "oth"
        }) &&
        is_field("pid", &|s| {
            s.len() == 9 &&
            s.chars().all(|c| c >= '0' && c <= '9')
        }) 
    }
}

type Input = Rc<Vec<Passport>>;

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
    input.iter().filter(|p| p.is_valid()).count() as i64
}

fn p02(input: &Input) -> i64 {
    input.iter().filter(|p| p.is_strict_valid()).count() as i64
}

pub fn parse(s: String) -> Day {
    let mut input: Vec<Passport> = Vec::new();
    let mut fields: HashMap<String, String> = HashMap::new();
    for line in s.lines() {
        if line.len() == 0 {
            input.push(Passport{fields: fields});
            fields = HashMap::new();
        } else {
            for token in line.split_whitespace() {
                let i = token.find(':').unwrap();
                fields.insert(token[..i].to_string(), token[(i + 1)..].to_string());
            }
        }
    }
    input.push(Passport{fields: fields});
    
    let part1 = Box::new(Part1 { input: Rc::new(input) });
    let part2 = Box::new(Part2 { input: part1.input.clone() });
    Day {
        parts: vec![part1, part2]
    }
}
