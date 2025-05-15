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
            println!("Cells on best paths: {}", maze.cells_on_best_path());
        }
    };
}

// Solve the problem returning the walked maze and the lowest score, or an error
// if something goes wrong.
fn run(data: &str) -> Result<(Maze, usize), Error> {
    let mut maze = read_maze(data)?;
    let score = walk_out(&mut maze)?;
    walk_back(&mut maze)?;

    Ok((maze, score))
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
                        'E' => Cell::End(Square::default()),
                        '#' => Cell::Wall,
                        _ => Cell::Square(Square::default()),
                    })
                    .collect()
            })
            .collect(),
    })
}

fn walk_out(maze: &mut Maze) -> Result<usize, Error> {
    let mut future_steps = Vec::new();
    let mut winning_score: Option<usize> = None;

    forward_steps(&mut future_steps, 0, find_start(&maze)?, 0);

    while future_steps.len() > 0 {
        //println!("{}", maze);
        let step = future_steps.remove(0);

        if let Some(winning_score) = winning_score {
            if step.score != winning_score {
                return Ok(winning_score);
            }
        }

        let cell = &mut maze.map[step.position.0][step.position.1];
        match cell {
            Cell::End(square) => {
                square.scores[step.direction] = Some(step.score);
                winning_score = Some(step.score);
            }
            Cell::Start => (),
            Cell::Wall => (),
            Cell::Square(square) => {
                if let None = square.scores[step.direction] {
                    square.scores[step.direction] = Some(step.score);
                    forward_steps(&mut future_steps, step.score, step.position, step.direction);
                }
            }
        }
        //println!("{:?} {:?}", step, cell);
    }
    Err(Error::NoRoute)
}

fn walk_back(maze: &mut Maze) -> Result<(), Error> {
    let mut future_steps = Vec::new();
    let end = find_end(maze)?;

    if let Cell::End(square) = &maze.map[end.0][end.1] {
        for (dir, score) in square.scores.iter().enumerate() {
            if let Some(score) = score {
                backward_steps(&mut future_steps, *score, end, dir);
            }
        }
    } else {
        return Err(Error::NoRoute);
    }

    while future_steps.len() > 0 {
        //println!("{}", maze);
        let step = future_steps.remove(0);
        match &mut maze.map[step.position.0][step.position.1] {
            Cell::Wall => return Err(Error::GotLost),
            Cell::End(_) => return Err(Error::GotLost),
            Cell::Start => (),
            Cell::Square(square) => {
                //println!("{:?} - {:?}", step, square);
                if let Some(score) = square.scores[step.direction] {
                    if score <= step.score {
                        square.on_best_path = true;
                        backward_steps(&mut future_steps, score, step.position, step.direction);
                    }
                }
            }
        };
    }

    Ok(())
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

// Find the end point, or NoReindeer if it cannot be found.
fn find_end(maze: &Maze) -> Result<(usize, usize), Error> {
    for (i, l) in maze.map.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            if let Cell::End(_) = *c {
                return Ok((i, j));
            }
        }
    }

    Err(Error::NoReindeer)
}

struct Maze {
    map: Vec<Vec<Cell>>,
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.map {
            for cell in line.iter() {
                match cell {
                    Cell::End(_) => f.write_str("E")?,
                    Cell::Start => f.write_str("S")?,
                    Cell::Wall => f.write_str("#")?,
                    Cell::Square(square) => {
                        if square.on_best_path {
                            f.write_str("O")?
                        } else {
                            f.write_str(".")?
                        }
                    }
                }
            }
            if let Err(e) = f.write_str("\n") {
                return Err(e);
            }
        }
        Ok(())
    }
}

impl Maze {
    fn cells_on_best_path(&self) -> usize {
        self.map
            .iter()
            .map(|line| {
                line.iter()
                    .filter(|cell| match cell {
                        Cell::End(_) => true,
                        Cell::Start => true,
                        Cell::Wall => false,
                        Cell::Square(square) => square.on_best_path,
                    })
                    .count()
            })
            .sum()
    }
}

#[derive(Debug)]
enum Cell {
    Wall,
    Start,
    End(Square),
    Square(Square),
}

#[derive(Debug, Default)]
struct Square {
    // Score achived when arriving traveling in each direction
    // None until this square has been reached
    scores: [Option<usize>; 4],
    on_best_path: bool,
}

// forward_steps adds a step going in each direction from the current location,
// working out the score based on the incoming direction. No checks are made on
// the target locations: they could be walls or already visited locations.
fn forward_steps(
    future_steps: &mut Vec<Step>,
    score: usize,
    position: (usize, usize),
    direction: usize,
) {
    push_foward_step(future_steps, score + 1, position, direction);
    push_foward_step(future_steps, score + 1001, position, (direction + 1) % 4);
    push_foward_step(future_steps, score + 2001, position, (direction + 2) % 4);
    push_foward_step(future_steps, score + 1001, position, (direction + 3) % 4);
    future_steps.sort();
}

fn push_foward_step(
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

// backward_steps steps backward in the given direction (that is moving against
// the direction) and works out the maximum score that could have been achieved
// if the new location was reached in each direction
fn backward_steps(
    future_steps: &mut Vec<Step>,
    score: usize,
    position: (usize, usize),
    direction: usize,
) {
    push_backward_step(
        future_steps,
        score.saturating_sub(1),
        position,
        direction,
        0,
    );
    push_backward_step(
        future_steps,
        score.saturating_sub(1001),
        position,
        direction,
        1,
    );
    push_backward_step(
        future_steps,
        score.saturating_sub(2001),
        position,
        direction,
        2,
    );
    push_backward_step(
        future_steps,
        score.saturating_sub(1001),
        position,
        direction,
        3,
    );
    future_steps.sort();
}

fn push_backward_step(
    future_steps: &mut Vec<Step>,
    score: usize,
    position: (usize, usize),
    direction: usize,
    rotation: usize,
) {
    future_steps.push(Step {
        score,
        position: (
            ((position.0 as isize) - DIRECTIONS[direction].direction.0) as usize,
            ((position.1 as isize) - DIRECTIONS[direction].direction.1) as usize,
        ),
        direction: (direction + rotation) % 4,
    });
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Step {
    score: usize,
    position: (usize, usize),
    direction: usize,
}

struct Direction {
    direction: (isize, isize),
}

const DIRECTIONS: [Direction; 4] = [
    Direction { direction: (0, 1) },  // East
    Direction { direction: (1, 0) },  // South
    Direction { direction: (0, -1) }, // West
    Direction { direction: (-1, 0) }, // North
];

#[derive(Debug)]
enum Error {
    NoReindeer,
    NoRoute,
    GotLost,
}
