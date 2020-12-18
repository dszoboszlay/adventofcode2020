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

#[derive(Debug, Clone, Eq, PartialEq)]
struct Seat {
    offset: usize,
    stable_occupied_neighbours_cnt: usize,
    unstable_neighbours: Vec<usize>
}

impl Seat {
    fn add_neighbour(&mut self, n: usize) {
        self.unstable_neighbours.push(n);
    }

    fn stabilise_neighbour(&mut self, n: usize, occupied: bool) {
        if occupied { self.stable_occupied_neighbours_cnt += 1 }
        self.unstable_neighbours.remove(self.unstable_neighbours.binary_search(&n).unwrap());
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WaitingArea {
    initial: Seats,
    vstep: usize,
    top_left: usize,
    bottom_right: usize,
}

fn is_seat(c: u8) -> bool {
    c == 'L' as u8 || c == '#' as u8
}

impl WaitingArea {
    fn immediate_neighbours(&self) -> Vec<Option<Seat>> {
        let mut neighbours: Vec<Option<Seat>> = Vec::new();

        let mut i = self.top_left;
        while i <= self.bottom_right {
            if is_seat(self.initial[i]) {
                let idx = neighbours.len();
                
                let mut unstable_neighbours: Vec<usize> = Vec::new();
                for d in &[self.vstep + 1, self.vstep, self.vstep - 1, 1] {
                    let j = i - d;
                    if is_seat(self.initial[j]) {
                        let j = neighbours.binary_search_by_key(&j, |s| s.as_ref().unwrap().offset).unwrap();
                        neighbours[j].as_mut().unwrap().add_neighbour(idx);
                        unstable_neighbours.push(j);
                    }
                }

                neighbours.push(Option::Some(Seat {
                    offset: i,
                    stable_occupied_neighbours_cnt: 0,
                    unstable_neighbours: unstable_neighbours
                }))
            }
            i += 1;
        }
        neighbours
    }

    fn extended_neighbours(&self) -> Vec<Option<Seat>> {
        let mut neighbours: Vec<Option<Seat>> = Vec::new();

        let mut i = self.top_left;
        while i <= self.bottom_right {
            if is_seat(self.initial[i]) {
                let idx = neighbours.len();
                
                let mut unstable_neighbours: Vec<usize> = Vec::new();
                for d in &[self.vstep + 1, self.vstep, self.vstep - 1, 1] {
                    let mut j = i - d;
                    while self.initial[j] == '.' as u8 { j -= d; }
                    if is_seat(self.initial[j]) {
                        let j = neighbours.binary_search_by_key(&j, |s| s.as_ref().unwrap().offset).unwrap();
                        neighbours[j].as_mut().unwrap().add_neighbour(idx);
                        unstable_neighbours.push(j);
                    }
                }

                unstable_neighbours.sort();
                neighbours.push(Option::Some(Seat {
                    offset: i,
                    stable_occupied_neighbours_cnt: 0,
                    unstable_neighbours: unstable_neighbours
                }))
            }
            i += 1;
        }
        neighbours
    }

    fn solve(&self, neighbours: &mut Vec<Option<Seat>>, threshold: usize) -> i64 {
        let mut buff_a: Vec<bool> = self.initial.iter().filter_map(|&c| {
            if c == 'L' as u8 { Option::Some(false)
            } else if c == '#' as u8 { Option::Some(true)
            } else { Option::None }
        }).collect();
        let mut buff_b = buff_a.clone();
        let mut stable_occupied = 0i64;
        
        fn step(neighbours: &mut Vec<Option<Seat>>, curr: &Vec<bool>, next: &mut Vec<bool>, stable_occupied: &mut i64, threshold: usize) -> Option<i64> {
            let mut changed = false;
            let mut occupied = 0i64;
            let mut i = 0usize;
            while i < curr.len() {
                match neighbours.get_mut(i).unwrap() {
                    Option::Some(n) => if curr[i] {
                        if n.stable_occupied_neighbours_cnt + n.unstable_neighbours.len() < threshold {
                            // Stays occupied and stabilises
                            let js = n.unstable_neighbours.clone();
                            for j in js { neighbours[j].as_mut().unwrap().stabilise_neighbour(i, true) }
                            neighbours[i] = Option::None;
                            *stable_occupied += 1;
                        } else if n.stable_occupied_neighbours_cnt + n.unstable_neighbours.iter().filter(|&&j| { curr[j] }).count() < threshold {
                            // Stay occupied
                            occupied += 1;
                            next[i] = true;
                        } else {
                            // Changes to free
                            next[i] = false;
                            changed = true;
                        }
                    } else {
                        if n.stable_occupied_neighbours_cnt > 0 {
                            // Stays free and stabilises
                            let js = n.unstable_neighbours.clone();
                            for j in js { neighbours[j].as_mut().unwrap().stabilise_neighbour(i, false) }
                            neighbours[i] = Option::None;
                        } else if n.unstable_neighbours.iter().any(|&j| curr[j]) {
                            // Stays free
                            next[i] = false;
                        } else {
                            // Changes to occupied
                            next[i] = true;
                            changed = true;
                        }
                    },
                    Option::None => ()
                }
                i = i + 1;
            }

            if changed { Option::None } else { Option::Some(occupied + *stable_occupied) }
        }

        loop {
            match step(&mut *neighbours, &buff_a, &mut buff_b, &mut stable_occupied, threshold) {
                Option::Some(occupied) => return occupied,
                Option::None => ()
            }
            match step(&mut *neighbours, &buff_b, &mut buff_a, &mut stable_occupied, threshold) {
                Option::Some(occupied) => return occupied,
                Option::None => ()
            }
        }
    }
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

fn p01(input: &Input) -> i64 {
    input.solve(&mut input.immediate_neighbours(), 4)
}

fn p02(input: &Input) -> i64 {
    input.solve(&mut input.extended_neighbours(), 5)
}

pub fn parse(s: String) -> Day {
    let input: WaitingArea = s.parse().unwrap();
    
    let part1 = Box::new(Part1 { input: Rc::new(input) });
    let part2 = Box::new(Part2 { input: part1.input.clone() });
    Day {
        parts: vec![part1, part2]
    }
}
