use std::collections::HashMap;
use std::collections::HashSet;

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
        for (idx1, ant1) in antennas.iter().enumerate() {
            for ant2 in antennas[idx1 + 1..].iter() {
                let rowdiff = ant1.0 as isize - ant2.0 as isize;
                let coldiff = ant1.1 as isize - ant2.1 as isize;
                let node1 = (ant1.0 as isize + rowdiff, ant1.1 as isize + coldiff);
                let node2 = (ant2.0 as isize - rowdiff, ant2.1 as isize - coldiff);
                if node1.0 >= 0 && node1.1 >= 0 {
                    let node1 = (node1.0 as usize, node1.1 as usize);
                    if node1.0 < size.0 && node1.1 < size.1 {
                        result.insert(node1);
                    }
                }
                if node2.0 >= 0 && node2.1 >= 0 {
                    let node2 = (node2.0 as usize, node2.1 as usize);
                    if node2.0 < size.0 && node2.1 < size.1 {
                        result.insert(node2);
                    }
                }
            }
        }
    }

    result
}
