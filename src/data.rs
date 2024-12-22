pub fn read_columns(data: &str) -> [Vec<i32>; 2] {
    let mut result = [Vec::new(), Vec::new()];
    for line in data.lines() {
        for (idx, word) in line.split_whitespace().enumerate() {
            result[idx].push(word.parse().expect("word should be a number"));
        }
    }

    result
}
