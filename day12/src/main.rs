use std::{
    collections::{HashMap, HashSet, VecDeque},
    mem,
    str::FromStr,
};

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone)]
enum Node {
    Start,
    CaveS(String),
    CaveM(String),
    End,
}

type AdjacencyList = HashMap<Node, Vec<Node>>;

impl FromStr for Node {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(Node::Start),
            "end" => Ok(Node::End),
            s => {
                if !s.chars().all(|c| c.is_alphabetic()) {
                    return Err(String::from("Failed to parse connection node"));
                }

                let is_lowercase = s.chars().all(|c| c.is_lowercase());
                if is_lowercase {
                    Ok(Node::CaveS(s.to_owned()))
                } else {
                    Ok(Node::CaveM(s.to_owned()))
                }
            }
        }
    }
}

fn parse_data(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|l| {
            let node_names = l.split('-').collect::<Vec<_>>();
            (String::from(node_names[0]), String::from(node_names[1]))
        })
        .collect()
}

fn build_adjacency_list(connections: Vec<(String, String)>) -> AdjacencyList {
    let mut adjacency_list = HashMap::new();

    connections.iter().for_each(|(start, end)| {
        let start_node = Node::from_str(start).unwrap();
        let end_node = Node::from_str(end).unwrap();

        let start_entry = adjacency_list
            .entry(start_node.clone())
            .or_insert(Vec::new());
        start_entry.push(end_node.clone());

        let end_entry = adjacency_list.entry(end_node).or_insert(Vec::new());
        end_entry.push(start_node);
    });

    adjacency_list
}

fn count_connections(adjacency_list: &AdjacencyList, visits_allowed: u32) -> u32 {
    let mut path_count = 0;
    let mut queue = VecDeque::new();
    queue.push_back((&Node::Start, HashSet::new(), 0));

    while let Some((node, path, repeated_visits_count)) = queue.pop_front() {
        for neighbor in adjacency_list.get(node).unwrap() {
            // TODO: get rid of these clones
            // I'm pretty sure I can simply use node references instead
            match neighbor {
                Node::CaveS(_) => {
                    if !path.contains(neighbor) {
                        let mut new_path = path.clone();
                        new_path.insert(node);
                        queue.push_back((neighbor, new_path, repeated_visits_count));
                    } else if repeated_visits_count < visits_allowed {
                        let mut new_path = path.clone();
                        new_path.insert(node);
                        queue.push_back((neighbor, new_path, repeated_visits_count + 1));
                    }
                }
                Node::CaveM(_) => {
                    let mut new_path = path.clone();
                    new_path.insert(node);
                    queue.push_back((neighbor, new_path, repeated_visits_count));
                }
                Node::End => {
                    path_count += 1;
                }
                Node::Start => {}
            }
        }
    }

    path_count
}

fn part1(data: &str) -> u32 {
    let connections_raw = parse_data(data);
    let adjacency_list = build_adjacency_list(connections_raw);
    count_connections(&adjacency_list, 0)
}

fn part2(data: &str) -> u32 {
    let connections_raw = parse_data(data);
    let adjacency_list = build_adjacency_list(connections_raw);
    count_connections(&adjacency_list, 1)
}

fn main() {
    let input = include_str!("data.txt");
    let result = part1(input);
    println!("result#1: {result}");

    let input = include_str!("data.txt");
    let result = part2(input);
    println!("result#2: {result}");

    assert_eq!(32, mem::size_of::<Node>());
    assert_eq!(8, mem::size_of::<&Node>());
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn part1_example() {
        let input = include_str!("data_example.txt");
        let result = part1(input);
        assert_eq!(result, 10);
    }

    #[test]
    fn part1_example_2() {
        let input = include_str!("data_example_2.txt");
        let result = part1(input);
        assert_eq!(result, 226);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("data_example.txt");
        let result = part2(input);
        assert_eq!(result, 36);
    }

    #[test]
    fn part2_example_2() {
        let input = include_str!("data_example_2.txt");
        let result = part2(input);
        assert_eq!(result, 3509);
    }
}
