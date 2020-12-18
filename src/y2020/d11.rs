use std::rc::Rc;
use std::str::FromStr;
use crate::{Day, Part};

// The 2D seat map represented as a 1D, line continuous vector with extra padding lines and rows around the actual area
type Seats = Vec<u8>;

#[derive(Clone, Debug, Eq, PartialEq)]
struct WaitingArea {
    initial: Seats,
    w: usize,
    h: usize,
    vstep: usize,
    top_left: usize,
    bottom_right: usize,
}

fn is_seat(c: u8) -> bool {
    c == 'L' as u8 || c == '#' as u8
}

impl WaitingArea {
    fn immediate_neighbours(&self) -> (Vec<u16>, Vec<u8>) {
        fn add_neighbour(neighbours: &mut Vec<u16>, from: u16, to: u16) {
            let mut i = (from as usize) << 3;
            while neighbours[i] < u16::MAX { i += 1; }
            neighbours[i] = to;
        }

        let mut neighbours: Vec<u16> = Vec::new();
        let mut map: Vec<u8> = Vec::new();
        let mut idxs: Vec<u16> = Vec::new();
        let def_neighbours = [u16::MAX; 8];

        let mut i = self.top_left;
        while i <= self.bottom_right {
            if is_seat(self.initial[i]) {
                let idx = idxs.len() as u16;
                neighbours.extend(def_neighbours.iter());
                
                for d in [self.vstep + 1, self.vstep, self.vstep - 1, 1].iter() {
                    let j = i - d;
                    if is_seat(self.initial[j]) {
                        let j = j as u16;
                        let jdx = idxs.binary_search(&j).unwrap() as u16;
                        
                        add_neighbour(&mut neighbours, idx, jdx);
                        add_neighbour(&mut neighbours, jdx, idx);
                    }
                }

                idxs.push(i as u16);
                map.push(((self.initial[i] == '#' as u8) as u8) << 7);
            }
            i += 1;
        }

        (neighbours, map)
    }

    fn crate_seats(&self) -> Seats {
        self.initial.to_vec()
    }

    fn step2(&self, from: &Seats, to: &mut Seats, neighbours: &Vec<(usize, Vec<usize>)>) {
        for (i, js) in neighbours {
            if from[*i] == 'L' as u8 {
                if js.iter().any(|&j| from[j] == '#' as u8) {
                    to[*i] = 'L' as u8
                } else {
                    to[*i] = '#' as u8
                }
            } else if from[*i] == '#' as u8 {
                if js.iter().filter(|&&j| from[j] == '#' as u8).count() >= 5 {
                    to[*i] = 'L' as u8
                } else {
                    to[*i] = '#' as u8
                }
            }
        }
    }
}

impl FromStr for WaitingArea {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let w = s.lines().next().unwrap().len();
        let h = s.lines().count();
        let vstep = w + 2;
        let top_left = vstep + 1;
        let bottom_right = w + 1 + vstep * h;

        let mut initial = vec!['X' as u8; (w + 2) * (h + 2)];
        let mut i = top_left;
        for line in s.lines() {
            initial[i..i+w].as_mut().copy_from_slice(line.as_bytes());
            i += vstep;
        }

        Result::Ok(WaitingArea{
            initial: initial,
            w: w,
            h: h,
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

fn solve(neighbours: &Vec<u16>, map: &mut Vec<u8>, threshold: u8) -> i64 {
    let threshold = threshold + 0x80;
    loop {
        let mut unchanged = true;
        let mut i = 0;

        // Step 1: update number of occupied neighbours
        while i < map.len() {
            if map[i] & 0x80u8 == 0x80u8 {
                let mut j = i << 3;
                while neighbours[j] < u16::MAX && j < ((i + 1) << 3) {
                    map[neighbours[j] as usize] += 1;
                    j += 1;
                }
            }
            i += 1;
        }

        // Step 2: update state
        i = 0;
        while i < map.len() {
            if map[i] == 0              { map[i] = 0x80; unchanged = false }
            else if map[i] >= threshold { map[i] = 0;    unchanged = false }
            else                        { map[i] = map[i] & 0x80;          }
            i += 1;
        }

        if unchanged { return map.iter().filter(|&&x| x == 0x80).count() as i64 }
    }
}

fn p01(input: &Input) -> i64 {
    let (neighbours, mut map) = input.immediate_neighbours();
    solve(&neighbours, &mut map, 4)
}

fn p02(input: &Input) -> i64 {
    let mut neighbours: Vec<(usize, Vec<usize>)> = Vec::new();

    // Build a lookup map of all neighbours
    let mut i = input.top_left;
    while i <= input.bottom_right {
        if input.initial[i] == 'L' as u8 || input.initial[i] == '#' as u8 {
            let mut directions = Vec::new();
            for d in &[1, input.vstep + 1, input.vstep - 1, input.vstep] {
                let mut j = i + d;
                while input.initial[j] == '.' as u8 { j += d }
                if input.initial[j] != 'X' as u8 { directions.push(j) }

                j = i - d;
                while input.initial[j] == '.' as u8 { j -= d }
                if input.initial[j] != 'X' as u8 { directions.push(j) }
            }
            neighbours.push((i, directions))
        }
        i += 1
    }

    // Run the simulation
    let mut b1 = input.crate_seats();
    let mut b2 = input.crate_seats();
    loop {
        input.step2(&b1, &mut b2, &neighbours);
        if b1 == b2 { return b2.iter().filter(|&&c| c == '#' as u8).count() as i64 }
        input.step2(&b2, &mut b1, &neighbours);
        if b1 == b2 { return b1.iter().filter(|&&c| c == '#' as u8).count() as i64 }
    }
}

pub fn parse(s: String) -> Day {
    let input: WaitingArea = s.parse().unwrap();

    let part1 = Box::new(Part1 { input: Rc::new(input) });
    let part2 = Box::new(Part2 { input: part1.input.clone() });
    Day {
        parts: vec![part1, part2]
    }
}
