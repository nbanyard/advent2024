use regex::Regex;

fn main() {
    let space = read_space(include_str!("day14-data.txt"));

    let left_max = space.width / 2; // 10:5, 11:5
    let right_min = (space.width + 1) / 2; // 10:5, 11:6
    let top_max = space.height / 2;
    let bottom_min = (space.height + 1) / 2;

    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;

    for robot in space.robots {
        let x = ((robot.0 + robot.2 * 100) % space.width + space.width) % space.width;
        let y = ((robot.1 + robot.3 * 100) % space.height + space.height) % space.height;
        if x < left_max {
            if y < top_max {
                top_left += 1;
            } else if y >= bottom_min {
                bottom_left += 1;
            }
        } else if x >= right_min {
            if y < top_max {
                top_right += 1;
            } else if y >= bottom_min {
                bottom_right += 1;
            }
        }
    }
    println!("{:?}", top_left * top_right * bottom_left * bottom_right);
}

#[derive(Debug)]
struct Space {
    width: i32,
    height: i32,
    robots: Vec<(i32, i32, i32, i32)>,
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
