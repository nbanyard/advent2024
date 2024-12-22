use std::iter::zip;
use advent2024::data;

fn main() {
    let [mut s1, mut s2] = data::read_columns(include_str!("day1-data.txt"));
    s1.sort();
    s2.sort();

    let mut distance: i32 = 0;
    for (i1, i2) in zip(s1, s2) {
        distance += (i1 - i2).abs();
    }
    println!("Distance: {}", distance);
}
