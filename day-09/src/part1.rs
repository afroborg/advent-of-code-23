pub fn process(input: &str) -> String {
    let input_lines = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().expect("Should be a number"))
                .collect::<Vec<_>>()
        })
        .map(iterate)
        .sum::<i32>();

    input_lines.to_string()
}

fn iterate(v: Vec<i32>) -> i32 {
    let mut temp = vec![];

    for i in 0..v.len() - 1 {
        temp.push(v[i + 1] - v[i]);
    }

    let last = v.last().unwrap().to_owned();

    if temp.iter().all(|n| n == &0) {
        last
    } else {
        last + iterate(temp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(process(input), "114");
    }
}
