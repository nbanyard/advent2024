use std::fmt;

fn main() {
    match run(include_str!("day16-data.txt")) {
        Err(err) => {
            println!("Error {:?}", err);
            return;
        }
        Ok((maze, score)) => {
            println!("{}", maze);
            println!("Score: {}", score);
        }
    };
}

// Solve the problem returning the walked maze and the lowest score, or an error
// if something goes wrong.
fn run(data: &str) -> Result<(Maze, usize), Error> {
    let mut maze = read_maze(data)?;
    let mut future_steps = Vec::new();

    next_steps(&mut future_steps, 0, find_start(&maze)?, 0);

    while future_steps.len() > 0 {
        //println!("{}", maze);
        let step = future_steps.remove(0);
        let cell = &mut maze.map[step.position.0][step.position.1];
        println!("{:?} {:?}", step, cell);
        match cell {
            Cell::End => {
                return Ok((maze, step.score));
            }
            Cell::Start => (),
            Cell::Wall => (),
            Cell::Square(directions) => {
                if !directions[step.direction] {
                    directions[step.direction] = true;
                    next_steps(&mut future_steps, step.score, step.position, step.direction);
                }
            }
        }
    }
    Err(Error::NoRoute)
}

// Convert the input data to a Maze structure.
fn read_maze(data: &str) -> Result<Maze, Error> {
    Ok(Maze {
        map: data
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        'S' => Cell::Start,
                        'E' => Cell::End,
                        '#' => Cell::Wall,
                        _ => Cell::Square([false; 4]),
                    })
                    .collect()
            })
            .collect(),
    })
}

// Find the start point, or NoReindeer if it cannot be found.
fn find_start(maze: &Maze) -> Result<(usize, usize), Error> {
    for (i, l) in maze.map.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            if let Cell::Start = *c {
                return Ok((i, j));
            }
        }
    }

    Err(Error::NoReindeer)
}

fn next_steps(
    future_steps: &mut Vec<Step>,
    score: usize,
    position: (usize, usize),
    direction: usize,
) {
    push_step(future_steps, score + 1, position, direction);
    push_step(future_steps, score + 1001, position, (direction + 1) % 4);
    push_step(future_steps, score + 2001, position, (direction + 2) % 4);
    push_step(future_steps, score + 1001, position, (direction + 3) % 4);
    future_steps.sort();
}

fn push_step(
    future_steps: &mut Vec<Step>,
    score: usize,
    position: (usize, usize),
    direction: usize,
) {
    future_steps.push(Step {
        score,
        position: (
            ((position.0 as isize) + DIRECTIONS[direction].direction.0) as usize,
            ((position.1 as isize) + DIRECTIONS[direction].direction.1) as usize,
        ),
        direction,
    });
}

struct Maze {
    map: Vec<Vec<Cell>>,
}

#[derive(Debug)]
enum Cell {
    Wall,
    Start,
    End,
    Square([bool; 4]),
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.map {
            for cell in line.iter() {
                match cell {
                    Cell::End => f.write_str("E")?,
                    Cell::Start => f.write_str("S")?,
                    Cell::Wall => f.write_str("#")?,
                    Cell::Square(_) => f.write_str(".")?,
                }
            }
            if let Err(e) = f.write_str("\n") {
                return Err(e);
            }
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Step {
    score: usize,
    position: (usize, usize),
    direction: usize,
}

#[derive(Debug)]
enum Error {
    NoReindeer,
    NoRoute,
}

struct Direction {
    direction: (isize, isize),
    marker: char,
}

const DIRECTIONS: [Direction; 4] = [
    Direction {
        direction: (0, 1),
        marker: '>',
    },
    Direction {
        direction: (1, 0),
        marker: 'v',
    },
    Direction {
        direction: (0, -1),
        marker: '<',
    },
    Direction {
        direction: (-1, 0),
        marker: '^',
    },
];
