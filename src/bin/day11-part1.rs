fn main() {
    let mut line: Vec<Stone> = include_str!("day11-data.txt")
        .split_whitespace()
        .map(|word| Stone(word.parse().unwrap()))
        .collect();
    for i in 0..25 {
        line = line.iter().fold(Vec::new(), |a, i| i.blink(a));

        println!("{}: {:?}", i + 1, line.len());
    }
}

#[derive(Debug)]
struct Stone(i64);

impl Stone {
    fn blink(&self, mut new_line: Vec<Stone>) -> Vec<Stone> {
        if self.0 == 0 {
            new_line.push(Stone(1));
        } else {
            let s = format!("{}", self.0);
            if s.len() % 2 == 0 {
                new_line.push(Stone(s[0..s.len() / 2].parse().unwrap()));
                new_line.push(Stone(s[s.len() / 2..].parse().unwrap()));
            } else {
                new_line.push(Stone(self.0 * 2024));
            }
        }

        new_line
    }
}
