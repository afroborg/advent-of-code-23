struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn get(&self, r: i32, c: i32) -> char {
        self.grid[r as usize][c as usize]
    }

    fn rows(&self) -> i32 {
        self.grid.len() as i32
    }

    fn cols(&self) -> i32 {
        self.grid[0].len() as i32
    }
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let grid = s
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self { grid }
    }
}

pub fn process(input: &str) -> String {
    let grids = input.split("\n\n").map(Grid::from);

    let ans = grids.fold(0, |mut acc, grid| {
        let c_len = grid.cols();
        let r_len = grid.rows();

        for c in 0..c_len - 1 {
            let mut b = 0;

            for dc in 0..c_len {
                let left = c - dc;
                let right = c + dc + 1;

                if 0 <= left && left < right && right < c_len {
                    for r in 0..r_len {
                        if grid.get(r, left) != grid.get(r, right) {
                            b += 1;
                        }
                    }
                }
            }

            if b == 1 {
                acc += c + 1;
            }
        }

        for r in 0..r_len - 1 {
            let mut b = 0;

            for dr in 0..r_len {
                let up = r - dr;
                let down = r + dr + 1;

                if 0 <= up && up < down && down < r_len {
                    for c in 0..c_len {
                        if grid.get(up, c) != grid.get(down, c) {
                            b += 1;
                        }
                    }
                }
            }

            if b == 1 {
                acc += 100 * (r + 1);
            }
        }

        acc
    });

    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(process(input), "400");
    }
}
