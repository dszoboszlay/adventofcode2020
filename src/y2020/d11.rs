use std::rc::Rc;
use std::str::FromStr;
use std::time::Instant;
use crate::{Day, Part};

// The 2D seat map represented as a 1D, line continuous vector with extra padding lines and rows around the actual area
type Seats = Vec<u8>;

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

    fn solve(&self, neighbours: &mut Vec<Option<Seat>>, threshold: usize) -> i64 {
        let t = Instant::now();
        let mut buff_a: Vec<bool> = self.initial.iter().filter_map(|&c| {
            if c == 'L' as u8 { Option::Some(false)
            } else if c == '#' as u8 { Option::Some(true)
            } else { Option::None }
        }).collect();
        let mut buff_b = buff_a.clone();
        let mut stable_occupied = 0i64;
        println!("setup      in {:14.3} μs", t.elapsed().as_nanos() as f32 / 1000.0);
        
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

fn p01(input: &Input) -> i64 {
    let t = Instant::now();
    let mut x = input.immediate_neighbours();
    println!("neighbours in {:14.3} μs", t.elapsed().as_nanos() as f32 / 1000.0);
    let t = Instant::now();
    let res = input.solve(&mut x, 4);
    println!("solved     in {:14.3} μs", t.elapsed().as_nanos() as f32 / 1000.0);
    res
}

fn p02(input: &Input) -> i64 {
    let t = Instant::now();
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

    println!("neighbours in {:14.3} μs", t.elapsed().as_nanos() as f32 / 1000.0);

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
