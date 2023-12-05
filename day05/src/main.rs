fn read_file() -> Vec<String> {
    include_str!("../test.txt")
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

    pub fn map_ranged(&self, source_start: usize, len: usize) -> Vec<usize> {
        let map_end = self.map_from + self.length;
        let source_end = source_start + len;
        if map_end > source_start && map_end < source_end {
            // Intersection, left
            let inside = self.map(source_start);
            let outside = self.map(self.map_from);
            vec![inside, outside]
        } else if source_end > self.map_from && source_start < map_end {
            // Intersection, right
            let inside = self.map(source_start);
            let outside = self.map(self.map_from);
            vec![inside, outside]
        } else if map_end > source_end && self.map_from < source_start {
            // Contains
            vec![self.map(source_start)]
        } else if self.map_from > source_start && source_end > map_end {
            // Contained
            let outside = self.map(source_start);
            let inside = self.map(self.map_from);
            vec![inside, outside]
        } else {
            // Does not contain
            vec![source_start]
        }
    }

    pub fn from_str(s: &str) -> MapObject {
        let numbers = s.split_ascii_whitespace().collect::<Vec<&str>>();
        MapObject { map_to: numbers[0].parse().unwrap(), map_from: numbers[1].parse().unwrap(), length: numbers[2].parse().unwrap() }
    }
}


fn main() {
    let map = read_file();
    //println!("{:?}", map);
    let seeds: Vec<usize> = map.iter().next().unwrap().split_at(6).1.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
    let maps: Vec<Vec<MapObject>> = map.iter().skip(1).map(|m| m.lines().into_iter().skip(1).map(|l| MapObject::from_str(l)).collect()).collect();
    let smallest_seed = seeds.iter().map(|s| {
        let mut s_orig = *s;
        let mut s_new = *s;
        for m in maps.iter() {
            let o = m.iter().find(|o| o.map(s_new) != s_orig);
            s_new = match o {
                Some(obj) => obj.map(s_new),
                None => s_new
            };
            s_orig = s_new;
        }
        s_new
    }).min().unwrap();

    println!("Solution 1: {}", smallest_seed);

    let smallest_range: usize = seeds.chunks(2).map(|s| {
        let mut s_orig = s[0];
        let mut s_new = s[0];
        for m in maps.iter() {
            let o = m.iter().find(|o| o.map_ranged(s_new, s[1]) != s_orig);
            s_new = match o {
                Some(obj) => obj.map(s_new),
                None => s_new
            };
            s_orig = s_new;
        }
        s_new
    }).min().unwrap();

    println!("Solution 2: {}", smallest_range);
}
