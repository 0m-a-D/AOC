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
