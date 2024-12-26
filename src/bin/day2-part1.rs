use advent2024::data;

fn main() {
    let reports = data::read_rows(include_str!("day2-data.txt"));
    let count = reports
        .into_iter()
        .filter(|report| {
            let dir = (report[1] - report[0]).signum();
            let mut prev = report[0];
            for level in &report[1..] {
                let diff = level - prev;
                if diff == 0 || diff.abs() > 3 || diff.signum() != dir {
                    return false;
                }
                prev = *level;
            }
            true
        })
        .count();

    println!("Rows: {:?}", count);
}
