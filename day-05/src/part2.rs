use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    slice::ParallelSlice,
};
use std::ops::Range;

#[derive(Debug)]
struct Map {
    mappings: Vec<Mapping>,
}

impl Map {
    fn convert(&self, source: u64) -> u64 {
        let mapping = self
            .mappings
            .iter()
            .find(|mapping| mapping.source.contains(&source));

        match mapping {
            Some(mapping) => mapping.destination.start + (source - mapping.source.start),
            None => source,
        }
    }
}

#[derive(Debug)]
struct Mapping {
    destination: Range<u64>,
    source: Range<u64>,
}

fn get_mapping<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Vec<Mapping> {
    let mut mappings = vec![];

    for line in iter.by_ref() {
        if line.is_empty() {
            break;
        }

        let mut parts = line.split(' ').peekable();

        if let Some(x) = parts.peek() {
            if x.contains('-') {
                continue;
            }
        }

        let range_start = parts.next().unwrap().parse::<u64>().unwrap();
        let source_range_start = parts.next().unwrap().parse::<u64>().unwrap();
        let range_length = parts.next().unwrap().parse::<u64>().unwrap();

        let mapping = Mapping {
            destination: range_start..range_start + range_length,
            source: source_range_start..source_range_start + range_length,
        };

        mappings.push(mapping);
    }

    mappings
}

pub fn process(input: &str) -> String {
    let mut lines = input.lines();
    let all_seeds = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let seeds = all_seeds
        .par_chunks(2)
        .flat_map(|chunk| {
            let range = chunk[0]..(chunk[0] + chunk[1]);
            range.into_par_iter()
        })
        .collect::<Vec<_>>();

    lines.next();

    let mut maps = vec![];

    while let Some(_) = lines.next() {
        let mappings = get_mapping(&mut lines);
        maps.push(Map { mappings });
    }

    let locations = seeds
        .into_par_iter()
        .map(|seed| maps.iter().fold(seed, |seed, map| map.convert(seed)))
        .min();

    locations.unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(process(input), "46");
    }
}
