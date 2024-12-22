
#[derive(Clone)]
#[derive(Copy)]
enum State {
    INIT,
    M,
    MU,
    MUL,
    MULLO,
    MULRO,
    D,
    DO,
    DON,
    DONAP,
    DONT,
    DOLB,
    DONTLB,
}

use State::*;

fn main() {
    let data = include_str!("day3-data.txt");

    let mut state = INIT;
    let mut left_op = 0;
    let mut right_op = 0;
    let mut total = 0;
    let mut dont = false;
    for c in data.chars() {
        match (state, c) {
            (INIT, 'm') => state = M,
            (M, 'u') => state = MU,
            (MU, 'l') => state = MUL,
            (MUL, '(') => state = MULLO,
            (MULLO, '0'..='9') => left_op = left_op * 10 + c.to_digit(10).unwrap(),
            (MULLO, ',') => state = MULRO,
            (MULRO, '0'..='9') => right_op = right_op * 10 + c.to_digit(10).unwrap(),
            (MULRO, ')') => {
                if !dont {
                    total += left_op * right_op;
                }
                state = INIT;
                left_op = 0;
                right_op = 0;
            },
            (INIT, 'd') => state = D,
            (D, 'o') => state = DO,
            (DO, '(') => state = DOLB,
            (DO, 'n') => state = DON,
            (DOLB, ')') => {
                state = INIT;
                dont = false;
            },
            (DON, '\'') => state = DONAP,
            (DONAP, 't') => state = DONT,
            (DONT, '(') => state = DONTLB,
            (DONTLB, ')') => {
                state = INIT;
                dont = true;
            },
            _ => {
                state = INIT;
                left_op = 0;
                right_op = 0;
            },
        }
    }

    println!("Total: {}", total);
}
