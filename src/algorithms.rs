use std::collections::{HashSet, VecDeque};
use std::hash::Hasher;
use std::rc::Rc;
use std::fmt;
use crate::entities::{Maze, Position};

pub trait Frontier {
    fn push(&mut self, value: Box<Node>);
    fn append(&mut self, other: &mut VecDeque<Box<Node>>);
    fn pop(&mut self) -> Option<Box<Node>>;
    fn is_empty(&mut self) -> bool;
}

pub struct Queue {
    vector: VecDeque<Box<Node>>
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            vector: VecDeque::new()
        }
    }
}

impl Frontier for Queue {
    fn push(&mut self, value: Box<Node>) {
        self.vector.push_back(value)
    }

    fn append(&mut self, other: &mut VecDeque<Box<Node>>) {
        self.vector.append(other)
    }

    fn pop(&mut self) -> Option<Box<Node>> {
        self.vector.pop_back()
    }

    fn is_empty(&mut self) -> bool {
        self.vector.is_empty()
    }
}

pub struct Stack {
    vector: VecDeque<Box<Node>>
}

impl Frontier for Stack {
    fn push(&mut self, value: Box<Node>) {
        self.vector.push_back(value)
    }

    fn append(&mut self, other: &mut VecDeque<Box<Node>>) {
        self.vector.append(other)
    }

    fn pop(&mut self) -> Option<Box<Node>> {
        self.vector.pop_front()
    }

    fn is_empty(&mut self) -> bool {
        self.vector.is_empty()
    }
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            vector: VecDeque::new()
        }
    }
}

pub fn dfs(maze: &Maze) -> (Node, HashSet<Position>) {
    search(maze, Queue::new())
}

pub fn bfs(maze: &Maze) -> (Node, HashSet<Position>) {
    search(maze, Stack::new())
}

fn search<T: Frontier>(maze: &Maze, mut frontier: T) -> (Node, HashSet<Position>) {
    let mut explored: HashSet<Position> = HashSet::new();

    let parent = Node {
        parent: None,
        position: maze.start
    };
    frontier.push(Box::new(parent));

    loop {
        if frontier.is_empty() {
            panic!("No solution found!")
        }
        let current: Node = *frontier.pop().expect("There must be a node here");
        let position = current.position;
        if current.position == maze.exit {
            break (current, explored);
        } else {
            if !explored.contains(&current.position) {
                let neighbours: Vec<Box<Node>> = maze.get_neighbours(current.position).iter()
                    .map
                    (|pos|
                        Box::new(Node {
                            parent: Some(Rc::new(current.to_owned())),
                            position: *pos
                        })).collect();
                frontier.append(&mut VecDeque::from(neighbours));
            }
        }
        explored.insert(position);
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub parent: Option<Rc<Node>>,
    pub position: crate::entities::Position,
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