const M: i32 = 0;
const U: i32 = 1;
const L: i32 = 2;
const LB: i32 = 3;
const LO: i32 = 4;
const RO: i32 = 5;

fn main() {
    let data = include_str!("day3-data.txt");

    let mut state = M;
    let mut left_op = 0;
    let mut right_op = 0;
    let mut total = 0;
    for c in data.chars() {
        match (state, c) {
            (M, 'm') => state = U,
            (U, 'u') => state = L,
            (L, 'l') => state = LB,
            (LB, '(') => state = LO,
            (LO, '0'..='9') => left_op = left_op * 10 + c.to_digit(10).unwrap(),
            (LO, ',') => state = RO,
            (RO, '0'..='9') => right_op = right_op * 10 + c.to_digit(10).unwrap(),
            (RO, ')') => {
                total += left_op * right_op;
                state = M;
                left_op = 0;
                right_op = 0;
            }
            _ => {
                state = M;
                left_op = 0;
                right_op = 0;
            }
        }
    }

    println!("Total: {}", total);
}
