fn main() {
    let map = include_str!("day9-data.txt").trim_end_matches("\n");
    let mut disk = format_disk(&map);
    defrag(&mut disk);
    println!("{:?}", checksum(&disk));
}

fn format_disk(map: &str) -> Vec<Option<i64>> {
    let mut result = Vec::new();
    let mut chars = map.chars();
    let mut id = 0_i64;
    loop {
        // File
        match chars.next() {
            Some(c) => {
                for _ in 0..(c as usize - '0' as usize) {
                    result.push(Some(id));
                }
            }
            None => break,
        }
        // Gap
        match chars.next() {
            Some(c) => {
                for _ in 0..(c as usize - '0' as usize) {
                    result.push(None);
                }
            }
            None => break,
        }
        id += 1;
    }
    result
}

fn defrag(disk: &mut Vec<Option<i64>>) {
    let mut start: usize = 0;
    let mut end: usize = disk.len();

    while start < end {
        if disk[start] != None {
            start += 1;
        } else if disk[end - 1] == None {
            end -= 1;
        } else {
            disk[start] = disk[end - 1];
            disk[end - 1] = None;
            start += 1;
            end -= 1;
        }
    }
}

fn checksum(disk: &Vec<Option<i64>>) -> i64 {
    let mut result = 0;

    for (idx, id) in disk.iter().enumerate() {
        if let Some(id) = id {
            result += idx as i64 * *id;
        }
    }

    result
}
