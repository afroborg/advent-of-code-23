use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Nodes(HashMap<String, (String, String)>);

#[derive(Debug)]
struct Node(String, (String, String));

impl From<&str> for Node {
    fn from(s: &str) -> Self {
        let (start, ends) = s.split_once(" = ").unwrap();
        let (left, right) = ends
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .unwrap();

        Node(start.to_string(), (left.to_string(), right.to_string()))
    }
}

impl From<Vec<Node>> for Nodes {
    fn from(nodes: Vec<Node>) -> Self {
        let mut map = HashMap::new();

        for node in nodes {
            map.insert(node.0, node.1);
        }

        Nodes(map)
    }
}

impl Nodes {
    fn get(&self, key: &str) -> &(String, String) {
        self.0.get(key).unwrap()
    }

    fn ends_with(&self, key: char) -> Vec<String> {
        self.0
            .keys()
            .filter(|k| k.ends_with(key))
            .cloned()
            .collect()
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

pub fn process(input: &str) -> String {
    let mut lines = input.lines();
    let directions = lines
        .next()
        .unwrap()
        .chars()
        .map(Direction::from)
        .collect::<Vec<_>>();

    lines.next();

    let nodes: Nodes = lines.map(Node::from).collect::<Vec<_>>().into();

    let starting_nodes = nodes.ends_with('A');

    let all = starting_nodes
        .iter()
        .map(|node| {
            let mut current = node;
            let mut steps = 0;

            while !current.ends_with('Z') {
                let (left, right) = nodes.get(current);

                match directions[steps % directions.len()] {
                    Direction::Left => current = left,
                    Direction::Right => current = right,
                }

                steps += 1;
            }

            steps
        })
        .collect::<Vec<_>>();

    lcm(all).to_string()
}

fn lcm(nums: Vec<usize>) -> usize {
    let mut lcm = nums[0];

    for num in nums {
        lcm = (num * lcm) / gcd(num, lcm);
    }

    lcm
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(process(input), "6");
    }
}
