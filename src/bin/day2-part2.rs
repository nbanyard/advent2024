use advent2024::data;

fn main() {
    let reports = data::read_rows(include_str!("day2-data.txt"));
    let count = reports.into_iter().filter(|report| {
        check_without_0(report) || check_without_1(report) || check_without_other(report)
    }).count();
    
    println!("Rows: {:?}", count);
}

fn check_without_0(report: &Vec<i32>) -> bool {
    let dir = (report[2] - report[1]).signum();
    let mut prev = report[1];
    for level in &report[2..] {
        let diff = level - prev;
        if diff == 0 || diff.abs() > 3 || diff.signum() != dir {
            return false;
        } else {
            prev = *level;
        }
    }
    true
}

fn check_without_1(report: &Vec<i32>) -> bool {
    let dir = (report[2] - report[0]).signum();
    let mut prev = report[0];
    for level in &report[2..] {
        let diff = level - prev;
        if diff == 0 || diff.abs() > 3 || diff.signum() != dir {
            return false;
        } else {
            prev = *level;
        }
    }
    true
}

fn check_without_other(report: &Vec<i32>) -> bool {
    let dir = (report[1] - report[0]).signum();
    let mut prev = report[0];
    let mut removed_one = false;
    for level in &report[1..] {
        let diff = level - prev;
        if diff == 0 || diff.abs() > 3 || diff.signum() != dir {
            if !removed_one {
                removed_one = true;
            } else {
                return false
            }
        } else {
            prev = *level;
        }
    }
    true
}
