pub fn create(string: String) -> crate::entities::Maze {
    let mut start: Option<crate::entities::Position> = None;
    let mut exit: Option<crate::entities::Position> = None;
    let mut grid: Vec<Vec<bool>> = Vec::new();
    for (i, s) in string.split("\r\n").enumerate() {
        let mut v: Vec<bool> = Vec::new();
        for (j, c) in s.chars().enumerate() {
            let val = c != '#';
            v.push(val);
            if c == '@' {
                start = Some(crate::entities::Position::of(i as u8, j as u8));
            }
            if c == 'X' {
                exit = Some(crate::entities::Position::of(i as u8, j as u8));
            }
        }
        grid.push(v);
    }

    let start = match start {
        Some(tuple) => tuple,
        _ => panic!("Start position not found"),
    };

    let exit = match exit {
        Some(tuple) => tuple,
        _ => panic!("End position not found"),
    };

    /*
    let mut player;
    let grid: Vec<Vec<bool>> = lines.split("\n")
        .map(|s| 
            s.chars().map(|c| {
                if c == '#' { false } else { true }
            }).collect())
        .collect();
    */

    crate::entities::Maze {
        grid,
        start,
        exit
    }
}