use std::fs;
use std::path::Path;
use std::time::Instant;

pub struct Year {
    year: u16,
    day_parsers: Vec<fn(String) -> Day>,
}

pub struct Day {
    parts: Vec<Box<dyn Part>>,
}

pub trait Part {
    fn solve(&self) -> i64;
}

fn solutions(from: &Path) -> Vec<Vec<i64>> {
    let mut res: Vec<Vec<i64>> = Vec::new();
    if from.is_file() {
        for line in fs::read_to_string(from).unwrap().lines() {
            res.push(line.split_whitespace().map(|x| x.parse().unwrap()).collect())
        }
    }
    res
}

mod y2020;

fn run_year(year: &Year, dir: &str) {
    let dir = Path::new(dir);
    let sols = solutions(&dir.join("solutions.txt"));
    for (d, (day_parser, day_sols)) in year.day_parsers.iter().zip(sols.iter()).enumerate() {
        let d = d + 1;
        let input = dir.join(format!("{:02}.txt", d));
        if input.is_file() {
            let input_str = fs::read_to_string(input).unwrap();
            let t = Instant::now();
            let day = day_parser(input_str);
            let t = t.elapsed().as_nanos();
            println!("\x1b[33m      y{:04} d{:02} {:18} {:14.3} μs\x1b[0m", year.year, d, "parsing", t as f32 / 1000.0);
            for (p, (part, expected)) in day.parts.iter().zip(day_sols.iter()).enumerate() {
                let p = p + 1;
                let t = Instant::now();
                let actual = part.solve();
                let t = t.elapsed().as_nanos();
                let result = if *expected == actual { "[\x1b[32mOK\x1b[0m] " } else { "\x1b[31m[ERR]" };
                println!("{} y{:04} d{:02} p{:02} {:14} {:14.3} μs\x1b[0m", result, year.year, d, p, actual, t as f32 / 1000.0)
            }
        } else {
            println!("{:?} not found", &input)
        }
    }
}

fn main() {
    run_year(&y2020::year(), "input");
}
