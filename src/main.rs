mod algorithms;

use std::{fs, fmt};
use colored::*;
use crate::entities::{Maze, Position};
use std::collections::HashSet;
use std::rc::Rc;
use clap::{Arg, App};

mod entities;
mod maze_reader;

fn main() {
    let matches = App::new("Deep First Search in Rust")
        .version("0.1")
        .author("Manuel Doncel Martos. <manueldoncelmartos@gmail.com>")
        .about("DFS in Rust")
        .arg(Arg::with_name("maze")
            .short("m")
            .long("maze")
            .value_name("FILE")
            .help("Sets the maze to be solved")
            .takes_value(true))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();

    let filename = matches.value_of("maze").unwrap_or("./maze1.txt");
    println!("{} {}", "Running maze ".green().bold(), filename);

    let maze = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let maze = maze_reader::create(maze);
    println!("{}:\r\n{}", "Maze to solve", maze);
    let (exit_node, seen) = algorithms::dfs(&maze);
    println!("{}", "Solution found!".green().bold());
    println!("Exit Node: {}", exit_node);
    let mut path = HashSet::new();
    let mut current_node = Box::new(exit_node);
    while current_node.parent != None {
        let parent = Rc::try_unwrap(current_node.parent.expect("")).expect("");
        path.insert(parent.position);
        current_node = Box::new(parent);
    }
    let maze_solution = MazeSolution {
        maze: Box::new(maze),
        path,
        seen
    };
    println!("{}", maze_solution)
    
}

struct MazeSolution {
    maze: Box<Maze>,
    path: HashSet<Position>,
    seen: HashSet<Position>
}

impl fmt::Display for MazeSolution {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let mut i = 0;
        let _write = write!(f, "{}", "");
        for row in &self.maze.grid {
            let mut j = 0;
            for c in row {
                let _write = if c == &false {
                    _write.and( write!(f, "{}", "#".blue().bold().on_blue()))
                } else if self.maze.start == Position::of(i, j) {
                    _write.and( write!(f, "{}", "@".yellow().bold().on_green()))
                } else if self.maze.exit == Position::of(i, j) {
                    _write.and( write!(f, "{}", "X".red().bold().on_green()))
                } else {
                    if self.seen.contains(&Position::of(i, j)) && !self.path.contains
                    (&Position::of(i, j)) {
                        _write.and(write!(f, "{}", " ".on_red()))
                    } else if self.path.contains(&Position::of(i, j)) {
                        _write.and( write!(f, "{}", " ".on_green()))
                    } else {
                        _write.and( write!(f, "{}", " ".on_white()))
                    }
                };
                j += 1;
            }
            i += 1;
            let _write = _write.and( write!(f, "{}", "\r\n"));
        }
        _write
    }
}