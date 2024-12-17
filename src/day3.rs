use {once_cell::sync::Lazy, regex::Regex};

use crate::file_reader;

fn part1(input: &std::path::Path) {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());

    let mut sum = 0;
    if let Ok(lines) = file_reader::read_lines(input) {
        lines.for_each(|l| {
            sum += RE.captures_iter(&l.unwrap()).fold(0, |acc, c| {
                let v1 = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let v2 = c.get(2).unwrap().as_str().parse::<i32>().unwrap();
                acc + v1 * v2
            })
        });
    }

    println!("sum: {sum}");
}

pub fn main(part: i32, input: std::path::PathBuf) {
    match part {
        1 => part1(input.as_path()),
        _ => println!("no"),
    }
}
