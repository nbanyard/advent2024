fn main() {
    let problems = parse_problems(include_str!("day7-data.txt"));
    let mut calibration_total = 0;
    for problem in &problems {
        if is_solvable(problem) {
            calibration_total += problem.0;
        }
    }
    println!("Calibration total: {}", calibration_total,);
}

fn parse_problems(raw: &str) -> Vec<(i64, Vec<i64>)> {
    raw.lines()
        .map(|l| {
            let mut parts = l.split(":");
            let sum = parts
                .next()
                .expect("Could not find sum")
                .parse()
                .expect("Could not parse sum");
            let operands = parts
                .next()
                .expect("Could not find operands")
                .split_whitespace()
                .map(|o| o.parse().expect("Could not parse operand"))
                .collect();
            (sum, operands)
        })
        .collect()
}

fn is_solvable(problem: &(i64, Vec<i64>)) -> bool {
    let (target, operands) = problem;
    let num_operators = operands.len() - 1;
    for solution in 0..(1 << num_operators) {
        let mut sum = operands[0];
        for operator in 0..num_operators {
            if solution & 1 << operator == 0 {
                sum += operands[operator + 1];
            } else {
                sum *= operands[operator + 1];
            }
        }
        if sum == *target {
            return true;
        }
    }
    false
}
