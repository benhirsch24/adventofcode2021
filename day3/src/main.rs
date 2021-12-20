use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct State {
    zeroes: Vec<u32>,
    ones: Vec<u32>,
}

fn main() {
    if let Ok(lines) = read_lines("./input") {
        let mut st = State{zeroes: vec![0; 12], ones: vec![0; 12]};
        lines.filter_map(|l| l.ok())
            .for_each(|x| {
                x.chars().enumerate().for_each(|(idx, c)| match c {
                    '0' => st.zeroes[idx] += 1,
                    '1' => st.ones[idx] += 1,
                    _ => {},
                });
            });

        let mut gamma = 0;
        let mut epsilon = 0;
        for i in 0..12 {
            gamma = gamma << 1;
            epsilon = epsilon << 1;
            if st.zeroes[i] > st.ones[i] {
                epsilon = epsilon | 1;
            } else {
                gamma = gamma | 1;
            }
        }
        println!("{:?}", st);
        println!("{} {} {}", gamma, epsilon, gamma * epsilon);
        println!("{:b} {:b}", gamma, epsilon);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
