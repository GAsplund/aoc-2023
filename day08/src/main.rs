use std::collections::BTreeMap;

fn main() {
    let mut input = include_str!("../input.txt").lines();
    let instructions = input.next().unwrap();
    let _ = input.next();

    let mut items: BTreeMap<String, (String, String)> = BTreeMap::new();
    while let Some(l) = input.next() {
        items.insert(l[0..3].to_string(), (l[7..10].to_string(), l[12..15].to_string()));
    }

    let mut curr = "AAA";
    let mut steps = 0;
    while curr != "ZZZ" {
        let sides = items.get(curr).unwrap();
        curr = match instructions.as_bytes()[steps % instructions.len()] {
            b'R' => sides.1.as_str(),
            b'L' => sides.0.as_str(),
            _ => curr,
        };
        steps += 1;
    }
    println!("Solution 1: {}", steps);
}
