use std::rc::Rc;
use crate::{Day, Part};

#[derive(Eq, PartialEq, Debug, Clone)]
enum BusLine {
    X,
    Id(u32)
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Schedule {
    t: u32,
    services: Vec<BusLine>
}

type Input = Rc<Schedule>;

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
    let mut min_wait = u32::MAX;
    let mut min_wait_id = 0;
    for service in &input.services {
        match service {
            BusLine::Id(id) => {
                let miss = input.t % id;
                if miss == 0 { return 0 }
                
                let wait = id - miss;
                if wait < min_wait {
                    min_wait = wait;
                    min_wait_id = *id;
                }
            },
            _ => ()
        }
    }
    
    min_wait_id as i64 * min_wait as i64
}

fn p02(input: &Input) -> i64 {
    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }
    
    let mut t = 0i64;
    let mut step = match input.services[0] {
        BusLine::Id(id) => id as i64,
        BusLine::X => -1i64 // invalid input
    };
    
    let mut i = 1;
    while i < input.services.len() {
        match input.services[i] {
            BusLine::Id(id) => {
                let id = id as i64;
                let idx = i as i64;
                
                // t + step*X = id*Y - idx
                // step*X + (t + idx ) = id*Y'
                let a = step % id;
                let mut b = (t + idx) % id;
                while b != 0 {
                    t += step;
                    b += a;
                    if b >= id { b -= id }
                }
                
                step *= id / gcd(step, id);
            },
            BusLine::X => ()
        }
        i += 1;
    }
    
    t
}

pub fn parse(s: String) -> Day {
    let mut lines = s.lines();
    let t = lines.next().unwrap().parse().unwrap();
    let services = lines.next().unwrap().split(',').map(|token| if token == "x" { BusLine::X } else { BusLine::Id(token.parse().unwrap()) }).collect();
    let input = Schedule {
        t: t,
        services: services
    };
    
    let part1 = Box::new(Part1 { input: Rc::new(input) });
    let part2 = Box::new(Part2 { input: part1.input.clone() });
    Day {
        parts: vec![part1, part2]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ex1() {
        let d = super::parse("939\n7,13,x,x,59,x,31,19".to_string());
        assert_eq!(d.parts[0].solve(), 295);
        assert_eq!(d.parts[1].solve(), 1068781);
    }
}