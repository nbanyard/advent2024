fn main() {
    let mut map: Vec<Vec<char>> = include_str!("day6-data.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let width = map[0].len() as isize;
    let height = map.len() as isize;
    let mut current = (0_usize, 0_usize);
    let mut dir = (0_isize, 0_isize);
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            match c {
                '^' => {
                    current = (y, x);
                    dir = (-1, 0);
                }
                '>' => {
                    current = (y, x);
                    dir = (0, 1);
                }
                'v' => {
                    current = (y, x);
                    dir = (1, 0);
                }
                '<' => {
                    current = (y, x);
                    dir = (0, -1);
                }
                _ => (),
            }
        }
    }
    assert_ne!(dir, (0_isize, 0_isize), "Did not find the guard");
    let mut positions = 0;
    loop {
        if map[current.0][current.1] != 'X' {
            positions += 1;
            map[current.0][current.1] = 'X';
        }
        let next = ((current.0 as isize + dir.0), (current.1 as isize + dir.1));
        if next.0 < 0 || next.0 >= height || next.1 < 0 || next.1 >= width {
            break;
        }
        if map[next.0 as usize][next.1 as usize] == '#' {
            dir = (dir.1, -dir.0)
        } else {
            current = (next.0 as usize, next.1 as usize);
        }
    }
    println!("Positions visited: {:?}", positions);
}
