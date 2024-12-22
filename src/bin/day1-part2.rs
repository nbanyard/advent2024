use std::collections::HashMap;
use advent2024::data;

fn main() {
    let [s1, s2] = data::read_columns(include_str!("day1-data.txt"));
    let m1 = count_items(&s1);
    let m2 = count_items(&s2);

    let mut distance: i32 = 0;
    for (item, count1) in m1.iter() {
        let count2 = m2.get(item).unwrap_or(&0);
        distance += item * count1 * count2;
    }
    println!("Distance: {}", distance);
}

fn count_items(items: &[i32]) -> HashMap<i32, i32> {
    let mut map = HashMap::new();
    for item in items.iter() {
        map.insert(*item, map.get(item).unwrap_or(&0) + 1);
    }
    map
}
