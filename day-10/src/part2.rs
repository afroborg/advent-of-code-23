use std::collections::HashSet;

#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEastBend,
    NorthWestBend,
    SouthEastBend,
    SouthWestBend,
    Ground,
    StartingPosition,
}

impl Tile {
    fn can_go(&self, direction: Direction) -> bool {
        use Direction::*;
        use Tile::*;

        match direction {
            North => matches!(
                self,
                Vertical | NorthEastBend | NorthWestBend | StartingPosition
            ),
            East => matches!(
                self,
                Horizontal | NorthEastBend | SouthEastBend | StartingPosition
            ),
            South => matches!(
                self,
                Vertical | SouthEastBend | SouthWestBend | StartingPosition
            ),
            West => matches!(
                self,
                Horizontal | NorthWestBend | SouthWestBend | StartingPosition
            ),
        }
    }

    fn can_accept(&self, direction: Direction) -> bool {
        use Direction::*;
        use Tile::*;

        match direction {
            North => matches!(self, Vertical | NorthEastBend | NorthWestBend),
            East => matches!(self, Horizontal | NorthEastBend | SouthEastBend),
            South => matches!(self, Vertical | SouthEastBend | SouthWestBend),
            West => matches!(self, Horizontal | NorthWestBend | SouthWestBend),
        }
    }

    fn all_values() -> HashSet<Self> {
        use Tile::*;

        let mut set = HashSet::new();

        set.insert(Vertical);
        set.insert(Horizontal);
        set.insert(NorthEastBend);
        set.insert(NorthWestBend);
        set.insert(SouthEastBend);
        set.insert(SouthWestBend);
        set.insert(Ground);
        set.insert(StartingPosition);

        set
    }

    fn possible_s(direction: Direction) -> HashSet<Self> {
        use Direction::*;
        use Tile::*;

        let mut set = HashSet::new();

        match direction {
            North => {
                set.insert(Vertical);
                set.insert(NorthEastBend);
                set.insert(NorthWestBend);
            }
            East => {
                set.insert(Horizontal);
                set.insert(NorthEastBend);
                set.insert(SouthEastBend);
            }
            South => {
                set.insert(Vertical);
                set.insert(SouthEastBend);
                set.insert(SouthWestBend);
            }
            West => {
                set.insert(Horizontal);
                set.insert(NorthWestBend);
                set.insert(SouthWestBend);
            }
        }

        set
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        use Tile::*;

        match c {
            '|' => Vertical,
            '-' => Horizontal,
            'L' => NorthEastBend,
            'J' => NorthWestBend,
            '7' => SouthWestBend,
            'F' => SouthEastBend,
            '.' => Ground,
            'S' => StartingPosition,
            _ => panic!("Invalid tile"),
        }
    }
}

pub fn process(input: &str) -> String {
    let grid = input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let starting_position = grid
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter().enumerate().find_map(|(x, tile)| match tile {
                Tile::StartingPosition => Some((x, y)),
                _ => None,
            })
        })
        .unwrap();

    let mut seen = HashSet::new();
    seen.insert(starting_position);

    let mut queue = vec![starting_position];

    let mut actual_s = Tile::all_values();

    while !queue.is_empty() {
        let (x, y) = queue.remove(0);
        let tile = &grid[y][x];

        if y > 0
            && tile.can_go(Direction::North)
            && grid[y - 1][x].can_accept(Direction::South)
            && !seen.contains(&(x, y - 1))
        {
            let pos = (x, y - 1);

            seen.insert(pos);
            queue.push(pos);

            actual_s = actual_s
                .intersection(&Tile::possible_s(Direction::North))
                .copied()
                .collect();
        }

        if x < grid[y].len() - 1
            && tile.can_go(Direction::East)
            && grid[y][x + 1].can_accept(Direction::West)
            && !seen.contains(&(x + 1, y))
        {
            let pos = (x + 1, y);

            seen.insert(pos);
            queue.push(pos);

            actual_s = actual_s
                .intersection(&Tile::possible_s(Direction::East))
                .copied()
                .collect();
        }

        if y < grid.len() - 1
            && tile.can_go(Direction::South)
            && grid[y + 1][x].can_accept(Direction::North)
            && !seen.contains(&(x, y + 1))
        {
            let pos = (x, y + 1);

            seen.insert(pos);
            queue.push(pos);

            actual_s = actual_s
                .intersection(&Tile::possible_s(Direction::South))
                .copied()
                .collect();
        }

        if x > 0
            && tile.can_go(Direction::West)
            && grid[y][x - 1].can_accept(Direction::East)
            && !seen.contains(&(x - 1, y))
        {
            let pos = (x - 1, y);

            seen.insert(pos);
            queue.push(pos);

            actual_s = actual_s
                .intersection(&Tile::possible_s(Direction::West))
                .copied()
                .collect();
        }
    }

    dbg!(actual_s.len()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(process(input), "4");
    }

    #[test]
    fn test_process2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(process(input), "8");
    }
}
