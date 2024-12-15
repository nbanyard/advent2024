use std::collections::HashMap;
use advent2024::data;

fn main() {
    let m1 = countItems(data::LIST1);
    let m2 = countItems(data::LIST2);

    let mut distance: i32 = 0;
    for (item, count1) in m1.iter() {
        let count2 = m2.unwrap_or(0);
        distance += item * count1 * count2;
    }
    println!("Distance: {}", distance);
}

fn countItems(items: &[u32]) -> HashMap<u32, u32> {
    let mut map = HashMap::new();
    for item in items.iter() {
        map.insert(item, map.get(item).unwrap_or(0) + 1);
    }
    map
}
