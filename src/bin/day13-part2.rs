use regex::Regex;

fn main() {
    let machines = parse_machines(include_str!("day13-data.txt"));
    let mut cost = 0;
    for machine in &machines {
        if let Some(machine_cost) = cheapest(machine) {
            cost += machine_cost;
        }
    }
    println!("{:?}", cost);
}

#[derive(Clone, Debug)]
struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

fn parse_machines(data: &str) -> Vec<Machine> {
    let button_re: Regex =
        Regex::new(r"Button (?P<button>[AB]): X[+](?P<x>[[:digit:]]+), Y[+](?P<y>[[:digit:]]+)")
            .unwrap();
    let prize_re: Regex =
        Regex::new(r"Prize: X=(?P<x>[[:digit:]]+), Y=(?P<y>[[:digit:]]+)").unwrap();

    let mut result = Vec::new();
    let mut new_machine: Machine = Machine {
        button_a: (0, 0),
        button_b: (0, 0),
        prize: (0, 0),
    };

    for line in data.lines() {
        if let Some(caps) = button_re.captures(line) {
            let x = caps["x"].parse().unwrap();
            let y = caps["y"].parse().unwrap();
            if &caps["button"] == "A" {
                new_machine.button_a = (x, y);
            } else {
                new_machine.button_b = (x, y);
            }
        } else if let Some(caps) = prize_re.captures(line) {
            let x = caps["x"].parse::<i64>().unwrap() + 10000000000000;
            let y = caps["y"].parse::<i64>().unwrap() + 10000000000000;
            new_machine.prize = (x, y);
            result.push(new_machine.clone());
        }
    }
    result
}

fn cheapest(machine: &Machine) -> Option<i64> {
    let a_numerator = machine.button_b.0 * machine.prize.1 - machine.prize.0 * machine.button_b.1;
    let a_denominator =
        machine.button_b.0 * machine.button_a.1 - machine.button_a.0 * machine.button_b.1;
    let b_numerator = machine.button_a.0 * machine.prize.1 - machine.prize.0 * machine.button_a.1;
    let b_denominator =
        machine.button_a.0 * machine.button_b.1 - machine.button_b.0 * machine.button_a.1;

    if a_numerator % a_denominator != 0 || b_numerator % b_denominator != 0 {
        return None;
    }

    Some(3 * a_numerator / a_denominator + b_numerator / b_denominator)
}
