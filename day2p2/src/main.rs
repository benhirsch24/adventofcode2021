use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
enum Command {
    Up(i32),
    Down(i32),
    Forward(i32),
}

struct State {
    depth: i32,
    position: i32,
    aim: i32,
}

fn main() {
    if let Ok(lines) = read_lines("./input") {
        let c = lines.filter_map(|l| l.ok())
            .filter_map(|l| parse_line(l))
            .fold(State{depth: 0, position: 0, aim: 0}, |acc, x| {
                match x {
                    Command::Up(v) => State{depth: acc.depth, position: acc.position, aim: acc.aim - v},
                    Command::Down(v) => State{depth: acc.depth, position: acc.position, aim: acc.aim + v},
                    Command::Forward(v) => State{depth: acc.depth + acc.aim * v, position: acc.position + v, aim: acc.aim},
                }
            });
        println!("Depth: {} X Position: {} Aim: {} Product: {}", c.depth, c.position, c.aim, c.depth * c.position);
    }
}

fn parse_line(line: String) -> Option<Command> {
    let mut iter = line.split_whitespace();
    if let Some(dir) = iter.next() {
        if let Some(mag) = iter.next() {
            if let Ok(val) = mag.parse::<i32>() {
                return match dir {
                    "up" => Some(Command::Up(val)),
                    "down" => Some(Command::Down(val)),
                    "forward" => Some(Command::Forward(val)),
                    _ => None
                };
            }
        }
    }

    return None
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
