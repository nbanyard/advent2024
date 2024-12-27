type Guard = ((usize, usize), (isize, isize));

fn main() {
    let map: Vec<Vec<char>> = include_str!("day6-data.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let guard = find_guard(&map);
    let mut loops_count = 0;
    for line in 0..map.len() {
        for column in 0..map[0].len() {
            if loops_with_one_more(&map, guard, (line, column)) {
                loops_count += 1;
            }
        }
    }
    println!("Ways of looping: {}", loops_count);
}

fn find_guard(map: &[Vec<char>]) -> Guard {
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            match c {
                '^' => return ((y, x), (-1, 0)),
                '>' => return ((y, x), (0, 1)),
                'v' => return ((y, x), (1, 0)),
                '<' => return ((y, x), (0, -1)),
                _ => (),
            }
        }
    }
    panic!("Did not find the guard");
}

fn loops_with_one_more(map: &[Vec<char>], guard: Guard, extra: (usize, usize)) -> bool {
    let height = map.len();
    let width = map[0].len();
    let (mut current, mut dir) = guard;
    let mut dir_mask = 1 << (dir.0 * dir.0 * (dir.0 + 1) + dir.1 * dir.1 * (dir.1 + 2));
    let mut breadcrumbs: Vec<Vec<usize>> = (0..height)
        .map(|_| (0..width).map(|_| 0).collect())
        .collect();

    if extra == current || map[extra.0][extra.1] == '#' {
        return false;
    }

    breadcrumbs[current.0][current.1] = dir_mask;

    loop {
        let next = ((current.0 as isize + dir.0), (current.1 as isize + dir.1));
        if next.0 < 0 || next.1 < 0 {
            return false;
        }
        let next = (next.0 as usize, next.1 as usize);
        if next.0 >= height || next.1 >= width {
            return false;
        }
        if next == extra || map[next.0][next.1] == '#' {
            dir = (dir.1, -dir.0);
            dir_mask = 1 << (dir.0 * dir.0 * (dir.0 + 1) + dir.1 * dir.1 * (dir.1 + 2));
        } else {
            current = next;
            if breadcrumbs[current.0][current.1] & dir_mask != 0 {
                return true;
            }
            breadcrumbs[current.0][current.1] |= dir_mask;
        }
    }
}
