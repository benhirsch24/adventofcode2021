use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut last = 0;
    let mut increased = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(l) = line {
                if let Ok(val) = l.parse::<u32>() {
                    print!("{} {}", last, val);
                    if val > last && last != 0 {
                        increased += 1;
                        println!(" increased");
                    } else {
                        println!();
                    }
                    last = val;
                }
            }
        }
    }
    println!("Increased: {}", increased);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
