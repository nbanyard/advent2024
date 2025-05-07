fn main() {
    let mut space = read_space(include_str!("day15-data.txt"));
    match run_directions(&mut space) {
        Ok(()) => {
            println!("{}", output_space(&space));
            println!("{}", sum_of_boxes(&space));
        }
        Err(e) => println!("ERROR: {:?}", e),
    }
}

#[derive(Debug, Clone, Copy)]
enum Square {
    Wall,
    BoxE,
    BoxW,
    Robot,
    Space,
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
enum Error {
    NoRobot,
    MultipleRobots,
}

impl Direction {
    /// Resolve to (vertical, horizontal), postive values going east and south
    fn resolve(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
            Direction::South => (1, 0),
        }
    }
}

enum Char {
    Square(Square, Square),
    Direction(Direction),
    None,
}

type Map = Vec<Vec<Square>>;

#[derive(Debug)]
struct Warehouse {
    map: Map,
    directions: Vec<Direction>,
}

/// Read in puzzel data containing the printable form of the warehouse map and
/// the directions.
fn read_space(data: &str) -> Warehouse {
    let mut warehouse = Warehouse {
        map: vec![],
        directions: vec![],
    };
    for line in data.lines() {
        let mut map_line: Vec<Square> = Vec::new();
        for c in line.chars() {
            let d = match c {
                '#' => Char::Square(Square::Wall, Square::Wall),
                'O' => Char::Square(Square::BoxE, Square::BoxW),
                '@' => Char::Square(Square::Robot, Square::Space),
                '.' => Char::Square(Square::Space, Square::Space),
                '^' => Char::Direction(Direction::North),
                '>' => Char::Direction(Direction::East),
                'v' => Char::Direction(Direction::South),
                '<' => Char::Direction(Direction::West),
                _ => Char::None,
            };
            match d {
                Char::Square(s1, s2) => {
                    map_line.push(s1);
                    map_line.push(s2);
                }
                Char::Direction(d) => warehouse.directions.push(d),
                Char::None => (),
            }
        }
        if map_line.len() > 0 {
            warehouse.map.push(map_line);
        }
    }
    warehouse
}

/// Return a string containing the printable form of the warehouse map.
fn output_space(w: &Warehouse) -> String {
    let mut result = String::new();

    for line in &w.map[..] {
        for square in &line[..] {
            result.push(match square {
                Square::Wall => '#',
                Square::BoxE => '[',
                Square::BoxW => ']',
                Square::Robot => '@',
                Square::Space => '.',
            });
        }
        result.push('\n');
    }

    result
}

/// Return the sum of the GPS coordinates for all the boxes in the warehouse.
fn sum_of_boxes(w: &Warehouse) -> usize {
    let mut result = 0;

    for (i, line) in w.map.iter().enumerate() {
        for (j, square) in line.iter().enumerate() {
            if let Square::BoxE = square {
                result += 100 * i + j;
            }
        }
    }
    result
}

/// Move the robot around the map according to the directions, moving the
/// affected boxes. If during the processing a second robot is found
/// Err(MultipleRobots) is returned, but the map will have been updated with the
/// steps that were carried out before the error was discovered.
fn run_directions(w: &mut Warehouse) -> Result<(), Error> {
    let mut robot = find_robot(w)?;
    for direction in &w.directions {
        let resolved = direction.resolve();
        if resolved.0 == 0 {
            east_west_move(&mut w.map, &mut robot, resolved)?;
        } else {
            north_south_move(&mut w.map, &mut robot, resolved)?;
        }
    }
    Ok(())
}

/// Move the robot one square east/west.
/// resolved.0 must be 0, resolved.1 must be +/-1
fn east_west_move(
    map: &mut Map,
    robot: &mut (isize, isize),
    resolved: (isize, isize),
) -> Result<(), Error> {
    let mut tomove = 1;
    loop {
        match map[(robot.0) as usize][(robot.1 + resolved.1 * tomove) as usize] {
            Square::Robot => return Err(Error::MultipleRobots),
            Square::Wall => {
                return Ok(());
            }
            Square::BoxE => {
                tomove += 1;
            }
            Square::BoxW => {
                tomove += 1;
            }
            Square::Space => {
                break;
            }
        }
    }
    while tomove > 0 {
        map[(robot.0) as usize][(robot.1 + resolved.1 * tomove) as usize] =
            map[(robot.0) as usize][(robot.1 + resolved.1 * (tomove - 1)) as usize];
        tomove -= 1;
    }
    map[(robot.0) as usize][(robot.1) as usize] = Square::Space;
    *robot = (robot.0, robot.1 + resolved.1);
    Ok(())
}

/// Move the robot one square east/west.
/// resolved.0 must be +/-1, resolved.1 must be 0
fn north_south_move(
    map: &mut Map,
    robot: &mut (isize, isize),
    resolved: (isize, isize),
) -> Result<(), Error> {
    // Outer vector represents the rows affected, starting with the one containing
    // the robot. The inner vector contains the squares being moved on this row.
    let mut tomove: Vec<Vec<isize>> = vec![vec![robot.1]];
    loop {
        let mut nexttomove: Vec<isize> = Vec::new();
        let nextrow = robot.0 + resolved.0 * tomove.len() as isize;

        for thissquare in &tomove[tomove.len() - 1] {
            match map[nextrow as usize][*thissquare as usize] {
                Square::Robot => return Err(Error::MultipleRobots),
                Square::Wall => {
                    return Ok(());
                }
                Square::BoxE => {
                    nexttomove.push(*thissquare);
                    nexttomove.push(*thissquare + 1);
                }
                Square::BoxW => {
                    nexttomove.push(*thissquare - 1);
                    nexttomove.push(*thissquare);
                }
                Square::Space => (),
            }
        }

        if nexttomove.len() == 0 {
            break;
        }
        tomove.push(nexttomove);
    }
    for (i, rowtomove) in tomove.iter().enumerate().rev() {
        let thisrow = robot.0 + resolved.0 * i as isize;
        let nextrow = thisrow + resolved.0;
        for thissquare in rowtomove {
            map[nextrow as usize][*thissquare as usize] =
                map[thisrow as usize][*thissquare as usize];
        }
        for thissquare in rowtomove {
            map[thisrow as usize][*thissquare as usize] = Square::Space;
        }
    }
    *robot = (robot.0 + resolved.0, robot.1);
    Ok(())
}

/// Return the coordinates of the robot, return Err(NoRobot) if the robot could
/// not be found.
fn find_robot(w: &Warehouse) -> Result<(isize, isize), Error> {
    for (i, line) in w.map.iter().enumerate() {
        for (j, square) in line.iter().enumerate() {
            if let Square::Robot = square {
                return Ok((i as isize, j as isize));
            }
        }
    }
    Err(Error::NoRobot)
}
