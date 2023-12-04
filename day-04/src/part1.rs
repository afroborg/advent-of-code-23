use std::collections::HashSet;

struct Card {
    winning: HashSet<usize>,
    yours: HashSet<usize>,
}

impl Card {
    pub fn new(winning: HashSet<usize>, yours: HashSet<usize>) -> Self {
        Self {
            winning,
            yours,
        }
    }

    fn count(&self) -> usize {
        self.yours.intersection(&self.winning).count()
    }

    pub fn score(&self) -> u32 {
        let count = self.count();

        if count == 0 {
            return 0;
        }

        2_u32.pow(self.count().saturating_sub(1) as u32)
    }
}

pub fn process(input: &str) -> String {
    let cards = input.lines().map(|line| {
        let (_, numbers) = line.split_once(": ").unwrap();
        let (winning_numbers, your_numbers) = numbers.split_once(" | ").unwrap();

        let winning_numbers = to_hashset(winning_numbers);
        let your_numbers = to_hashset(your_numbers);

        Card::new(winning_numbers, your_numbers)
    });

    cards.map(|c| c.score()).sum::<u32>().to_string()
}

fn to_hashset(input: &str) -> HashSet<usize> {
    input.split_whitespace().map(|n| n.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(process(input), "13");
    }
}