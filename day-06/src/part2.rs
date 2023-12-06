#[derive(Debug)]
struct Race {
    time: u64,
    best_distance: u64,
}

fn get_number(line: &str) -> u64 {
    line.split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .fold(String::new(), |mut acc, curr| {
            acc.push_str(curr);
            acc
        })
        .parse()
        .unwrap()
}

fn get_race(input: &str) -> Race {
    let mut lines = input.lines();
    let time = get_number(lines.next().unwrap());
    let best_distance = get_number(lines.next().unwrap());

    Race {
        time,
        best_distance,
    }
}

pub fn process(input: &str) -> String {
    let race = get_race(input);

    let ways = (0..race.time)
        .filter(|velocity| ((race.time - velocity) * velocity) > race.best_distance)
        .count();

    ways.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(process(input), "71503");
    }
}
