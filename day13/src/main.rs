use ndarray::prelude::*;

fn read_file() -> Vec<String> {
    include_str!("../input.txt")
        .split("\r\n\r\n")
        .map(|l| l.to_string())
        .collect()
}

struct Pattern {
    objects: Array2<bool>,
}

impl Pattern {
    pub fn from_2d_str(map: &Vec<String>) -> Pattern {
        let ncols = map.first().map_or(0, |row| row.len());
        let data: Vec<bool> = map.iter().flat_map(|i| i.chars().map(|o| o == '#')).collect();

        Pattern {
            objects: Array2::from_shape_vec((map.len(), ncols), data).unwrap(),
        }
    }

    fn valid_offset(&self, c: usize, o: usize, column: bool) -> bool {
        let length = column
            .then_some(self.objects.columns().into_iter().len())
            .unwrap_or(self.objects.rows().into_iter().len());
        let idx_1 = c.checked_sub(o);
        let idx_2 = ((c + o - 1) < length).then_some(c + o - 1);

        match (idx_1, idx_2, column) {
            (Some(i_1), Some(i_2), true) => self.objects.column(i_1) == self.objects.column(i_2),
            (Some(i_1), Some(i_2), false) => self.objects.row(i_1) == self.objects.row(i_2),
            (None, None, _) => false,
            _ => true,
        }
    }

    fn valid_split(&self, c: usize, column: bool) -> bool {
        (1..=c).all(|i| self.valid_offset(c, i, column))
    }

    pub fn find_mirror(&self) -> usize {
        let cols = self.objects.columns().into_iter().len();
        let idx = (1..cols).find(|i| self.valid_split(*i, true));

        if let Some(i) = idx {
            i
        } else {
            let rows = self.objects.rows().into_iter().len();
            let idx_2 = (1..rows).find(|i| self.valid_split(*i, false));
            100 * idx_2.unwrap_or(0)
        }
    }
}

fn main() {
    let mirrors: usize = read_file()
        .iter()
        .map(|m| Pattern::from_2d_str(&m.lines().map(|s| s.to_string()).collect()))
        .map(|m| m.find_mirror())
        .sum();
    println!("Solution 1: {}", mirrors);
}
