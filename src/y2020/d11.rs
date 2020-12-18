use std::rc::Rc;
use std::str::FromStr;
use crate::{Day, Part};

type Index = u16;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Field {
    Empty,
    Padding,
    Seat(Index) // index of the field in the occupancy map
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
// XXXXXX.S..SXSS..SXXXXXX
//
// Where 'X' is the padding field and S means a seat (free or occupied).
type Seats = Vec<Field>;

type Occupancy = Vec<bool>;

#[derive(Clone, Debug, Eq, PartialEq)]
struct WaitingArea {
    map: Seats,
    initial: Occupancy,
    initial_occupied_cnt: i64,
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
        
        let mut map = vec![Field::Padding; (w + 1) * (h + 2) + 1];
        let mut initial = Vec::new();
        let mut initial_occupied_cnt = 0;
        let mut i = top_left;
        for line in s.lines() {
            for &c in line.as_bytes() {
                if c == '.' as u8 {
                    map[i] = Field::Empty;
                } else {
                    map[i] = Field::Seat(initial.len() as Index);
                    if c == '#' as u8 {
                        initial.push(true);
                        initial_occupied_cnt += 1;
                    } else {
                        initial.push(false);
                    }
                }
                i += 1;
            }
            i += 1;
        }
        
        Result::Ok(WaitingArea{
            map: map,
            initial: initial,
            initial_occupied_cnt: initial_occupied_cnt,
            vstep: vstep,
            top_left: top_left,
            bottom_right: bottom_right,
        })
    }
}

impl WaitingArea {
    fn solve(&self, neighbours: &Vec<Index>, threshold: usize) -> i64 {
        let mut buff1 = self.initial.clone();
        let mut buff2 = self.initial.clone();
        let mut to_update1 = vec![false; buff1.len()];
        let mut to_update2 = to_update1.clone();
        let mut occupied_cnt = self.initial_occupied_cnt;

        fn update_field(i: usize, neighbours: &Vec<Index>, threshold: usize, from: &Vec<bool>, to: &mut Vec<bool>, to_update: &mut Vec<bool>, occupied_cnt: &mut i64) {
            let start = i * 8;
            let end = start + 8;
            let mut ii = start;
            let mut cnt = 0;
            while ii < end && neighbours[ii] < Index::MAX {
                if from[neighbours[ii] as usize] { cnt += 1; }
                ii += 1;
            }
            if (from[i] && cnt >= threshold) || (!from[i] && cnt == 0) {
                if from[i] {
                    to[i] = false;
                    *occupied_cnt -= 1;
                } else {
                    to[i] = true;
                    *occupied_cnt += 1;
                }
                while ii > start {
                    ii -= 1;
                    to_update[neighbours[ii] as usize] = true;
                }
            } else {
                to[i] = from[i];
            }
        }
        
        // Round 0: update all the seats
        let mut i = 0;
        while i < self.initial.len() {
            update_field(i, &neighbours, threshold, &buff1, &mut buff2, &mut to_update1, &mut occupied_cnt);
            i += 1;
        }
        
        // Regular rounds: update only the neighbours of changed seats
        loop {
            let mut stable = true;
            i = 0;
            while i < to_update1.len() {
                if to_update1[i] {
                    stable = false;
                    to_update1[i] = false;
                    update_field(i, &neighbours, threshold, &buff2, &mut buff1, &mut to_update2, &mut occupied_cnt)
                }
                i += 1;
            }
            if stable { return occupied_cnt; }

            stable = true;
            i = 0;
            while i < to_update2.len() {
                if to_update2[i] {
                    stable = false;
                    to_update2[i] = false;
                    update_field(i, &neighbours, threshold, &buff1, &mut buff2, &mut to_update1, &mut occupied_cnt)
                }
                i += 1;
            }
            if stable { return occupied_cnt; }
            for &i in to_update1.iter() {
                update_field(i as usize, &neighbours, threshold, &buff2, &mut buff1, &mut to_update2, &mut occupied_cnt)
            }
        }
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

fn store_neighbour(neighbours: &mut Vec<Index>, i: Index, n: Index) {
    let mut offset = (i * 8) as usize;
    while neighbours[offset] != Index::MAX { offset += 1; }
    neighbours[offset] = n;
}

fn p01(input: &Input) -> i64 {
    let offsets = [input.vstep + 1, input.vstep, input.vstep - 1, 1];
    let mut neighbours: Vec<Index> = vec![Index::MAX; input.initial.len() * 8];
    let mut i = input.top_left;
    while i <= input.bottom_right {
        match input.map[i] {
            Field::Seat(n) => for offset in offsets.iter() {
                match input.map[i + offset] {
                    Field::Seat(m) => {
                        store_neighbour(&mut neighbours, n, m);
                        store_neighbour(&mut neighbours, m, n);
                    },
                    _ => ()
                }
            },
            _ => ()
        }
        i += 1;
    }
    input.solve(&neighbours, 4)
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
