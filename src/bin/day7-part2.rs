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

const BASE: usize = 3;

fn is_solvable(problem: &(i64, Vec<i64>)) -> bool {
    let (target, operands) = problem;
    let num_operators = operands.len() - 1;
    for solution in 0..(BASE.pow(num_operators as u32)) {
        let mut sum = operands[0];
        let mut operators = solution;
        for operator in 0..num_operators {
            if operators % BASE == 0 {
                sum += operands[operator + 1];
            } else if operators % BASE == 1 {
                sum *= operands[operator + 1];
            } else {
                sum = format!("{}{}", sum, operands[operator + 1])
                    .parse()
                    .unwrap();
            }
            operators /= BASE;
        }
        if sum == *target {
            return true;
        }
    }
    false
}
