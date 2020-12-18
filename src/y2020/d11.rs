use std::rc::Rc;
use std::str::FromStr;
use crate::{Day, Part};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Field {
    Empty,
    Padding,
    FreeSeat(u8),
    OccupiedSeat(u8)
}

// The 2D seat map represented as a 1D, line continuous vector with extra padding lines and rows around the actual area
//
// For example this map:
//
// .L..#
// ##..L
//
// Becomes this vector:
// 
// XXXXXX.L..#X##..LXXXXXX
//
// Where 'X' is the padding field.
type Seats = Vec<Field>;

#[derive(Clone, Debug, Eq, PartialEq)]
struct WaitingArea {
    initial: Seats,
    vstep: usize,
    top_left: usize,
    bottom_right: usize,
}

impl FromStr for WaitingArea {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let w = s.lines().next().unwrap().len();
        let h = s.lines().count();
        let vstep = w + 1;
        let top_left = vstep + 1;
        let bottom_right = w + 1 + vstep * h;

        let mut initial = vec![Field::Padding; (w + 1) * (h + 2) + 1];
        let mut i = top_left;
        for line in s.lines() {
            for &c in line.as_bytes() {
                let field = if c == '.' as u8 { Field::Empty }
                else if c == 'L' as u8 { Field::FreeSeat(0) }
                else { Field::OccupiedSeat(0) };
                initial[i] = field;
                i += 1;
            }
            i += 1;
        }

        Result::Ok(WaitingArea{
            initial: initial,
            vstep: vstep,
            top_left: top_left,
            bottom_right: bottom_right,
        })
    }
}

type Input = Rc<WaitingArea>;

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

fn step1(from: &mut Seats, to: &mut Seats, top_left: usize, bottom_right: usize, vstep: usize) -> Option<i64> {
    // In the first buffer: update state
    // In the second buffer: calculate number of occupied neighbours
    let neighbours = [vstep + 1, vstep, vstep - 1, 1];
    let mut unchanged = true;
    let mut occupied_cnt = 0;
    let mut i = top_left;
    while i <= bottom_right {
        match from[i] {
            Field::OccupiedSeat(m) => if m < 4 {
                let cnt = neighbours.iter().filter(|d| {
                    let j = i - **d;
                    match to[j] {
                        Field::OccupiedSeat(n) => {
                            to[j] = Field::OccupiedSeat(n + 1);
                            true
                        },
                        Field::FreeSeat(n) => {
                            to[j] = Field::FreeSeat(n + 1);
                            false
                        },
                        _ => false
                    }
                }).count();
                to[i] = Field::OccupiedSeat(cnt as u8);
                occupied_cnt += 1;
            } else {
                let cnt = neighbours.iter().filter(|d| {
                    let j = i - **d;
                    match to[j] {
                        Field::OccupiedSeat(_) => {
                            true
                        },
                        _ => false
                    }
                }).count();
                to[i] = Field::FreeSeat(cnt as u8);
                unchanged = false;
            },
            Field::FreeSeat(m) => if m > 0 {
                let cnt = neighbours.iter().filter(|d| {
                    let j = i - **d;
                    match to[j] {
                        Field::OccupiedSeat(_) => {
                            true
                        },
                        _ => false
                    }
                }).count();
                to[i] = Field::FreeSeat(cnt as u8)
            } else {
                let cnt = neighbours.iter().filter(|d| {
                    let j = i - **d;
                    match to[j] {
                        Field::OccupiedSeat(n) => {
                            to[j] = Field::OccupiedSeat(n + 1);
                            true
                        },
                        Field::FreeSeat(n) => {
                            to[j] = Field::FreeSeat(n + 1);
                            false
                        },
                        _ => false
                    }
                }).count();
                to[i] = Field::OccupiedSeat(cnt as u8);
                occupied_cnt += 1;
                unchanged = false;
            },
            _ => ()
        }
        i += 1;
    }

    if unchanged {
        Option::Some(occupied_cnt)
    } else {
        Option::None
    }
}

fn p01(input: &Input) -> i64 {
    let mut b1 = input.initial.clone();
    let mut b2 = b1.clone();

    loop {
        match step1(&mut b1, &mut b2, input.top_left, input.bottom_right, input.vstep) {
            Option::Some(occupied) => return occupied,
            Option::None => ()
        }
        match step1(&mut b2, &mut b1, input.top_left, input.bottom_right, input.vstep) {
            Option::Some(occupied) => return occupied,
            Option::None => ()
        }
    }
}

fn p02(_input: &Input) -> i64 {
    0
}

pub fn parse(s: String) -> Day {
    let input: WaitingArea = s.parse().unwrap();

    let part1 = Box::new(Part1 { input: Rc::new(input) });
    let part2 = Box::new(Part2 { input: part1.input.clone() });
    Day {
        parts: vec![part1, part2]
    }
}
