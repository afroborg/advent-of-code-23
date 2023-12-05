pub fn process(input: &str) -> String {
    let output = input
        .lines()
        .map(|line| {
            let mut numbers = line.chars().filter_map(|character| character.to_digit(10));

            let first = numbers.next().expect("No first number");
            let last = numbers.last().unwrap_or(first);

            format!("{first}{last}")
                .parse::<u32>()
                .expect("Failed to parse")
        })
        .sum::<u32>();

    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(process(input), "142");
    }
}
