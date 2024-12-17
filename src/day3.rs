use {once_cell::sync::Lazy, regex::Regex};

use crate::file_reader;

fn exec(part: i32, input: &std::path::Path) {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))").unwrap());

    let mut sum = 0;
    if let Ok(lines) = file_reader::read_lines(input) {
        let mut enabled = true;
        lines.for_each(|l| {
            sum += RE.captures_iter(&l.unwrap()).fold(0, |acc, c| {
                let temp = match c.get(0).unwrap().as_str() {
                    "do()" => {
                        enabled = true;
                        0
                    }
                    "don't()" => {
                        enabled = part == 1;
                        0
                    }
                    _ => {
                        let v1 = c.get(2).unwrap().as_str().parse::<i32>().unwrap();
                        let v2 = c.get(3).unwrap().as_str().parse::<i32>().unwrap();
                        if enabled {
                            v1 * v2
                        } else {
                            0
                        }
                    }
                };
                acc + temp
            })
        });
    }

    println!("sum: {sum}");
}

pub fn main(part: i32, input: std::path::PathBuf) {
    match part {
        1 => exec(part, input.as_path()),
        2 => exec(part, input.as_path()),
        _ => println!("no"),
    }
}
