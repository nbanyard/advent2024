fn main() {
    let map = include_str!("day9-data.txt").trim_end_matches("\n");
    let mut disk = format_disk(&map);
    defrag(&mut disk);
    println!("{:?}", checksum(&disk.0));
}

fn format_disk(map: &str) -> (Vec<(i64, i64)>, Vec<(i64, i64)>) {
    let mut files = Vec::new();
    let mut free = Vec::new();
    let mut chars = map.chars();
    let mut block = 0_i64;
    loop {
        // File
        match chars.next() {
            Some(c) => {
                let len = c as i64 - '0' as i64;
                files.push((block, len));
                block += len;
            }
            None => break,
        }
        // Gap
        match chars.next() {
            Some(c) => {
                let len = c as i64 - '0' as i64;
                free.push((block, len));
                block += len;
            }
            None => break,
        }
    }
    (files, free)
}

fn defrag(disk: &mut (Vec<(i64, i64)>, Vec<(i64, i64)>)) {
    let (files, free) = disk;
    for file_idx in (0..files.len()).rev() {
        let (file_start, file_len) = files[file_idx];
        for free_idx in 0..free.len() {
            let (free_start, free_len) = free[free_idx];
            if file_len <= free_len && free_start < file_start {
                files[file_idx].0 = free_start;
                free[free_idx].0 += file_len;
                free[free_idx].1 -= file_len;
                free.push((file_start, file_len));
                break;
            }
        }
        free.sort();
    }
}

fn checksum(files: &Vec<(i64, i64)>) -> i64 {
    let mut result = 0;

    for (id, (start, len)) in files.iter().enumerate() {
        for i in 0..*len {
            result += (start + i) * id as i64;
        }
    }

    result
}
