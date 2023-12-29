use std::collections::VecDeque;

use ahash::{HashMap, HashMapExt};
use arrayvec::ArrayVec;

const N_NODE: usize = 59;

#[derive(Debug, Clone)]
struct FlipFlop {
    state: bool,
}

#[derive(Debug, Clone)]
struct Conj {
    inputs: HashMap<usize, bool>,
}

#[derive(Debug, Clone)]
enum Node {
    FlipFlop(FlipFlop),
    Conj(Conj),
    Undef,
    Broadcaster,
}

#[derive(Debug, Clone)]
pub struct Graph {
    nodes: ArrayVec<Node, N_NODE>,
    parents: ArrayVec<Vec<usize>, N_NODE>,
    children: ArrayVec<Vec<usize>, N_NODE>,
    broadcaster_id: usize,
    rx_id: usize,
}

struct GraphBuilder<'a> {
    graph: Graph,
    next_id: usize,
    name_to_id: HashMap<&'a [u8], usize>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: ArrayVec::new(),
            parents: ArrayVec::new(),
            children: ArrayVec::new(),
            broadcaster_id: 0,
            rx_id: 0,
        }
    }
}

impl<'a> GraphBuilder<'a> {
    fn new() -> GraphBuilder<'a> {
        GraphBuilder {
            graph: Graph::new(),
            next_id: 0,
            name_to_id: HashMap::<&'a [u8], usize>::new(),
        }
    }

    fn get_or_create_node_id(&mut self, name: &'a [u8]) -> usize {
        self.name_to_id
            .entry(name)
            .or_insert_with(|| {
                self.next_id += 1;
                if name.len() == 10 {
                    self.graph.nodes.push(Node::Broadcaster);
                    self.graph.broadcaster_id = self.next_id - 1;
                } else {
                    self.graph.nodes.push(Node::Undef);
                }
                self.graph.parents.push(vec![]);
                self.graph.children.push(vec![]);
                self.next_id - 1
            })
            .to_owned()
    }
}

pub fn generator(input: &[u8]) -> Graph {
    let mut graph_builder = GraphBuilder::new();

    let lines = input[0..input.len() - 1]
        .split(|b| b == &b'\n')
        .collect::<Vec<_>>();

    for line in lines {
        let (t, line) = line.split_first().unwrap();

        let node = if t == &b'%' {
            Node::FlipFlop(FlipFlop { state: false })
        } else if t == &b'&' {
            Node::Conj(Conj {
                inputs: HashMap::new(),
            })
        } else {
            Node::Undef
        };

        let node_ids = line
            .split(|b| !b.is_ascii_lowercase())
            .filter(|i| !i.is_empty())
            .map(|name| graph_builder.get_or_create_node_id(name))
            .collect::<Vec<usize>>();

        graph_builder.graph.nodes[node_ids[0]] = node;
        graph_builder.graph.children[node_ids[0]] = node_ids[1..].to_vec();
    }
    // now connect parents
    for (parent, children) in graph_builder.graph.children.iter().enumerate() {
        for child in children.iter() {
            graph_builder.graph.parents[*child].push(parent);
            if let Node::Conj(node) = &mut graph_builder.graph.nodes[*child] {
                node.inputs.insert(parent, false);
            }
        }
    }

    let k = &[b'r', b'x'][..];
    graph_builder.graph.rx_id = graph_builder.name_to_id.get(k).unwrap().to_owned();
    graph_builder.graph
}

pub fn part1(input: &Graph) -> usize {
    let mut graph = input.to_owned();
    let mut event_queue = VecDeque::new();

    let mut n_low = 0;
    let mut n_high = 0;
    for _ in 0..1000 {
        n_low += 1;

        for i in graph.children[graph.broadcaster_id].iter() {
            event_queue.push_back((graph.broadcaster_id, *i, false));
        }
        while let Some((event_from, event_to, event_type)) = event_queue.pop_front() {
            if event_type {
                n_high += 1;
            } else {
                n_low += 1;
            }
            if let Node::FlipFlop(FlipFlop { state }) = &mut graph.nodes[event_to] {
                if !event_type {
                    *state = !*state;
                    for c in graph.children[event_to].iter() {
                        event_queue.push_back((event_to, *c, *state));
                    }
                }
            }
            if let Node::Conj(Conj { inputs }) = &mut graph.nodes[event_to] {
                inputs.insert(event_from, event_type);
                let s = !inputs.values().all(|v| *v);
                for c in graph.children[event_to].iter() {
                    event_queue.push_back((event_to, *c, s));
                }
            }
        }
    }
    n_high * n_low
}

pub fn part2(input: &Graph) -> usize {
    let mut graph = input.to_owned();
    let mut event_queue = VecDeque::new();

    let rx_parent_ids = &graph.parents[graph.rx_id];
    assert!(rx_parent_ids.len() == 1);
    let rx_parent_id = rx_parent_ids[0];

    let mut out = 1;
    let mut n_out = 0;

    for j in 0..5000 {
        for i in graph.children[graph.broadcaster_id].iter() {
            event_queue.push_back((graph.broadcaster_id, *i, false));
        }
        while let Some((event_from, event_to, event_type)) = event_queue.pop_front() {
            if event_to == rx_parent_id && event_type {
                out *= j + 1;
                n_out += 1;
                if n_out == 4 {
                    return out;
                }
            }
            if let Node::FlipFlop(FlipFlop { state }) = &mut graph.nodes[event_to] {
                if !event_type {
                    *state = !*state;
                    for c in graph.children[event_to].iter() {
                        event_queue.push_back((event_to, *c, *state));
                    }
                }
            }
            if let Node::Conj(Conj { inputs }) = &mut graph.nodes[event_to] {
                inputs.insert(event_from, event_type);
                let s = !inputs.values().all(|v| *v);
                for c in graph.children[event_to].iter() {
                    event_queue.push_back((event_to, *c, s));
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc_macro::test_parts;
    test_parts!(20, 919383692, 247702167614647);

    #[test]
    fn test_base() {
        let example = "broadcaster -> a\n\
                            %a -> inv, con\n\
                            &inv -> b\n\
                            %b -> con\n\
                            &con -> rx\n"
            .as_bytes();
        assert_eq!(part1(&generator(example)), 11687500);
    }
}
