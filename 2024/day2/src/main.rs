use day2::part1;
use std::fs;
fn main() {
    let file = fs::read_to_string("input.txt").expect("failed to read file");
    let content: Vec<&str> = file.lines().collect();

    let mut data = Vec::with_capacity(1000);
    for line in content {
        let mut lines = Vec::new();
        for i in line.split(' ') {
            lines.push(i);
        }
        data.push(lines);
    }

    let mut counter = 0;
    for i in data {
        let cond = part1(&i);
        if cond {
            counter += 1;
        }
    }
    println!("{counter}");
}
