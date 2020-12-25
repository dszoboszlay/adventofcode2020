use std::rc::Rc;
use std::str::FromStr;
use crate::{Day, Part};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Content {
    Children(u8, Box<Node>, Box<Node>),
    Value(u64),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    addr: u64,
    floats: u64,
    content: Content
}

impl Node {
    fn new() -> Node {
        Node {
            addr: 0,
            floats: 0,
            content: Content::Value(0)
        }
    }
    
    fn insert(&mut self, addr: u64, floats: u64, value: u64) {
        fn rec_insert(bits: u8, child0: &mut Node, child1: &mut Node, addr: u64, floats: u64, value: u64) {
            let mask = 1 << (bits - 1);
            let rec_addr = addr >> bits;
            let rec_floats = floats >> bits;
            if floats & mask == 0 {
                if addr & mask == 0 {
                    child0.insert(rec_addr, rec_floats, value)
                } else {
                    child1.insert(rec_addr, rec_floats, value)
                }
            } else {
                child0.insert(rec_addr, rec_floats, value);
                child1.insert(rec_addr, rec_floats, value);
            }
        }

        if Content::Value(0) == self.content {
            // There's no value in storing information about 0-s, they don't affect the sum.
            // So let's simply overwrite this node with the new info!
            self.addr = addr;
            self.floats = floats;
            self.content = Content::Value(value);

            return
        }
        
        let cmp = (self.addr ^ addr) | (self.floats ^ floats);
        if cmp == 0 {
            // Perfect match on this layer of the tree
            match &mut self.content {
                Content::Value(v) => *v = value,
                Content::Children(bits, child0, child1) => rec_insert(*bits, child0, child1, addr, floats, value)
            }
        } else {
            // Find common bits
            let mut new_bits = 1;
            let mut mask = 1;
            while cmp & mask == 0 {
                mask = mask << 1;
                new_bits += 1;
            }
            
            match &mut self.content {
                Content::Value(v) => {
                    let mut child = Box::new(Node {
                        addr: self.addr >> new_bits,
                        floats: self.floats >> new_bits,
                        content: Content::Value(*v) 
                    });

                    if self.floats & mask > 0 {
                        let mut other_child = child.clone();
                        rec_insert(new_bits, &mut child, &mut other_child, addr, floats, value);
                        self.content = Content::Children(new_bits, child, other_child)
                    } else {
                        let mut other_child = Box::new(Node::new());
                        if self.addr & mask > 0 {
                            rec_insert(new_bits, &mut other_child, &mut child, addr, floats, value);
                            self.content = Content::Children(new_bits, other_child, child)
                        } else {
                            rec_insert(new_bits, &mut child, &mut other_child, addr, floats, value);
                            self.content = Content::Children(new_bits, child, other_child)
                        }
                    }
                },
                Content::Children(bits, child0, child1) if *bits <= new_bits => rec_insert(*bits, child0, child1, addr, floats, value),
                Content::Children(bits, child0, child1) => {
                    let child = Box::new(Node {
                        addr: self.addr >> new_bits,
                        floats: self.floats >> new_bits,
                        content: Content::Children(
                            *bits - new_bits,
                            Box::new(*child0.clone()),
                            Box::new(*child1.clone())
                        )
                    });
                    
                    if self.floats & mask > 0 {
                        *child0 = child;
                        *child1 = child0.clone();
                    } else {
                        let other_child = Box::new(Node::new());
                        if self.addr & mask > 0 {
                            *child1 = child;
                            *child0 = other_child;
                        } else {
                            *child0 = child;
                            *child1 = other_child;
                        }
                    }
                    *bits = new_bits;
                    rec_insert(new_bits, child0, child1, addr, floats, value)
                }
            }

            let mask = mask - 1;
            self.addr = self.addr & mask;
            self.floats = self.floats & mask;
        }
    }
    
