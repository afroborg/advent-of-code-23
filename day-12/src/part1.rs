use itertools::{repeat_n, Itertools};
use rayon::{iter::ParallelIterator, str::ParallelString};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SpringType {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for SpringType {
    fn from(c: char) -> Self {
        match c {
            '.' => SpringType::Operational,
            '#' => SpringType::Damaged,
            '?' => SpringType::Unknown,
            _ => panic!("Invalid spring type"),
        }
    }
}

#[derive(Debug)]
struct Line {
    springs_types: Vec<SpringType>,
    groupings: Vec<i32>,
}

impl Line {
    fn unknowns(&self) -> usize {
        self.springs_types
            .iter()
            .filter(|s| matches!(s, SpringType::Unknown))
            .count()
    }

    fn options(&self) -> impl Iterator<Item = Vec<SpringType>> {
        let unknowns = self.unknowns();

        repeat_n(
            [SpringType::Operational, SpringType::Damaged].into_iter(),
            unknowns,
        )
        .multi_cartesian_product()
    }
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        let (springs, groupings) = s.split_once(' ').expect("Invalid input");
        let springs_types = springs.chars().map(|c| c.into()).collect::<Vec<_>>();
        let groupings = groupings
            .split(',')
            .map(|g| g.parse::<i32>().expect("Invalid input"))
            .collect::<Vec<_>>();

        Line {
            springs_types,
            groupings,
        }
    }
}

pub fn process(input: &str) -> String {
    let lines = input.par_lines().map(Line::from);

    let counts = lines.map(|l| {
        let options = l.options();

        let valid = options.filter(|option| {
            let mut it = option.iter();

            l.springs_types
                .iter()
                .group_by(|s| {
                    let s = match s {
                        SpringType::Unknown => it.next().unwrap(),
                        _ => s,
                    };

                    *s == SpringType::Damaged
                })
                .into_iter()
                .filter_map(|(k, v)| k.then_some(v.count() as i32))
                .eq(l.groupings.iter().copied())
        });

        valid.count()
    });

    counts.sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(process(input), "21");
    }
}
