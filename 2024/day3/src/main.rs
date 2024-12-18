#![allow(unused)]

use regex::Regex;
use std::fs;
fn main() {
    let file = fs::read_to_string("input.txt").expect("failed to read file");
    let total = part1(file);
    println!("{total}");
}

fn part1(file: String) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total = 0;
    for line in file.lines() {
        for cap in re.captures_iter(line) {
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();
            total += x * y;
        }
    }
    total
}
