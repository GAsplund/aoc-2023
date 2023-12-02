fn read_file() -> Vec<String> {
    include_str!("../input.txt")
        .lines()
        .map(|l| l.to_string())
        .collect()
}

fn get_id(game: &str) -> u32 {
    game.split_at(5)
        .1
        .split_once(":")
        .unwrap()
        .0
        .parse::<u32>()
        .unwrap()
}

fn parse_game(game: &str) -> Vec<(&str, &str)> {
    let hands_str = game.split_once(": ").unwrap().1;
    let hands = hands_str.split("; ").collect::<Vec<&str>>();
    hands
        .iter()
        .flat_map(|h| h.split(", "))
        .map(|c| c.split_once(" ").unwrap())
        .collect()
}

pub fn determine_possible(game: &str) -> bool {
    let cubes = parse_game(game);

    !cubes.iter().any(|c| {
        let cnt = c.0.parse::<u32>().unwrap();
        match c.1 {
            "red" => cnt > 12,
            "green" => cnt > 13,
            "blue" => cnt > 14,
            _ => true,
        }
    })
}

fn determine_min(game: &str) -> u32 {
    let cubes = parse_game(game);

    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for c in cubes {
        let cnt = c.0.parse::<u32>().unwrap();
        match c.1 {
            "red" => {
                if cnt > max_red {
                    max_red = cnt
                }
            }
            "green" => {
                if cnt > max_green {
                    max_green = cnt
                }
            }
            "blue" => {
                if cnt > max_blue {
                    max_blue = cnt
                }
            }
            _ => {}
        }
    }

    max_red * max_green * max_blue
}

fn main() {
    let games = read_file();
    let possible_ids = games
        .iter()
        .filter(|g| determine_possible(g))
        .map(|g| get_id(g));
    let possible_sum = possible_ids.sum::<u32>();
    println!("Solution 1: {}", possible_sum);

    let game_powers = games.iter().map(|g| determine_min(g)).sum::<u32>();
    println!("Solution 2: {}", game_powers);
}
