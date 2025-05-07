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
    Boxx,
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
    Square(Square),
    Direction(Direction),
    None,
}

#[derive(Debug)]
struct Warehouse {
    map: Vec<Vec<Square>>,
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
                '#' => Char::Square(Square::Wall),
                'O' => Char::Square(Square::Boxx),
                '@' => Char::Square(Square::Robot),
                '.' => Char::Square(Square::Space),
                '^' => Char::Direction(Direction::North),
                '>' => Char::Direction(Direction::East),
                'v' => Char::Direction(Direction::South),
                '<' => Char::Direction(Direction::West),
                _ => Char::None,
            };
            match d {
                Char::Square(s) => map_line.push(s),
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
                Square::Boxx => 'O',
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
            if let Square::Boxx = square {
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
        let mut tomove = 1;
        loop {
            match w.map[(robot.0 + resolved.0 * tomove) as usize]
                [(robot.1 + resolved.1 * tomove) as usize]
            {
                Square::Robot => return Err(Error::MultipleRobots),
                Square::Wall => {
                    tomove = 0;
                    break;
                }
                Square::Boxx => {
                    tomove += 1;
                }
                Square::Space => {
                    break;
                }
            }
        }
        if tomove > 0 {
            while tomove > 0 {
                w.map[(robot.0 + resolved.0 * tomove) as usize]
                    [(robot.1 + resolved.1 * tomove) as usize] = w.map
                    [(robot.0 + resolved.0 * (tomove - 1)) as usize]
                    [(robot.1 + resolved.1 * (tomove - 1)) as usize];
                tomove -= 1;
            }
            w.map[(robot.0) as usize][(robot.1) as usize] = Square::Space;
            robot = (robot.0 + resolved.0, robot.1 + resolved.1);
        }
    }
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
