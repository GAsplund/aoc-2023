use std::collections::{BTreeMap, BTreeSet};

fn read_file() -> Vec<String> {
    include_str!("../input.txt")
        .lines()
        .map(|l| l.to_string())
        .collect()
}

fn parse_map(map: Vec<String>) -> Vec<Vec<char>> {
    map.iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}

fn get_numbers(map: Vec<Vec<char>>) -> (Vec<(u32, bool)>, Vec<(u32, u32)>) {
    let mut valid_num = false;
    let mut valid_gears: Vec<(usize, usize)> = Vec::new();
    let mut buf: String = String::from("");
    let mut nums: Vec<(u32, bool)> = Vec::new();
    let mut gears: BTreeMap<(usize, usize), BTreeSet<u32>> = BTreeMap::new();

    for (i, l) in map.iter().enumerate() {
        for (j, c) in l.iter().chain(&['.']).enumerate() {
            if c.is_ascii_alphanumeric() {
                buf.push(*c);
                let (is_part, mut gears_pos) = check_around(&map, j, i);
                valid_num = valid_num || is_part;

                valid_gears.append(&mut gears_pos);
            } else if !buf.is_empty() {
                let num = buf.parse().unwrap();
                nums.push((num, valid_num));
                for g in &valid_gears {
                    let new = BTreeSet::new();
                    let pos = gears.entry(*g).or_insert_with(|| new);
                    pos.insert(num);
                }
                buf.clear();
                valid_gears.clear();
                valid_num = false;
            }
        }
    }

    let gear_ratios = gears
        .values()
        .filter(|r| r.len() == 2)
        .map(|g| (*g.first().unwrap(), *g.last().unwrap()))
        .collect();
    (nums, gear_ratios)
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_ascii_alphanumeric()
}

fn get_at_offset(map: &Vec<Vec<char>>, x: i32, y: i32) -> char {
    let empty = Vec::new();
    let l = map.get(y as usize).unwrap_or(&empty);
    *l.get(x as usize).unwrap_or(&'.')
}

const OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn check_around(map: &Vec<Vec<char>>, x: usize, y: usize) -> (bool, Vec<(usize, usize)>) {
    let symbol = OFFSETS
        .iter()
        .any(|(x_o, y_o)| is_symbol(get_at_offset(map, x as i32 + x_o, y as i32 + y_o)));
    let gears = OFFSETS
        .iter()
        .filter(|(x_o, y_o)| get_at_offset(map, x as i32 + x_o, y as i32 + y_o) == '*')
        .map(|(x_o, y_o)| ((x as i32 + x_o) as usize, (y as i32 + y_o) as usize))
        .collect();

    (symbol, gears)
}

fn main() {
    let parts = parse_map(read_file());
    let (part_numbers, gears) = get_numbers(parts);
    let numbers = part_numbers
        .iter()
        .filter(|n| n.1)
        .map(|n| n.0)
        .collect::<Vec<u32>>();

    let sum: u32 = numbers.iter().sum();
    println!("Solution 1: {}", sum);

    let ratios: u32 = gears.iter().map(|g| g.0 * g.1).sum();
    println!("Solution 2: {}", ratios);
}
