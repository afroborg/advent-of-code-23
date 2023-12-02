enum Cube {
    Red,
    Green,
    Blue
}
struct Move(Cube, u8);
struct Round(Vec<Move>);
struct Game(Vec<Round>);

pub fn process(input: &str) -> String {
    let games = input.lines().map(|line| {
        // remove the "Game: " prefix
        let data = line.split(": ").last().expect("No data");

        let rounds = data.split("; ").map(|round| {
            let moves = round.split(", ").map(|m| {
                let mut split_move = m.split_whitespace();

                let num = split_move.next().expect("No number").parse::<u8>().expect("Not a number");

                let cube = match split_move.next().expect("No cube") {
                    "red" => Cube::Red,
                    "green" => Cube::Green,
                    "blue" => Cube::Blue,
                    _ => panic!("Invalid cube")
                };

                Move(cube, num)
            }).collect::<Vec<Move>>();

            Round(moves)
        }).collect::<Vec<Round>>();

        Game(rounds)
    }).collect::<Vec<Game>>();

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let tot = games.iter().enumerate().filter_map(|(i, game)| {
        let possible = game.0.iter().all(|round| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            round.0.iter().all(|m| {
                match m.0 {
                    Cube::Red => {
                        red += m.1;
                        red <= max_red
                    },
                    Cube::Green => {
                        green += m.1;
                        green <= max_green
                    },
                    Cube::Blue => {
                        blue += m.1;
                        blue <= max_blue
                    }
                }
            })
        });

        if possible {
            Some(i + 1)
        } else {
            None
        }
    }).sum::<usize>();

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