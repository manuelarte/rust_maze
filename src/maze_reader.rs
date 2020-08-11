pub fn create(string: String) -> crate::entities::Maze {
    let mut start: Option<crate::entities::Position> = None;
    let mut exit: Option<crate::entities::Position> = None;
    let mut grid: Vec<Vec<bool>> = Vec::new();
    let mut i = 0;
    for s in string.split("\r\n") {
        let mut j = 0;
        let mut v: Vec<bool> = Vec::new();
        for c in s.chars() {
            let val = if c == '#' { false } else { true };
            v.push(val);
            if c == '@' {
                start = Some(crate::entities::Position::of(i, j));
            }
            if c == 'X' {
                exit = Some(crate::entities::Position::of(i, j));
            }
            j += 1; 
        }
        grid.push(v);
        i += 1;
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