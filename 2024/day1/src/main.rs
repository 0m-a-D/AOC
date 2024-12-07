#![allow(unused)]
use day1::{part1, part2};

use std::{fs, path::Path};
fn main() {
    let path = Path::new("input.txt");
    let file = fs::read_to_string(path).expect("failed to read file");
    let contents: Vec<&str> = file.lines().collect();

    let mut vec1 = Vec::with_capacity(1000);
    let mut vec2 = Vec::with_capacity(1000);

    for line in contents {
        let num1: i32 = line[..5].parse().expect("failed to parse to i32");
        let num2: i32 = line[8..].parse().expect("failed to parse to i32");
        vec1.push(num1);
        vec2.push(num2);
    }
    vec1.sort();
    vec2.sort();

    // part 1 solution
    let part1_sol = part1(&vec1, &vec2);
    println!("{part1_sol}");

    // part 2 solution
    let part2_sol = part2(vec1, vec2);
    println!("{part2_sol}");
}
