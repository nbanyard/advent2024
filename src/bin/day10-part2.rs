fn main() {
    let map: Vec<Vec<i64>> = include_str!("day10-data.txt")
        .lines()
        .map(|l| l.chars().map(|c| c as i64 - '0' as i64).collect())
        .collect();
    let mut routes = 0;
    for line in 0..map.len() {
        for column in 0..map[0].len() {
            if map[line][column] == 0 {
                routes += count_routes_from(&map, line, column);
            }
        }
    }
    println!("{:?}", routes);
}

fn count_routes_from(map: &Vec<Vec<i64>>, line: usize, column: usize) -> i64 {
    let height = map[line][column];
    if height == 9 {
        return 1;
    }

    let mut routes = 0;
    if line > 0 {
        if map[line - 1][column] == height + 1 {
            routes += count_routes_from(map, line - 1, column);
        }
    }
    if line + 1 < map.len() {
        if map[line + 1][column] == height + 1 {
            routes += count_routes_from(map, line + 1, column);
        }
    }
    if column > 0 {
        if map[line][column - 1] == height + 1 {
            routes += count_routes_from(map, line, column - 1);
        }
    }
    if column + 1 < map[0].len() {
        if map[line][column + 1] == height + 1 {
            routes += count_routes_from(map, line, column + 1);
        }
    }

    routes
}
