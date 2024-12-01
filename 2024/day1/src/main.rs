#![allow(unused)]

use std::{fs, path::Path};
fn main() {
    let path = Path::new("input.txt");
    let file = fs::read_to_string(path).expect("failed to read file");
    let contents: Vec<&str> = file.lines().collect();

    let mut vec1 = Vec::with_capacity(1000);
    let mut vec2 = Vec::with_capacity(1000);

    // part 1 solution
    for line in contents {
        let num1: i32 = line[..5].parse().expect("failed to parse to i32");
        let num2: i32 = line[8..].parse().expect("failed to parse to i32");
        vec1.push(num1);
        vec2.push(num2);
    }
    vec1.sort();
    vec2.sort();
    let mut total = 0;
    for i in 0..1000 {
        let diff1 = vec1[i];
        let diff2 = vec2[i];
        let diff = (diff1 - diff2).abs();
        total += diff;
    }
    println!("{total}");

    // part 2 solution
    let mut another_total = 0;
    for i in 0..1000 {
        let mut counter = 0;
        if vec2.contains(&vec1[i]) {
            for j in 0..(vec2.len()) {
                if vec1[i] == vec2[j] {
                    counter += 1;
                }
            }
        }
        another_total += vec1[i] * counter;
    }
    println!("{another_total}");
}
