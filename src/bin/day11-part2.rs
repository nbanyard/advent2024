use std::collections::HashMap;

fn main() {
    let mut line: HashMap<i64, i64> = HashMap::new();

    for word in include_str!("day11-data.txt").split_whitespace() {
        *line.entry(word.parse().unwrap()).or_insert(0) += 1;
    }

    for blink in 0..75 {
        let mut new_line: HashMap<i64, i64> = HashMap::new();
        for (value, number) in line.iter() {
            if *value == 0 {
                *new_line.entry(1).or_insert(0) += *number;
            } else {
                let s = format!("{}", *value);
                if s.len() % 2 == 0 {
                    *new_line
                        .entry(s[0..s.len() / 2].parse().unwrap())
                        .or_insert(0) += *number;
                    *new_line
                        .entry(s[s.len() / 2..].parse().unwrap())
                        .or_insert(0) += *number;
                } else {
                    *new_line.entry(value * 2024).or_insert(0) += *number;
                }
            }
        }
        line = new_line;
        println!(
            "Blink {}: {}, {} unique",
            blink,
            line.iter().fold(0, |a, (_, v)| a + v),
            line.len(),
        );
    }
}
