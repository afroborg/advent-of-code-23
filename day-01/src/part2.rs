use std::char;

const NUMBER_STRINGS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn process(input: &str) -> String {
    let output = input
        .lines()
        .map(|line| {
            let mut numbers =
                (0..line.len()).filter_map(|index| try_get_number(&line[index..]).to_digit(10));

            let first = numbers.next().expect("No first number");
            let last = numbers.last().unwrap_or(first);

            format!("{first}{last}")
                .parse::<u32>()
                .expect("Failed to parse")
        })
        .sum::<u32>();

    output.to_string()
}

fn try_get_number(input: &str) -> char {
    for (index, number_string) in NUMBER_STRINGS.iter().enumerate() {
        if input.starts_with(number_string) {
            return char::from_digit((index + 1) as u32, 10).expect("Failed to convert");
        }
    }

    input.chars().next().expect("No character")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(process(input), "281");
    }
}
