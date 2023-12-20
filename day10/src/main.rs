use std::collections::{VecDeque, BTreeSet, BTreeMap};

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

pub fn dir_str(d: Direction) -> String {
    match d {
        Direction::North => "North".to_string(),
        Direction::South => "South".to_string(),
        Direction::West => "West".to_string(),
        Direction::East => "East".to_string(),
    }
}

pub fn dir_offset(d: Direction) -> (i32, i32) {
    match d {
        Direction::North => (-1, 0),
        Direction::South => (1, 0),
        Direction::West => (0, -1),
        Direction::East => (0, 1),
    }
}

pub fn dir_connecting(d: Direction) -> Direction {
    match d {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::West => Direction::East,
        Direction::East => Direction::West,
    }
}

pub fn get_input() -> Vec<Vec<char>> {
    let input = include_str!("../input.txt").lines().map(|l| l.chars().collect());
    input.collect()
}

pub fn pipe_sides(pipe: char) -> Vec<Direction> {
    match pipe {
        '|' => vec![Direction::North, Direction::South],
        '-' => vec![Direction::East, Direction::West],
        'L' => vec![Direction::North, Direction::East],
        'J' => vec![Direction::North, Direction::West],
        '7' => vec![Direction::South, Direction::West],
        'F' => vec![Direction::South, Direction::East],
        _ => vec![],
    }
}

pub fn find_start(map: &Vec<Vec<char>>) -> (usize, usize) {
    map.iter()
        .enumerate()
        .find_map(|(y, r)| {
            r.iter()
                .enumerate()
                .find(|(_, c)| **c == 'S')
                .map(|(x, _)| (y, x))
        })
        .unwrap()
}

pub fn connect(pipe: char, dir: Direction) -> Option<Direction> {
    let sides = pipe_sides(pipe);
    if sides.is_empty() {
        None
    } else {
        sides.iter().any(|d| *d == dir).then_some(*sides.iter().filter(|d| **d != dir).next().unwrap())
    }
}

pub fn go_next(map: &Vec<Vec<char>>, pos: (usize, usize), dir: Direction) -> Option<(usize, usize, Direction)> {
    let offset = dir_offset(dir);
    let y = (pos.0 as i32 + offset.0) as usize;
    let x = (pos.1 as i32 + offset.1) as usize;
    let pipe_at = *map.get(y).map(|row| row.get(x).unwrap_or(&'.')).unwrap_or(&'.');
    let dir_conn = dir_connecting(dir);
    connect(pipe_at, dir_conn).map(|d| (y, x, d))
}

pub fn bfs_longest(map: &Vec<Vec<char>>, start: (usize, usize)) -> u32 {
    let mut visited: BTreeMap<(usize, usize), u32> = BTreeMap::new();
    let mut queue: VecDeque<(usize, usize, Direction, u32)> = VecDeque::new();
    queue.push_back((start.0, start.1, Direction::North, 0));
    queue.push_back((start.0, start.1, Direction::South, 0));
    queue.push_back((start.0, start.1, Direction::West, 0));
    queue.push_back((start.0, start.1, Direction::East, 0));
    
    while let Some(p) = queue.pop_front() {
        visited.insert((p.0, p.1), p.3);

        let next = go_next(map, (p.0, p.1), p.2);
        if let Some(n) = next {
            if !visited.contains_key(&(n.0, n.1)) {
                queue.push_back((n.0, n.1, n.2, p.3 + 1));
            }
        }
    }

    *visited.values().max().unwrap()
}

fn main() {
    let input = get_input();
    let start = find_start(&input);
    let maximum = bfs_longest(&input, start);
    println!("Solution 1: {}", maximum);
}
