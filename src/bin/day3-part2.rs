
#[derive(Clone)]
#[derive(Copy)]
enum State {
    Init,
    M,
    Mu,
    Mul,
    Mullo,
    Mulro,
    D,
    Do,
    Don,
    Donap,
    Dont,
    Dolb,
    Dontlb,
}

use State::*;

fn main() {
    let data = include_str!("day3-data.txt");

    let mut state = Init;
    let mut left_op = 0;
    let mut right_op = 0;
    let mut total = 0;
    let mut dont = false;
    for c in data.chars() {
        match (state, c) {
            (Init, 'm') => state = M,
            (M, 'u') => state = Mu,
            (Mu, 'l') => state = Mul,
            (Mul, '(') => state = Mullo,
            (Mullo, '0'..='9') => left_op = left_op * 10 + c.to_digit(10).unwrap(),
            (Mullo, ',') => state = Mulro,
            (Mulro, '0'..='9') => right_op = right_op * 10 + c.to_digit(10).unwrap(),
            (Mulro, ')') => {
                if !dont {
                    total += left_op * right_op;
                }
                state = Init;
                left_op = 0;
                right_op = 0;
            },
            (Init, 'd') => state = D,
            (D, 'o') => state = Do,
            (Do, '(') => state = Dolb,
            (Do, 'n') => state = Don,
            (Dolb, ')') => {
                state = Init;
                dont = false;
            },
            (Don, '\'') => state = Donap,
            (Donap, 't') => state = Dont,
            (Dont, '(') => state = Dontlb,
            (Dontlb, ')') => {
                state = Init;
                dont = true;
            },
            _ => {
                state = Init;
                left_op = 0;
                right_op = 0;
            },
        }
    }

    println!("Total: {}", total);
}
