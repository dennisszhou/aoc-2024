use std::fs::File;
use std::io::{self, BufRead};

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<std::path::Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse(input: &std::path::Path) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    if let Ok(lines) = read_lines(input) {
        lines.for_each(|l| {
            let s = l.unwrap();

            reports.push(s.split(" ").map(|i| -> i32{i.parse().unwrap()}).collect())
        });
    }

    return reports;
}

fn part1(r: &Vec<i32>) -> bool {
    let deltas: Vec<i32> = r.windows(2).map(|w| { w[0] - w[1] }).collect();

    return deltas.iter().all(|&i| { i > 0 && i <= 3 }) ||
        deltas.iter().all(|&i| { i < 0 && i >= -3 });
}

fn part2(r: &Vec<i32>) -> bool {
    for i in 0..r.len() {
        let slice: Vec<i32> = [&r[0..i], &r[i+1..r.len()]].concat();
        if part1(&slice) {
            return true;
        }
    }

    return false;
}

fn evaluator(part: i32, input: &std::path::Path) {
    let reports = parse(input);

    let mut valid = 0;
    for r in reports {
        if r.len() >= 2 {
            if part1(&r) || part == 2 && part2(&r) {
                valid += 1;
            }
        }
    }

    println!("valid: {}", valid);
}


pub fn main(part: i32, input: std::path::PathBuf) {
    match part {
        1 => evaluator(part, input.as_path()),
        2 => evaluator(part, input.as_path()),
        _ => println!("no")
    }
}
