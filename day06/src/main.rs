fn read_file() -> Vec<String> {
    include_str!("../input.txt")
        .lines()
        .map(|l| l.to_string())
        .collect()
}

fn win_ways(time: u64, distance: u64) -> u64 {
    (0..=time).into_iter().filter(|t| t * (time - t) > distance).count() as u64
}

fn main() {
    let input = read_file();
    let times: Vec<u64> = input[0].split_at(9).1.split_whitespace().map(|d| d.parse().unwrap()).collect();
    let distances: Vec<u64> = input[1].split_at(9).1.split_whitespace().map(|d| d.parse().unwrap()).collect();

    let ways: u64 = times.iter().zip(distances.iter()).map(|(t, d)| win_ways(*t, *d)).product();
    println!("Solution 1: {}", ways);

    let times_all: u64 = input[0].split_at(9).1.split_whitespace().collect::<Vec<&str>>().join("").parse().unwrap();
    let distances_all: u64 = input[1].split_at(9).1.split_whitespace().collect::<Vec<&str>>().join("").parse().unwrap();
    println!("Solution 2: {}", win_ways(times_all, distances_all));
}
