const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];
const WORD: &str = "XMAS";

fn main() {
    let data: Vec<Vec<char>> = include_str!("day4-data.txt").lines().map(|line| line.chars().collect()).collect();
    let lines = data.len() as isize;
    let columns = data[0].len() as isize;
    let mut words: isize = 0;
    let mut hits: isize = 0;
    for line in 0..lines {
        for column in 0..columns {
            'nextdir: for (vdir, hdir) in DIRECTIONS {
                let vend = line + vdir * (WORD.len() as isize - 1);
                let hend = column + hdir * (WORD.len() as isize - 1);
                if vend < 0 || vend >= lines || hend < 0 || hend >= columns {
                    continue 'nextdir;
                }
                words += 1;
                for (i, c) in WORD.chars().enumerate() {
                    let v = line + vdir * i as isize;
                    let h = column + hdir * i as isize;
                    if data[v as usize][h as usize] != c {
                        continue 'nextdir;
                    }
                }
                hits += 1;
            }
        }
    }
    println!("Count: {}/{}", hits, words);
}
