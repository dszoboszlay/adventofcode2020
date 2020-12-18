use std::rc::Rc;
use std::collections::HashMap;
use crate::{Day, Part};

type Colour = String;
type ContainedBags = HashMap<Colour, u32>;
type Bags = HashMap<Colour, ContainedBags>;
type Input = Rc<Bags>;

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
    let my_colour = "shiny gold".to_string();
    let mut containers: HashMap<&Colour, Vec<&Colour>> = HashMap::new();
    let mut to_search: Vec<&Colour> = vec![&my_colour];
    let mut cnt = 0;
    
    // Build inverse lookup table
    for (container, contained_bags) in input.iter() {
        for contained in contained_bags.keys() {
            match containers.get_mut(contained) {
                Option::Some(v) => v.push(container),
                Option::None => { containers.insert(contained, vec![container]); }
            }
        }
    }

    // Interpretation of values in containers:
    // - none: a new possible outermost colour without any possible enclosing containers
    // - nonempty list: a new possible outermost colour with new enclosing containers to check
    // - empty list: an already visited outermost colour
    loop {
        match to_search.pop() {
            Option::None => return (cnt - 1) as i64,
            Option::Some(colour) => match containers.get_mut(colour) {
                Option::Some(v) => if !v.is_empty() {
                    // Found a new possible outermost colour
                    cnt += 1;
                    to_search.append(v);
                },
                Option::None => {
                    cnt += 1;
                    containers.insert(colour, Vec::new());
                }
            }
        }
    }
}

fn p02(input: &Input) -> i64 {
    fn contained_bags<'a>(colour: &'a Colour, input: &'a Input, cache: &mut HashMap<&'a Colour, u32>) -> u32 {
        match cache.get(colour) {
            Option::Some(n) => *n,
            Option::None => {
                let n = input.get(colour).unwrap().iter().map(|(c, cnt)| {
                    cnt * (contained_bags(c, input, cache) + 1)
                }).sum();
                cache.insert(colour, n);
                n
            }
        }
    }

    let my_colour = "shiny gold".to_string();
    let mut cache: HashMap<&Colour, u32> = HashMap::new();
    contained_bags(&my_colour, input, &mut cache) as i64
}

fn parse_contained_bags(line: &str) -> ContainedBags {
    let mut bags: ContainedBags = HashMap::new();

    // (<N> " " <B> " bag" ("s" | "") ", ")* <N> " " <B> " bag" ("s" | "") "."
    for s in line.split(", ") {
        let p = s.find(' ').unwrap();
        let n: u32 = s[..p].parse().unwrap();
        let s = &s[p + 1..];

        let p = s.find(" bag").unwrap();
        let colour = s[..p].to_string();

        bags.insert(colour, n);
    }

    bags
}

pub fn parse(s: String) -> Day {
    let mut input: Bags = HashMap::new();
    for line in s.lines() {
        // <B> " bags contain " ( "no other bags." | <parse_contained_bags> )
        let pat_bags_contain = " bags contain ";
        let p = line.find(pat_bags_contain).unwrap();
        let key = line[..p].to_string();

        let line = &line[p + pat_bags_contain.len()..];
        if line == "no other bags." {
            input.insert(key, HashMap::new());
        } else {
            input.insert(key, parse_contained_bags(line));
        }
    }

    let part1 = Box::new(Part1 { input: Rc::new(input) });
    let part2 = Box::new(Part2 { input: part1.input.clone() });
    Day {
        parts: vec![part1, part2]
    }
}
