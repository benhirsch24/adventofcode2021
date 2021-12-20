use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Command {
    Up(i32),
    Down(i32),
    Forward(i32),
}

fn main() {
    let mut depth = 0;
    let mut posX = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(l) = line {
                if let Ok(c) = parse_line(l) {
                    match c {
                        Command::Up(x) => depth += x,
                        Command::Down(x) => depth -= x,
                        Command::Forward(x) => posX += x,
                    }
                }
            }
        }
    }
    println!("Depth: {} X Position: {} Product: {}", depth, posX, depth * posX);
}

fn parse_line(line: String) -> std::result::Result<Command, String> {
    let mut iter = line.split_whitespace();
    if let Some(dir) = iter.next() {
        if let Some(mag) = iter.next() {
            if let Ok(val) = mag.parse::<i32>() {
                return match dir {
                    "up" => Ok(Command::Up(val)),
                    "down" => Ok(Command::Down(val)),
                    "forward" => Ok(Command::Forward(val)),
                    _ => Err(format!("Failed to parse {}", dir)),
                };
            }
        }
    }

    return std::result::Result::Err("Failed in some way".to_string());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
