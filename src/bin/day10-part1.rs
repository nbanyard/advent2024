use std::collections::HashSet;

fn main() {
    let map: Vec<Vec<i64>> = include_str!("day10-data.txt")
        .lines()
        .map(|l| l.chars().map(|c| c as i64 - '0' as i64).collect())
        .collect();
    let mut routes = 0;
    for line in 0..map.len() {
        for column in 0..map[0].len() {
            if map[line][column] == 0 {
                let mut tails = HashSet::new();
                find_routes_from(&map, line, column, &mut tails);
                routes += tails.len();
            }
        }
    }
    println!("{:?}", routes);
}

fn find_routes_from(
    map: &Vec<Vec<i64>>,
    line: usize,
    column: usize,
    routes: &mut HashSet<(usize, usize)>,
) {
    let height = map[line][column];
    if height == 9 {
        routes.insert((line, column));
        return;
    }

    if line > 0 {
        if map[line - 1][column] == height + 1 {
            find_routes_from(map, line - 1, column, routes);
        }
    }
    if line + 1 < map.len() {
        if map[line + 1][column] == height + 1 {
            find_routes_from(map, line + 1, column, routes);
        }
    }
    if column > 0 {
        if map[line][column - 1] == height + 1 {
            find_routes_from(map, line, column - 1, routes);
        }
    }
    if column + 1 < map[0].len() {
        if map[line][column + 1] == height + 1 {
            find_routes_from(map, line, column + 1, routes);
        }
    }
}
