use day1::count_all;
use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("failed to read file");
    let mut sum = 0;
    let lines: Vec<&str> = content.lines().collect();
    for i in lines {
        sum += count_all(i);
    }
    println!("{sum}");
}
