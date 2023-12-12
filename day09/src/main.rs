fn read_file() -> impl Iterator<Item = Vec<i32>> {
    include_str!("../input.txt").lines().map(|l| {
        l.to_string()
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect()
    })
}

fn extrapolate(numbers: &Vec<i32>, rev: bool) -> i32 {
    let mut changes = Vec::new();

    let n_1 = numbers.iter();
    let n_2 = numbers.iter().skip(1);

    for n in n_1.zip(n_2) {
        changes.push(n.1 - n.0);
    }

    if numbers.iter().all(|n| *n == 0) {
        return 0;
    }
    let change = extrapolate(&changes, rev);
    if rev {
        numbers.first().unwrap() - change
    } else {
        numbers.last().unwrap() + change
    }
}

fn main() {
    let extrapolated_values_sum: i32 = read_file().map(|nums| extrapolate(&nums, false)).sum();
    println!("Solution 1: {}", extrapolated_values_sum);

    let extrapolated_values_sum: i32 = read_file().map(|nums| extrapolate(&nums, true)).sum();
    println!("Solution 2: {}", extrapolated_values_sum);
}
