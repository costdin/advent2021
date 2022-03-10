use super::get_input_lines;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub fn day12() {
    let input = get_input_lines(12);
    let nodes = parse(input);

    let result1 = build_path(
        nodes.iter().filter(|n| n.is_start).nth(0).unwrap(),
        &mut Path::new(),
        &is_valid_path,
    );

    let result2 = build_path(
        nodes.iter().filter(|n| n.is_start).nth(0).unwrap(),
        &mut Path::new(),
        &is_valid_path2,
    );

    println!("DAY 12\nSolution 1: {}\nSolution 2: {}", result1, result2);
}

fn parse(lines: Vec<String>) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::with_capacity(lines.len());
    let mut hashmap = HashMap::new();
    let ptr = nodes.as_mut_ptr();

    for connection in lines {
        if let [input, output] = connection.split("-").collect::<Vec<_>>()[..] {
            let output = match hashmap.get(output) {
                Some(i) => unsafe { ptr.offset(*i as isize) },
                None => {
                    nodes.push(Node::parse(output));
                    hashmap.insert(output.to_string(), nodes.len() - 1);
                    unsafe { ptr.offset(nodes.len() as isize - 1) }
                }
            };

            let input = match hashmap.get(input) {
                Some(i) => {
                    nodes[*i].connections.push(output);

                    unsafe { ptr.offset(*i as isize) }
                }
                None => {
                    let mut n = Node::parse(input);
                    n.connections.push(output);
                    nodes.push(n);

                    hashmap.insert(input.to_string(), nodes.len() - 1);

                    unsafe { ptr.offset(nodes.len() as isize - 1) }
                }
            };

            unsafe {
                (*output).connections.push(input);
            }
        }
    }

    nodes
}

fn build_path<'a, 'b, F>(node: &'a Node, current_path: &mut Path<'a>, path_validator: &F) -> u32
where
    F: Fn(&Path, &Node) -> bool,
{
    if node.is_end {
        1
    } else if path_validator(current_path, node) {
        current_path.add(node);

        let mut total = 0;

        for x in node.connections() {
            total += build_path(x, current_path, path_validator)
        }

        current_path.remove(node);

        total
    } else {
        0
    }
}

fn is_valid_path(current_path: &Path, new_node: &Node) -> bool {
    new_node.is_large || !current_path.small.contains_key(&new_node)
}

fn is_valid_path2(current_path: &Path, new_node: &Node) -> bool {
    if is_valid_path(current_path, new_node) {
        true
    } else {
        !new_node.is_start && !new_node.is_end && !current_path.has_double_small
    }
}

struct Path<'a> {
    has_double_small: bool,
    small: HashMap<&'a Node, u8>,
}

impl<'a> Path<'a> {
    fn new() -> Path<'a> {
        Path {
            has_double_small: false,
            small: HashMap::new(),
        }
    }

    fn add(&mut self, node: &'a Node) {
        if !node.is_large {
            match self.small.get_mut(node) {
                Some(c) => {
                    *c += 1;
                    self.has_double_small = true;
                }
                None => {
                    self.small.insert(node, 1);
                }
            }
        }
    }

    fn remove(&mut self, node: &'a Node) {
        if !node.is_large {
            let remove = match self.small.get_mut(node) {
                Some(c) if *c == 2 => {
                    *c = 1;
                    self.has_double_small = false;
                    false
                }
                Some(c) if *c == 1 => true,
                Some(_) => unreachable!("We should never get here!"),
                None => false,
            };

            if remove {
                self.small.remove(node);
            }
        }
    }
}

struct Node {
    is_start: bool,
    is_end: bool,
    is_large: bool,
    connections: Vec<*mut Node>,
    id: String,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Node {}

impl Node {
    #[inline]
    fn connections(&self) -> impl Iterator<Item = &Node> {
        self.connections
            .iter()
            .map(|c| unsafe { c.as_ref().unwrap() })
    }

    fn parse(s: &str) -> Node {
        match s {
            "start" => Node {
                is_start: true,
                is_end: false,
                is_large: false,
                connections: vec![],
                id: s.to_string(),
            },
            "end" => Node {
                is_start: false,
                is_end: true,
                is_large: false,
                connections: vec![],
                id: s.to_string(),
            },
            s => Node {
                is_start: false,
                is_end: false,
                is_large: s.chars().nth(0).unwrap().is_ascii_uppercase(),
                connections: vec![],
                id: s.to_string(),
            },
        }
    }
}
