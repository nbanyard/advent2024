use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let data: Vec<&str> = include_str!("day5-testdata.txt").lines().collect();
    let (rules, updates) = extract_data(&data);
    let mut valid = 0;
    for update in updates {
        if is_valid_update(&update, &rules) {
            valid += middle_number(&update);
        } else {
        }
    }
    println!("Number of valid updates: {}", valid);
}

fn extract_data(data: &[&str]) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let mut rules = HashMap::new();
    let mut updates = Vec::new();

    for line in data {
        if line.contains("|") {
            let parts: Vec<&str> = line.split("|").collect();
            assert_eq!(parts.len(), 2);
            let before = i32::from_str_radix(parts[0], 10).expect("Before value must be an i32");
            let after = i32::from_str_radix(parts[1], 10).expect("After value must be an i32");
            rules
                .entry(after)
                .or_insert_with(|| HashSet::new())
                .insert(before);
        } else if line.contains(",") {
            updates.push(
                line.split(",")
                    .map(|part| i32::from_str_radix(part, 10).expect("Update part must be an i32"))
                    .collect(),
            );
        }
    }
    (rules, updates)
}

fn is_valid_update(update: &[i32], rules: &HashMap<i32, HashSet<i32>>) -> bool {
    let mut not_allowed: HashSet<i32> = HashSet::new();
    for page in update {
        if not_allowed.contains(page) {
            return false;
        }
        if let Some(disallowed) = rules.get(page) {
            not_allowed.extend(disallowed);
        }
    }
    true
}

fn middle_number(update: &[i32]) -> i32 {
    update[update.len() / 2]
}
