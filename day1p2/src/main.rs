use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Windows {
    one: u32,
    two: u32,
    three: u32,
    increased: u32,
}

fn main() {
    let init = Windows{
        one:   0,
        two:   0,
        three: 0,
        increased: 0,
    };

    if let Ok(lines) = read_lines("./input") {
        let acc = lines.enumerate()
            .map(|(idx, line)| {
                if let Ok(l) = line {
                    if let Ok(val) = l.parse::<u32>() {
                        return (idx, val)
                    }
                }
                panic!("Failed to parse")
            }).fold(init, |acc, (idx, val)| {
                let new = Windows {
                    one: if idx > 1 { acc.two+val } else { 0 },
                    two: if idx > 0 { acc.three+val } else { 0 },
                    three: val,
                    increased: if idx > 2 && acc.two+val > acc.one { acc.increased + 1 } else { acc.increased },
                };
                new
            });
        println!("Increased: {}", acc.increased);
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
