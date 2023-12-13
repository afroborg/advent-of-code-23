use itertools::{repeat_n, Itertools};
use rayon::{iter::ParallelIterator, str::ParallelString};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
    num_unknowns: usize,
}

impl Line {
    fn options(&self) -> impl Iterator<Item = Vec<SpringType>> {
        repeat_n(
            [SpringType::Operational, SpringType::Damaged].into_iter(),
            self.num_unknowns,
        )
        .multi_cartesian_product()
    }
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        let (springs, groupings) = s.split_once(' ').expect("Invalid input");

        let springs = repeat_with_delimiter(springs, "?", 5);
        let groupings = repeat_with_delimiter(groupings, ",", 5);

        let num_unknowns = springs.chars().filter(|c| *c == '?').count();

        Line {
            springs_types: springs.chars().map(|c| c.into()).collect(),
            groupings: groupings
                .split(',')
                .map(|g| g.parse::<i32>().expect("Invalid input"))
                .collect(),
            num_unknowns,
        }
    }
}

fn repeat_with_delimiter(str: &str, delimiter: &str, n: usize) -> String {
    repeat_n(str, n).join(delimiter)
}

pub fn process(input: &str) -> String {
    input
        .par_lines()
        .map(|l| {
            let line = Line::from(l);

            line.options()
                .filter(|option| {
                    let mut it = option.iter();

                    line.springs_types
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
                        .eq(line.groupings.iter().copied())
                })
                .count()
        })
        .sum::<usize>()
        .to_string()
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
        assert_eq!(process(input), "525152");
    }

    #[test]
    fn test_repeat_with_delimiter1() {
        let input1 = ".#";
        let input2 = "1";

        assert_eq!(repeat_with_delimiter(input1, "?", 5), ".#?.#?.#?.#?.#");
        assert_eq!(repeat_with_delimiter(input2, ",", 5), "1,1,1,1,1");
    }
}
