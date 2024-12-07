pub fn part1(vec1: &[i32], vec2: &[i32]) -> i32 {
    let mut total = 0;
    for i in 0..1000 {
        let diff1 = vec1[i];
        let diff2 = vec2[i];
        let diff = (diff1 - diff2).abs();
        total += diff;
    }
    total
}

pub fn part2(vec1: Vec<i32>, vec2: Vec<i32>) -> i32 {
    let mut another_total = 0;
    for i in vec1.iter().take(1000) {
        let mut counter = 0;
        if vec2.contains(i) {
            for j in &vec2 {
                if *i == *j {
                    counter += 1;
                }
            }
        }
        another_total += i * counter;
    }
    another_total
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_works() {
        let file = fs::read_to_string("sample.txt").expect("failed to read file");
        let contents: Vec<&str> = file.lines().collect();

        let mut vec1 = Vec::with_capacity(5);
        let mut vec2 = Vec::with_capacity(5);

        for line in contents {
            let num1: i32 = line[..1].parse().expect("failed to parse to i32");
            let num2: i32 = line[4..].parse().expect("failed to parse to i32");
            vec1.push(num1);
            vec2.push(num2);
        }
        vec1.sort();
        vec2.sort();

        let res = part1(&vec1, &vec2);
        assert_eq!(res, 11);
    }
}
