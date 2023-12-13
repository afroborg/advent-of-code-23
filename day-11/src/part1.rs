use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Galaxy,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Galaxy,
            _ => panic!("Invalid tile"),
        }
    }
}

pub fn process(input: &str) -> String {
    let mut galaxies = vec![];
    let mut empty_rows: HashSet<i32> = HashSet::new();
    let mut empty_cols: HashSet<i32> = HashSet::new();

    let tiles = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let tiles = line
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    let tile = Tile::from(c);

                    if let Tile::Galaxy = tile {
                        galaxies.push((x as i32, y as i32));
                    }

                    tile
                })
                .collect::<Vec<_>>();

            if tiles.iter().all(|t| matches!(t, Tile::Empty)) {
                empty_rows.insert(y as i32);
            }

            tiles
        })
        .collect::<Vec<_>>();

    for x in 0..tiles[0].len() {
        if tiles.iter().all(|row| matches!(row[x], Tile::Empty)) {
            empty_cols.insert(x as i32);
        }
    }

    let expansion_factor = 1;

    let sum = galaxies.iter().enumerate().map(|(i, (x, y))| {
        (i..galaxies.len()).fold(0, |acc2, j| {
            let (x2, y2) = galaxies[j];
            let mut dij = (y2 - y).abs() + (x2 - x).abs();

            for er in empty_rows.iter() {
                if y.min(&y2) < er && er < y.max(&y2) {
                    dij += expansion_factor;
                }
            }

            for ec in empty_cols.iter() {
                if x.min(&x2) < ec && ec < x.max(&x2) {
                    dij += expansion_factor;
                }
            }

            acc2 + dij
        })
    });

    sum.sum::<i32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(process(input), "374");
    }
}
