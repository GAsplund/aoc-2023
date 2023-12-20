pub fn calculate_HASH(s: &str) -> u8 {
    s.chars().into_iter().fold(0, |acc, c| ((acc + c as u32) * 17) % 256) as u8
}

fn main() {
    let hashes: u32 = include_str!("../input.txt").split(',').map(|v| calculate_HASH(v) as u32).sum();
    println!("Solution 1: {}", hashes);
}
