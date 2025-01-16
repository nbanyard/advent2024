use regex::Regex;
use std::{thread, time};

fn main() {
    let space = read_space(include_str!("day14-data.txt"));
    let pause = time::Duration::from_millis(1000);
    let mut top_rating = 0;

    for i in 0.. {
        let map = get_map(&space, i);
        let this_rating = rate_map(&map);
        if this_rating > top_rating {
            display_map(i, map);
            top_rating = this_rating;
            thread::sleep(pause);
        }
    }
}

fn get_map(space: &Space, step: i64) -> Vec<Vec<u8>> {
    let mut map = new_map(&space);

    for robot in &space.robots {
        let row =
            (((robot.1 + robot.3 * step) % space.height + space.height) % space.height) as usize;
        let column =
            (((robot.0 + robot.2 * step) % space.width + space.width) % space.width) as usize;

        map[row][column] = 'X' as u8;
    }

    map
}

#[derive(Debug)]
struct Space {
    width: i64,
    height: i64,
    robots: Vec<(i64, i64, i64, i64)>,
}

fn read_space(data: &str) -> Space {
    let space_re = Regex::new("space=([[:digit:]]+),([[:digit:]]+)").unwrap();
    let robot_re =
        Regex::new("p=([[:digit:]]+),([[:digit:]]+) v=(-?[[:digit:]]+),(-?[[:digit:]]+)").unwrap();
    let mut space = Space {
        width: 0,
        height: 0,
        robots: Vec::new(),
    };

    for line in data.lines() {
        if let Some(caps) = space_re.captures(line) {
            space.width = caps[1].parse().unwrap();
            space.height = caps[2].parse().unwrap();
        } else if let Some(caps) = robot_re.captures(line) {
            space.robots.push((
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
                caps[4].parse().unwrap(),
            ));
        } else {
            println!("Cannot read {}", line);
        }
    }

    space
}

fn new_map(space: &Space) -> Vec<Vec<u8>> {
    let mut result = Vec::with_capacity(space.height as usize);
    let mut row = Vec::with_capacity(space.width as usize);

    for _ in 0..space.width {
        row.push('.' as u8);
    }

    for _ in 0..space.height {
        result.push(row.clone());
    }

    result
}

fn rate_map(map: &Vec<Vec<u8>>) -> i64 {
    let mut result = 0;

    for (y, row) in (&map[1..map.len() - 1]).iter().enumerate() {
        for (x, cell) in (&row[1..row.len() - 1]).iter().enumerate() {
            if *cell == 'X' as u8 {
                // y and x are under by 1
                if map[y][x] == 'X' as u8 {
                    result += 1;
                }
                if map[y][x + 1] == 'X' as u8 {
                    result += 1;
                }
                if map[y][x + 2] == 'X' as u8 {
                    result += 1;
                }
                if map[y + 1][x] == 'X' as u8 {
                    result += 1;
                }
                if map[y + 1][x + 2] == 'X' as u8 {
                    result += 1;
                }
                if map[y + 2][x] == 'X' as u8 {
                    result += 1;
                }
                if map[y + 2][x + 1] == 'X' as u8 {
                    result += 1;
                }
                if map[y + 2][x + 2] == 'X' as u8 {
                    result += 1;
                }
            }
        }
    }

    result
}

fn display_map(i: i64, map: Vec<Vec<u8>>) {
    println!("");
    for row in map {
        println!("{}", String::from_utf8(row).unwrap());
    }
    println!("Step: {}", i);
}
