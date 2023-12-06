#[derive(Debug)]
struct Race {
    time: u32,
    best_distance: u32,
}

fn get_numbers(line: &str) -> Vec<u32> {
    line.split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_races(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let time = get_numbers(lines.next().unwrap());
    let distances = get_numbers(lines.next().unwrap());

    time.into_iter()
        .zip(distances)
        .map(|(time, best_distance)| Race {
            time,
            best_distance,
        })
        .collect()
}

pub fn process(input: &str) -> String {
    let races = get_races(input);

    let result = races
        .iter()
        .map(|r| {
            (0..r.time)
                .filter(|velocity| ((r.time - velocity) * velocity) > r.best_distance)
                .count()
        })
        .product::<usize>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(process(input), "288");
    }
}