    fn sum(&self) -> u64 {
        let val = match &self.content {
            Content::Value(v) => *v,
            Content::Children(_, c0, c1) => c0.sum() + c1.sum()
        };
        
        let mul = self.floats.count_ones();
        val << mul
    }
}

enum Op {
    Mask(u64, u64), // and, or
    Mem(u64, u64)   // addr, val
}

type Ops = Vec<Op>;

impl FromStr for Op {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.as_bytes()[5] == '=' as u8 {
            // Mask
            let mut and = 0u64;
            let mut or = 0u64;
            for c in s[7..].bytes() {
                and <<= 1;
                or <<= 1;
                if c == '1' as u8 {
                    or |= 1;
                    and |= 1;
                } else if c == 'X' as u8 {
                    and |= 1;
                }
            }
            
            Result::Ok(Op::Mask(and, or))
        } else {
            // Mem
            let end_of_addr = s.find(']').unwrap();
            let addr: u64 = s[4..end_of_addr].parse().unwrap();
            let val: u64 = s[end_of_addr+4..].parse().unwrap();
            
            Result::Ok(Op::Mem(addr, val))
        }
    }
}

type Input = Rc<Ops>;

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
    let mut mem = Node::new();
    let mut and = 0u64;
    let mut or = 0u64;
    
    for op in input.iter() {
        match op {
            Op::Mask(a, o) => {
                and = *a;
                or = *o;
            },
            Op::Mem(addr, value) => {
                mem.insert(*addr, 0, (*value | or) & and);
            }
        }
    }
    
    mem.sum() as i64
}

fn p02(input: &Input) -> i64 {
    let mut mem = Node::new();
    let mut sets = 0u64;
    let mut floats = 0u64;
    
    for op in input.iter() {
        match op {
            Op::Mask(and, or) => {
                // Need to transcode and + or fields to sets + floats
                //
                // and | or | original | set | float
                // ----+----+----------+-----+------
                //   0 |  0 |        0 | 0   | 0 
                //   1 |  0 |        X | 1   | 1
                //   1 |  1 |        1 | 1   | 0
                //
                // sets = and | or
                // floats = and & not or
                //
                // Note: when a bit floats, we set it to 1 in the address. This is an arbitrary choice,
                // but guarantees that two identical addresses with floating bits would always be equal,
                // no matter how they were specified in the input.
                sets = and | or;
                floats = and & !or;
            },
            Op::Mem(addr, value) => {
                let addr = addr | sets;
                mem.insert(addr, floats, *value)
            }
        }
    }
    
    mem.sum() as i64
}

pub fn parse(s: String) -> Day {
    let input: Ops = s.lines().map(|l| l.parse().unwrap()).collect();
    
    let part1 = Box::new(Part1 { input: Rc::new(input) });
    let part2 = Box::new(Part2 { input: part1.input.clone() });
    Day {
        parts: vec![part1, part2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p01_test() {
        let d = parse(
            String::from("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
                          mem[8] = 11\n\
                          mem[7] = 101\n\
                          mem[8] = 0\n"));
        assert_eq!(d.parts[0].solve(), 165);
    }

    #[test]
    fn p02_test() {
        let d = parse(
            String::from("mask = 000000000000000000000000000000X1001X\n\
                          mem[42] = 100\n\
                          mask = 00000000000000000000000000000000X0XX\n\
                          mem[26] = 1\n"));
        assert_eq!(d.parts[1].solve(), 208);
    }

    #[test]
    fn insert_to_content() {
        let mut n0 = Node {addr: 4, floats: 0, content: Content::Value(10)};
        n0.insert(2, 0, 11);
        assert_eq!(
            n0,
            Node {addr: 0, floats: 0, content:
                Content::Children(2,
                    Box::new(Node {addr: 1, floats: 0, content: Content::Value(10)}),
                    Box::new(Node {addr: 0, floats: 0, content: Content::Value(11)})
                )});
    }
}