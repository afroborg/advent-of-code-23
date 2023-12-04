use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, PartialEq)]
enum Value {
    Number(u32),
    Symbol(char),
    None,
}

impl From<char> for Value {
    fn from(c: char) -> Self {
        match c {
            '.' => Value::None,
            c if c.is_ascii_digit() => Value::Number(c.to_digit(10).unwrap()),
            _ => Value::Symbol(c),
        }
    }
}


pub fn process(input: &str) -> String {
    let r = Regex::new(r"\d+").unwrap();
    let mut symbols = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            let value = Value::from(c);

            if let Value::Symbol(_) = value {
                symbols.insert((x as i32, y as i32), value);
            }
        }
    }

    let mut part_numbers = vec![];

    for (y, line) in input.lines().enumerate() {
        for m in r.find_iter(line) {
            let v = m.as_str().parse::<u32>().unwrap();

            let mut has_symbol = false;

            for cx in m.start().saturating_sub(1)..=m.end() {
                for cy in y.saturating_sub(1)..=y+1 {
                    let pos = (cx as i32, cy as i32);
                    let symbol = symbols.get(&pos);

                    has_symbol |= symbol.is_some();
                }
            }

            if has_symbol {
                part_numbers.push(v);
            }
        }
    }


    part_numbers.iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(process(input), "4361");
    }
}