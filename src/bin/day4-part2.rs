fn main() {
    let data: Vec<Vec<char>> = include_str!("day4-data.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let lines = data.len();
    let columns = data[0].len();
    let mut xs: isize = 0;
    let mut hits: isize = 0;
    for line in 0..lines - 2 {
        for column in 0..columns - 2 {
            let ul = data[line][column];
            let ur = data[line][column + 2_usize];
            let c = data[line + 1_usize][column + 1_usize];
            let ll = data[line + 2_usize][column];
            let lr = data[line + 2_usize][column + 2_usize];
            xs += 1;
            if (ul == 'M' && lr == 'S' || ul == 'S' && lr == 'M')
                && c == 'A'
                && (ll == 'M' && ur == 'S' || ll == 'S' && ur == 'M')
            {
                hits += 1;
            }
        }
    }
    println!("Count: {}/{}", hits, xs);
}
