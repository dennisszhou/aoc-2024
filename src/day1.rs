use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<std::path::Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse1(input: &std::path::Path) -> (Vec<i32>, Vec<i32>) {
    let mut l1: Vec<i32> = Vec::new();
    let mut l2: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(input) {
        let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();

        lines.for_each(|l| {
            let s = l.unwrap();
            let caps = re.captures(&s).unwrap();
            l1.push(caps.get(1).unwrap().as_str().parse::<i32>().unwrap());
            l2.push(caps.get(2).unwrap().as_str().parse::<i32>().unwrap());
        });
    }

    l1.sort();
    l2.sort();

    return (l1, l2);
}

fn part1(input: &std::path::Path) {
    let (l1, l2) = parse1(input);

    let mut sum = 0;
    for (a, b) in l1.into_iter().zip(l2.into_iter()) {
        sum += (a - b).abs();
    }
    
    println!("delta: {}", sum);
}

fn count(l: Vec<i32>) -> HashMap<i32, i32> {
    let mut map = HashMap::new();
    for num in l {
        if let Some(c) = map.get(&num) {
            map.insert(num, c + 1);
        } else {
            map.insert(num, 1);
        }
    }

    return map;
}

fn part2(input: &std::path::Path) {
    let (l1, l2) = parse1(input);

    let l2_map = count(l2);

    let mut sum = 0;
    for i in l1 {
        if let Some(c) = l2_map.get(&i) {
            sum += i * c;
        }
    }
    
    println!("delta: {}", sum);
}

pub fn main(part: i32, input: std::path::PathBuf) {
    match part {
        1 => part1(input.as_path()),
        2 => part2(input.as_path()),
        _ => println!("no")
    }
}
