use std::collections::{HashMap, HashSet};

fn main() {
    let map = read_map(include_str!("day8-data.txt"));
    let antinodes = find_antinodes(&map);
    println!("{:?}", antinodes.len());
}

fn read_map(data: &str) -> ((usize, usize), HashMap<char, Vec<(usize, usize)>>) {
    let mut groups = HashMap::new();
    let lines: Vec<&str> = data.lines().collect();

    for (row, line) in lines.iter().enumerate() {
        for (column, cell) in line.chars().enumerate() {
            if cell != '.' {
                groups
                    .entry(cell)
                    .or_insert_with(Vec::new)
                    .push((row, column));
            }
        }
    }

    ((lines.len(), lines[0].len()), groups)
}

fn find_antinodes(
    map: &((usize, usize), HashMap<char, Vec<(usize, usize)>>),
) -> HashSet<(usize, usize)> {
    let mut result = HashSet::new();
    let (size, groups) = map;
    for (_, antennas) in groups.iter() {
        for ant1 in antennas.iter() {
            for ant2 in antennas.iter() {
                if ant1 == ant2 {
                    continue;
                }
                let diff = (
                    ant1.0 as isize - ant2.0 as isize,
                    ant1.1 as isize - ant2.1 as isize,
                );
                let mut node = ant1.clone();
                loop {
                    result.insert(node);
                    let inode = (node.0 as isize + diff.0, node.1 as isize + diff.1);
                    if inode.0 < 0 || inode.1 < 0 {
                        break;
                    }
                    node = (inode.0 as usize, inode.1 as usize);
                    if node.0 >= size.0 || node.1 >= size.1 {
                        break;
                    }
                }
            }
        }
    }

    result
}
