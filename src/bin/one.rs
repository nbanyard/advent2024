use std::iter::zip;
use crate::data;

fn main() {
    let mut s1 = Vec::from(data.LIST1);
    let mut s2 = Vec::from(data.LIST2);
    s1.sort();
    s2.sort();

    let mut distance: i32 = 0;
    for (i1, i2) in zip(s1, s2) {
        distance += (i1 - i2).abs();
    }
    println!("Distance: {}", distance);
}
