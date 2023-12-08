use std::collections::HashMap;

const END: &str = "ZZZ";

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

    let mut current = "AAA";
    let mut steps = 0;

    while current != END {
        let (left, right) = nodes.get(current);

        match directions[steps % directions.len()] {
            Direction::Left => current = left,
            Direction::Right => current = right,
        }

        steps += 1;
    }

    steps.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(process(input), "2");
    }

    #[test]
    fn test_process2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(process(input), "6");
    }
}
