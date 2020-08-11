use std::collections::{HashSet, VecDeque};
use std::hash::Hasher;
use std::rc::Rc;
use std::fmt;
use crate::entities::Position;

pub fn dfs<'a>(maze: &crate::entities::Maze) -> (Node, HashSet<crate::entities::Position>) {
    let mut traversed = Vec::new();
    let mut frontier: VecDeque<Box<Node>> = VecDeque::new();
    let mut seen: HashSet<crate::entities::Position> = HashSet::new();

    let parent = Node {
        parent: None,
        position: maze.start
    };
    frontier.push_back(Box::new(parent));

    loop {
        if frontier.is_empty() {
            panic!("No solution found!")
        }
        let current: Node = *frontier.pop_front().expect("There must be a node here");
        let position = current.position;
        if current.position == maze.exit {
            break (current, seen);
        } else {
            if !seen.contains(&current.position) {
                let neighbours: Vec<Box<Node>> = maze.get_neighbours(current.position).iter()
                    .map
                (|pos|
                    Box::new(Node {
                        parent: Some(Rc::new(current.to_owned())),
                        position: *pos
                })).collect();
                frontier.append(&mut VecDeque::from(neighbours));
                traversed.push(current);
            }
        }
        seen.insert(position);
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    parent: Option<Rc<Node>>,
    position: crate::entities::Position,
}

impl std::hash::Hash for Node {

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.row.hash(state);
        self.position.column.hash(state);
    }
}

impl std::cmp::PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for Node {}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.parent == None {
            write!(f, "{}", self.position)
        } else {
            write!(f, "{} <- {}", self.position, self.parent.as_ref().expect("Node expected"))
        }

    }
}