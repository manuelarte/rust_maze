use std::fmt;
use std::hash::Hasher;

pub struct Maze {
    pub grid: Vec<Vec<bool>>,
    pub start: Position,
    pub exit: Position
}

impl Maze {
    pub fn get_neighbours(&self, position: Position) -> Vec<Position> {
        let mut children: Vec<Position> = Vec::new();
        let row_start_index: isize = if position.row == 0 { 0 } else { -1 };
        let row_end_index = if usize::from(position.row) == self.grid.len()-1 { 1 } else { 2 };
        let column_start_index: isize = if position.column == 0 { 0 } else { -1 };
        let column_end_index = if usize::from(position.column) < self.grid[usize::from(position
            .row)].len()-1 { 2 } else
        { 1 };
        for i in row_start_index..row_end_index {
            let row = isize::from(position.row) + i;
            for j in column_start_index..column_end_index {
                let column = isize::from(position.column) + j;
                if i != 0 && j == 0 || (i == 0 && j != 0) {
                    if self.grid[row as usize][column as usize] {
                        children.push(Position::of(row as u8, column as u8))
                    }
                }
            }
            
        }    
        children
    }
}

impl fmt::Display for Maze {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        let mut string = String::from("");
        let mut i = 0;
        for row in &self.grid {
            let mut j = 0;
            for c in row {
                let r = if c == &false { 
                    '#' 
                } else if self.start == Position::of(i, j) {
                    'S'
                } else if self.exit == Position::of(i, j) {
                    'E'
                } else { 
                    ' ' 
                };

                string.push(r);
                j += 1;
            }
            i += 1;
            string.push_str("\n");
        }
        write!(f, "{}", string)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub row: u8,
    pub column: u8
}

impl Position {
    pub fn of(i: u8, j: u8) -> Position {
        Position {
            row: i,
            column: j
        }
    }
}

impl PartialEq for Position {

    fn eq(&self, other: &Position) -> bool { 
        self.row == other.row && self.column == other.column
    }
}

impl Eq for Position {}

impl std::hash::Hash for Position {

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.column.hash(state);
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.row, self.column)
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn it_get_neighbours_in_0_0() {
        let v0 = vec![false, false, true, false];
        let v1 = vec![false, true, true, true];
        let v2 = vec![false, true, false, false];
        let v3 = vec![false, true, false, false];
        let grid = vec![v0, v1, v2, v3];
        let start = Position::of(0, 2);
        let exit = Position::of(3, 1);
        let maze = Maze {
            grid,
            start,
            exit
        };
        assert_eq!(1, maze.get_neighbours(Position::of(0, 2)).len());
        assert_eq!(3, maze.get_neighbours(Position::of(1, 2)).len());
        let neighbours = maze.get_neighbours(Position::of(1, 3));
        println!("{:?}", neighbours);
        assert_eq!(1, maze.get_neighbours(Position::of(1, 3)).len());
    }

}