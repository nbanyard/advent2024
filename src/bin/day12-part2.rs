fn main() {
    let map: Vec<Vec<char>> = include_str!("day12-data.txt")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let size = (map.len(), map[0].len());
    let mut seen = vec![vec![false; size.1]; size.0];
    let mut cost = 0;
    for line in 0..size.0 {
        for column in 0..size.1 {
            if seen[line][column] {
                continue;
            }

            let region = map[line][column];
            let mut area = 0;
            let mut perimeter = 0;
            let mut stack = vec![(line, column)];

            while let Some(cell) = stack.pop() {
                if seen[cell.0][cell.1] {
                    continue;
                }
                seen[cell.0][cell.1] = true;
                area += 1;

                // Above
                if cell.0 > 0 && map[cell.0 - 1][cell.1] == region {
                    stack.push((cell.0 - 1, cell.1));
                } else {
                    if cell.1 == 0
                        || map[cell.0][cell.1 - 1] != region
                        || cell.0 > 0 && map[cell.0 - 1][cell.1 - 1] == region
                    {
                        perimeter += 1;
                    }
                }

                // Left
                if cell.1 > 0 && map[cell.0][cell.1 - 1] == region {
                    stack.push((cell.0, cell.1 - 1));
                } else {
                    if cell.0 == 0
                        || map[cell.0 - 1][cell.1] != region
                        || cell.1 > 0 && map[cell.0 - 1][cell.1 - 1] == region
                    {
                        perimeter += 1;
                    }
                }

                // Below
                if cell.0 + 1 < size.0 && map[cell.0 + 1][cell.1] == region {
                    stack.push((cell.0 + 1, cell.1));
                } else {
                    if cell.1 == 0
                        || map[cell.0][cell.1 - 1] != region
                        || cell.0 + 1 < size.0 && map[cell.0 + 1][cell.1 - 1] == region
                    {
                        perimeter += 1;
                    }
                }

                // Right
                if cell.1 + 1 < size.1 && map[cell.0][cell.1 + 1] == region {
                    stack.push((cell.0, cell.1 + 1));
                } else {
                    if cell.0 == 0
                        || map[cell.0 - 1][cell.1] != region
                        || cell.1 + 1 < size.1 && map[cell.0 - 1][cell.1 + 1] == region
                    {
                        perimeter += 1;
                    }
                }
            }

            cost += area * perimeter;
        }
    }
    println!("Cost: {}", cost);
}
