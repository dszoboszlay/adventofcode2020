use std::rc::Rc;
use std::str::FromStr;
use crate::{Day, Part};

#[derive(Eq, PartialEq, Debug, Clone)]
enum Action {
    North,
    East,
    South,
    West,
    Left,
    Right,
    Forward,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "N" { Result::Ok(Action::North)
        } else if s == "E" { Result::Ok(Action::East)
        } else if s == "S" { Result::Ok(Action::South)
        } else if s == "W" { Result::Ok(Action::West)
        } else if s == "L" { Result::Ok(Action::Left)
        } else if s == "R" { Result::Ok(Action::Right)
        } else if s == "F" { Result::Ok(Action::Forward)
        } else { Result::Err(()) }
    }
}

type Instruction = (Action, i32);

type Input = Rc<Vec<Instruction>>;

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
    let mut x = 0i32;
    let mut y = 0i32;
    let mut d = 0usize;
    let forward_to_action = [Action::East, Action::South, Action::West, Action::North];

    for (action, val) in input.iter() {
        let action = if *action == Action::Forward { &forward_to_action[d] } else { action };
        match action {
            Action::East => x += val,
            Action::South => y += val,
            Action::West => x -= val,
            Action::North => y -= val,
            Action::Left => d = (d + *val as usize / 30) % 4,
            Action::Right => d = (d + *val as usize / 90) % 4,
            _ => () // cannot happen
        }
    }

    (x.abs() + y.abs()) as i64
}

fn p02(input: &Input) -> i64 {
    let mut wx = 10;
    let mut wy = -1;
    let mut sx = 0;
    let mut sy = 0;

    fn  left(x: &mut i32, y: &mut i32) { let t = *x; *x = *y; *y = -t }
    fn right(x: &mut i32, y: &mut i32) { let t = *x; *x = -*y; *y = t }
    fn  back(x: &mut i32, y: &mut i32) { *x = -*x; *y = -*y }

    for (action, val) in input.iter() {
        match action {
            Action::East => wx += val,
            Action::South => wy += val,
            Action::West => wx -= val,
            Action::North => wy -= val,
            Action::Forward => { sx += wx * val; sy += wy * val },
            Action::Left => if *val == 90 { left(&mut wx, &mut wy)
            } else if *val == 180 { back(&mut wx, &mut wy)
            } else { right(&mut wx, &mut wy) },
            Action::Right => if *val == 90 { right(&mut wx, &mut wy)
            } else if *val == 180 { back(&mut wx, &mut wy)
            } else { left(&mut wx, &mut wy) },
        }
    }

    (sx.abs() + sy.abs()) as i64
}

pub fn parse(s: String) -> Day {
    let input = s.lines().map(|l| {
        (l[..1].parse::<Action>().unwrap(), l[1..].parse::<i32>().unwrap())
    }).collect();

    let part1 = Box::new(Part1 { input: Rc::new(input) });
    let part2 = Box::new(Part2 { input: part1.input.clone() });
    Day {
        parts: vec![part1, part2]
    }
}
