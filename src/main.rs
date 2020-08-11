mod algorithms;

use std::fs;

mod entities;
mod maze_reader;

fn main() {
    println!("Running maze");
    let filename = "maze1.txt";

    let maze = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let maze = maze_reader::create(maze);
    println!("{}", maze);
    let (exit_node, seen) = algorithms::dfs(&maze);
    println!("Path: {}", exit_node);
    println!("Seen: {:?}", seen);
    
}