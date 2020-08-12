mod algorithms;

use std::{fs, fmt};
use colored::*;
use crate::entities::{Maze, Position};
use std::collections::HashSet;
use std::rc::Rc;
use clap::{Arg, App};
use crate::Algorithm::{DFS, BFS};

mod entities;
mod maze_reader;

#[derive(Debug)]
enum Algorithm {
    DFS,
    BFS,
}

impl std::str::FromStr for Algorithm {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DFS" => Ok(DFS),
            "BFS" => Ok(BFS),
            _ => Err(format!("'{}' is not a valid value for Algorithm", s)),
        }
    }
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() {
    let matches = App::new("Deep and Breadth First Search in Rust")
        .version("0.1")
        .author("Manuel Doncel Martos. <manueldoncelmartos@gmail.com>")
        .arg(Arg::with_name("maze")
            .short("m")
            .long("maze")
            .value_name("FILE")
            .help("Sets the maze to be solved")
            .takes_value(true))
        .arg(Arg::with_name("algorithm")
            .short("a")
            .long("algorithm")
            .value_name("algorithm")
            .help("Sets the algorithm to be used.")
            .possible_value("dfs")
            .possible_value("bfs")
            .takes_value(true))
        .get_matches();

    let filename = matches.value_of("maze").unwrap_or("./maze1.txt");
    let algorithm: Algorithm = matches.value_of("algorithm").unwrap_or("dfs").to_uppercase()
        .parse().unwrap();
    println!("Running maze {} with algorithm {}", filename.yellow().bold(), algorithm.to_string
    ().green().bold());

    let maze = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let maze = maze_reader::create(maze);
    println!("Maze to solve:\r\n{}", maze);
    let (exit_node, seen) = match algorithm {
        DFS => algorithms::dfs(&maze),
        BFS => algorithms::bfs(&maze)
    };
    let path = if let Some(exit_node) = exit_node {
        println!("{}", "Solution found \u{263A}!".green().bold());
        let mut path = HashSet::new();
        let mut current_node = Box::new(exit_node);
        while current_node.parent != None {
            let parent = Rc::try_unwrap(current_node.parent.expect("")).expect("");
            path.insert(parent.position);
            current_node = Box::new(parent);
        }
        path
    } else {
        println!("{}", "No solution found!".red().bold());
        HashSet::new()
    };
    let maze_solution = MazeSolution {
        maze: Box::new(maze),
        path,
        seen
    };
    println!("{}", maze_solution);

}

struct MazeSolution {
    maze: Box<Maze>,
    path: HashSet<Position>,
    seen: HashSet<Position>
}

impl fmt::Display for MazeSolution {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let _write = write!(f, "");
        for (i, row) in self.maze.grid.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                let _write = if c == &false {
                    _write.and( write!(f, "{}", "#".blue().bold().on_blue()))
                } else if self.maze.start == Position::of(i as u8, j as u8) {
                    _write.and( write!(f, "{}", "@".yellow().bold().on_green()))
                } else if self.maze.exit == Position::of(i as u8, j as u8) {
                    _write.and( write!(f, "{}", "X".red().bold().on_green()))
                } else {
                    let position = Position::of(i as u8, j as u8);
                    if self.seen.contains(&position) && !self.path.contains
                    (&position) {
                        _write.and(write!(f, "{}", " ".on_red()))
                    } else if self.path.contains(&position) {
                        _write.and( write!(f, "{}", "\u{00B7}".on_green()))
                    } else {
                        _write.and( write!(f, "{}", " ".on_white()))
                    }
                };
            }
            let _write = _write.and( write!(f, "\r\n"));
        }
        _write
    }
}