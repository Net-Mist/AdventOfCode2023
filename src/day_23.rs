use std::collections::VecDeque;

use ahash::{HashMap, HashMapExt};
use arrayvec::ArrayVec;

#[derive(Debug)]
pub struct Node {
    position: (u8, u8),                // line, col
    connected: ArrayVec<(u8, u16), 2>, // id, distance
    from: ArrayVec<(u8, u16), 2>,      // id_origin, distance
}

#[derive(Debug)]
pub struct Graph {
    nodes: ArrayVec<Node, 36>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: ArrayVec::new(),
        }
    }

    fn len(&self) -> u8 {
        self.nodes.len() as u8
    }

    fn create_node(&mut self, position: (u8, u8)) {
        self.nodes.push(Node {
            position,
            connected: ArrayVec::new(),
            from: ArrayVec::new(),
        })
    }

    fn get_node(&self, id: u8) -> &Node {
        &self.nodes[id as usize]
    }

    fn connect(&mut self, origin_id: u8, dest_id: u8, distance: u16) {
        self.nodes[origin_id as usize]
            .connected
            .push((dest_id, distance));
        self.nodes[dest_id as usize]
            .from
            .push((origin_id, distance));
    }
}

fn follow_until_intersect(
    map: &[&[u8]],
    position: (i16, i16),
    direction: (i8, i8),
) -> Option<((u8, u8), ArrayVec<(i8, i8), 3>, u16)> {
    let mut distance = 0;
    let mut position = position;
    let mut direction = direction;
    loop {
        if position.0 as usize == map.len() - 1 {
            return Some((
                (position.0 as u8, position.1 as u8),
                ArrayVec::new(),
                distance,
            ));
        }
        let tile_type = map[position.0 as usize][position.1 as usize];
        match (tile_type, direction.0, direction.1) {
            (b'v', -1, _) => return None,
            (b'^', 1, _) => return None,
            (b'>', _, -1) => return None,
            (b'<', _, 1) => return None,
            _ => {}
        }

        let possible_directions = [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .filter_map(|possible_direction| {
                if possible_direction.0 == direction.0 * -1
                    && possible_direction.1 == direction.1 * -1
                {
                    return None;
                }
                if map[(position.0 + possible_direction.0 as i16) as usize]
                    [(position.1 + possible_direction.1 as i16) as usize]
                    != b'#'
                {
                    return Some(possible_direction);
                }
                None
            })
            .collect::<ArrayVec<_, 3>>();
        distance += 1;

        if possible_directions.len() == 0 {
            return None;
        }
        if possible_directions.len() > 1 {
            return Some((
                (position.0 as u8, position.1 as u8),
                possible_directions,
                distance,
            ));
        }

        position.0 = position.0 + possible_directions[0].0 as i16;
        position.1 = position.1 + possible_directions[0].1 as i16;
        direction = possible_directions[0];
    }
}

pub fn generator(input: &[u8]) -> Graph {
    let map: Vec<&[u8]> = input[0..input.len() - 1].split(|b| b == &b'\n').collect();
    let mut graph = Graph::new();
    let start_x = map[0]
        .iter()
        .enumerate()
        .find_map(|(a, b)| if b == &b'.' { Some(a) } else { None })
        .unwrap();
    graph.create_node((0, start_x as u8));

    let mut node_to_process: VecDeque<(u8, (i8, i8))> = VecDeque::new();
    node_to_process.push_back((0, (1, 0)));
    let mut position_to_node = HashMap::new();
    position_to_node.insert((0u8, start_x as u8), 0u8);

    while let Some((node_id, direction)) = node_to_process.pop_front() {
        let node = &graph.get_node(node_id);
        let current_position = (
            node.position.0 as i16 + direction.0 as i16,
            node.position.1 as i16 + direction.1 as i16,
        );
        if let Some((position, possible_directions, distance)) =
            follow_until_intersect(&map, current_position, direction)
        {
            if position_to_node.contains_key(&position) {
                // add the connection between node we started from and this node
                let current_node_id = position_to_node.get(&position).unwrap();
                graph.connect(node_id, *current_node_id, distance);
            } else {
                // create new node, add known connection, and other in the list of stuff to explore
                let current_node_id = graph.len();
                graph.create_node(position);
                graph.connect(node_id, current_node_id, distance);
                for direction in possible_directions {
                    node_to_process.push_back((current_node_id, direction));
                }
                position_to_node.insert(position, current_node_id);
            }
        }
    }

    graph
}

pub fn part1(input: &Graph) -> u16 {
    let mut cache = vec![0; 36];
    distance(input, &mut cache, input.nodes.len() as u8 - 1)
}

fn distance(input: &Graph, cache: &mut [u16], id: u8) -> u16 {
    if cache[id as usize] != 0 {
        return cache[id as usize];
    }
    let out = input.nodes[id as usize]
        .from
        .iter()
        .map(|(id, d)| distance(input, cache, *id) + *d)
        .max()
        .unwrap_or(1);
    cache[id as usize] = out;
    return out;
}

pub fn part2(input: &Graph) -> u16 {
    let mut explo_state: ArrayVec<(u8, u64, u16), 30> = ArrayVec::new();
    explo_state.push((0, 0u64, 1)); // node_id, marker, distance
    let mut max = 0;
    while let Some((node_id, marker, distance)) = explo_state.pop() {
        if node_id == input.nodes.len() as u8 - 1 {
            max = max.max(distance);
        }
        let node = &input.nodes[node_id as usize];
        for n in node.connected.iter().chain(node.from.iter()) {
            if (marker >> n.0) & 1 == 0 {
                explo_state.push((n.0, marker | 1 << n.0, distance + n.1 as u16));
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc_macro::test_parts;
    test_parts!(23, 2394, 6554);

    #[test]
    fn test_base() {
        let example = "#.#####################\n\
                              #.......#########...###\n\
                              #######.#########.#.###\n\
                              ###.....#.>.>.###.#.###\n\
                              ###v#####.#v#.###.#.###\n\
                              ###.>...#.#.#.....#...#\n\
                              ###v###.#.#.#########.#\n\
                              ###...#.#.#.......#...#\n\
                              #####.#.#.#######.#.###\n\
                              #.....#.#.#.......#...#\n\
                              #.#####.#.#.#########v#\n\
                              #.#...#...#...###...>.#\n\
                              #.#.#v#######v###.###v#\n\
                              #...#.>.#...>.>.#.###.#\n\
                              #####v#.#.###v#.#.###.#\n\
                              #.....#...#...#.#.#...#\n\
                              #.#########.###.#.#.###\n\
                              #...###...#...#...#.###\n\
                              ###.###.#.###v#####v###\n\
                              #...#...#.#.>.>.#.>.###\n\
                              #.###.###.#.###.#.#v###\n\
                              #.....###...###...#...#\n\
                              #####################.#\n"
            .as_bytes();
        assert_eq!(part1(&generator(example)), 94);
        assert_eq!(part2(&generator(example)), 154);
    }
}
