enum Cube {
    Red,
    Green,
    Blue
}
struct Move {
    cube: Cube,
    count: usize,
}


pub fn process(input: &str) -> String {
    let games = input.lines().map(|line| {
        // remove the "Game: " prefix
        let data = line.split(": ").last().expect("No data");

        let rounds = data.split("; ").map(|round| {
            let moves = round.split(", ").map(|m| {
                let mut split_move = m.split_whitespace();

                let num = split_move.next().expect("No number").parse().expect("Not a number");

                let cube = match split_move.next().expect("No cube") {
                    "red" => Cube::Red,
                    "green" => Cube::Green,
                    "blue" => Cube::Blue,
                    _ => panic!("Invalid cube")
                };

                Move {
                    cube,
                    count: num,
                }
            }).collect::<Vec<_>>();

            moves
        }).collect::<Vec<_>>();

        rounds
    });

    let tot = games.map(|game| {
        let rgb = game.iter().fold((0,0,0), |mut acc, round| {
            round.iter().for_each(|m| {
                match m.cube {
                    Cube::Red => {
                        acc.0 = acc.0.max(m.count);
                    },
                    Cube::Green => {
                        acc.1 = acc.1.max(m.count);
                    },
                    Cube::Blue => {
                        acc.2  = acc.2.max(m.count);
                    }
                }
            });

            acc
        });

        rgb.0 * rgb.1 * rgb.2
    });


    tot.sum::<usize>().to_string()

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
        assert_eq!(process(input), "2286");
    }
}