fn read_file() -> Vec<String> {
    include_str!("../input.txt")
        .split("\r\n\r\n")
        .map(|l| l.to_string())
        .collect()
}

struct MapObject {
    map_to: usize,
    map_from: usize,
    length: usize,
}

impl MapObject {
    pub fn map(&self, n: usize) -> usize {
        let source_max = self.map_from + self.length;
        if self.map_from <= n && n <= source_max {
            let offset = n - self.map_from;
            self.map_to + offset
        } else {
            n
        }
    }

    pub fn map_rev(&self, n: usize) -> usize {
        let source_max = self.map_to + self.length;
        if self.map_to <= n && n <= source_max {
            let offset = n - self.map_to;
            self.map_from + offset
        } else {
            n
        }
    }

    pub fn from_str(s: &str) -> MapObject {
        let numbers = s.split_ascii_whitespace().collect::<Vec<&str>>();
        MapObject { map_to: numbers[0].parse().unwrap(), map_from: numbers[1].parse().unwrap(), length: numbers[2].parse().unwrap() }
    }
}

fn apply_map(n: usize, m: &Vec<MapObject>) -> usize {
    let o = m.iter().find(|o| o.map(n) != n);
    match o {
        Some(obj) => obj.map(n),
        None => n
    }
}

fn apply_map_rev(n: usize, m: &Vec<MapObject>) -> usize {
    let o = m.iter().find(|o| o.map_rev(n) != n);
    match o {
        Some(obj) => obj.map_rev(n),
        None => n
    }
}

fn within_range(n: usize, ranges: &Vec<&[usize]>) -> bool {
    ranges.iter().any(|s| s[0] <= n && n <= (s[0] + s[1]))
}


fn main() {
    let map = read_file();
    //println!("{:?}", map);
    let seeds: Vec<usize> = map.iter().next().unwrap().split_at(6).1.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
    let maps: Vec<Vec<MapObject>> = map.iter().skip(1).map(|m| m.lines().into_iter().skip(1).map(|l| MapObject::from_str(l)).collect()).collect();
    let smallest_seed = seeds.iter().map(|s| {
        let mut s_new = *s;
        for m in maps.iter() {
            s_new = apply_map(s_new, m);
        }
        s_new
    }).min().unwrap();

    println!("Solution 1: {}", smallest_seed);

    let seed_ranges: Vec<&[usize]> = seeds.chunks(2).collect();

    let mut i: usize = 0;
    let smallest_range: usize = loop {
        let mut s_new = i;
        for map in maps.iter().rev() {
            s_new = apply_map_rev(s_new, map);
        }
        if within_range(s_new, &seed_ranges) {
            break i;
        }
        i += 1;
    };

    println!("Solution 2: {}", smallest_range);
}
