use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let data: Vec<&str> = include_str!("day5-data.txt").lines().collect();
    let (rules, updates) = extract_data(&data);
    let mut fixed = 0;
    for update in updates {
        if !is_valid_update(&update, &rules) {
            let fixed_update = fix_update(&update, &rules);
            fixed += middle_number(&fixed_update);
        }
    }
    println!("Sum of middle of fixed updates: {}", fixed);
}

fn extract_data(data: &[&str]) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let mut rules = HashMap::new();
    let mut updates = Vec::new();

    for line in data {
        if line.contains("|") {
            let parts: Vec<&str> = line.split("|").collect();
            assert_eq!(parts.len(), 2);
            let before = parts[0].parse().expect("Before value must be an i32");
            let after = parts[1].parse().expect("After value must be an i32");
            rules
                .entry(after)
                .or_insert_with(HashSet::new)
                .insert(before);
        } else if line.contains(",") {
            updates.push(
                line.split(",")
                    .map(|part| part.parse().expect("Update part must be an i32"))
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

fn fix_update(update: &[i32], rules: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let mut working = Vec::from(update);
    let mut result = Vec::new();

    'find_next: while !working.is_empty() {
        for (i, page) in working.iter().enumerate() {
            match rules.get(page) {
                None => {
                    result.push(working.remove(i));
                    continue 'find_next;
                }
                Some(need_first) => {
                    let mut ok = true;
                    for later in &working[i + 1..] {
                        if need_first.contains(later) {
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        result.push(working.remove(i));
                        continue 'find_next;
                    }
                }
            }
        }
    }
    result
}

fn middle_number(update: &[i32]) -> i32 {
    update[update.len() / 2]
}
