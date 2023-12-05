const MAX_CUBES: &[(Cube, usize); 3] = &[(Cube::Red, 12), (Cube::Green, 13), (Cube::Blue, 14)];

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cube {
    Red,
    Green,
    Blue,
}

struct Move {
    cube: Cube,
    count: usize,
}

pub fn process(input: &str) -> String {
    let games = input.lines().map(|line| {
        // remove the "Game: " prefix
        let data = line.split(": ").last().expect("No data");

        let rounds = data
            .split("; ")
            .map(|round| {
                round
                    .split(", ")
                    .map(|m| {
                        let mut split_move = m.split_whitespace();

                        let num = split_move
                            .next()
                            .expect("No number")
                            .parse::<usize>()
                            .expect("Not a number");

                        let cube = match split_move.next().expect("No cube") {
                            "red" => Cube::Red,
                            "green" => Cube::Green,
                            "blue" => Cube::Blue,
                            _ => panic!("Invalid cube"),
                        };

                        Move { cube, count: num }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        rounds
    });

    let tot = games
        .enumerate()
        .filter_map(|(i, game)| {
            let possible = game.iter().all(|round| {
                let get_num_cubes = |cube: Cube| {
                    round
                        .iter()
                        .filter_map(|m| if m.cube == cube { Some(m.count) } else { None })
                        .sum::<usize>()
                };

                MAX_CUBES
                    .iter()
                    .all(|(cube, max)| get_num_cubes(*cube) <= *max)
            });

            if possible {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum::<usize>();

    tot.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(process(input), "8");
    }
}
