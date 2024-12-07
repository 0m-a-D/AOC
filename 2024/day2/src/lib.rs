#[inline]
pub fn part1(vec: &[&str]) -> bool {
    let mut data = Vec::new();
    let mut safe = true;
    for i in vec {
        let num: i8 = i.parse().unwrap();
        data.push(num);
    }
    let inc = data.is_sorted_by(|a, b| a < b);
    let dec = data.is_sorted_by(|a, b| a > b);
    let sorted = inc || dec;

    for i in 1..data.len() {
        let abs = (data[i] - data[i - 1]).abs();
        if !((1..=3).contains(&abs)) {
            safe = false;
            break;
        }
    }

    sorted && safe
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn part1_works() {
        let file = fs::read_to_string("sample.txt").expect("failed to read file");
        let content: Vec<&str> = file.lines().collect();

        let mut data = Vec::with_capacity(6);
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
        assert_eq!(2, counter);
    }
}
