pub fn process(input: &str) -> String {
    let input_lines = input.lines().map(|line| {
        line.split_whitespace()
            .map(|n| n.parse::<i32>().expect("Should be a number"))
            .collect::<Vec<_>>()
    });

    let remainders = input_lines
        .map(|mut numbers| {
            let mut ends = vec![numbers.first().unwrap().to_owned()];

            loop {
                if numbers.iter().all(|n| n == &0) {
                    break;
                }

                numbers = numbers
                    .windows(2)
                    .map(|pair| pair[0] - pair[1])
                    .collect::<Vec<_>>();

                ends.push(numbers.first().unwrap().to_owned());
            }

            ends.iter().sum::<i32>()
        })
        .sum::<i32>();

    remainders.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(process(input), "2");
    }
}
