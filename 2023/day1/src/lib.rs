#[inline(always)]
pub fn count_all(string: &str) -> u32 {
    let mut result = Vec::new();
    let num_words = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    for (i, c) in string.chars().enumerate() {
        if c.is_ascii_digit() {
            result.push((i, c.to_digit(10).unwrap()));
        }
    }
    for (word, num) in num_words {
        let mut start_idx = 0;
        while let Some(idx) = string[start_idx..].find(word) {
            let abs_index = start_idx + idx;
            result.push((abs_index, num));
            start_idx = start_idx + idx + 1;
        }
    }
    result.sort_by_key(|&(idx, _)| idx);

    if result.is_empty() {
        return 0;
    }
    let first = result.first().unwrap();
    let last = result.last().unwrap();

    first.1 * 10 + last.1
}
