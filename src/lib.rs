#[derive(Debug, Default, PartialEq)]
pub struct Conway {
    // what goes here?
    pub cells: Vec<bool>,
    pub rows: usize,
    pub cols: usize,
}

impl Conway {
    pub fn get(&self, row: usize, col: usize) -> bool {
        // row * cols + col
        let index = row * self.cols + col;
        self.cells[index]
    }

    pub fn set(&mut self, row: usize, col: usize, val: bool) {
        let index = row * self.cols + col;
        self.cells[index] = val;
    }

    pub fn update(&self) -> Conway {
        
        let mut new = Conway { rows: self.rows, cols: self.cols, cells: vec![false; self.cells.len()]};

        for row in 0..self.rows {
            for col in 0..self.cols {
                let alive = self.should_live(row, col);
                new.set(row, col, alive);
            }
        }


        new
    }
    // RULES
    // Any live cell with fewer than two live neighbours dies, as if by underpopulation.
    // Any live cell with two or three live neighbours lives on to the next generation.
    // Any live cell with more than three live neighbours dies, as if by overpopulation.
    // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    fn should_live(&self, row: usize, col: usize) -> bool {
        let neighbors =  self.count_live_neighbors(row, col);
        let was_living = self.get(row, col);

        if was_living {
            neighbors == 2 || neighbors == 3
        } else {
            neighbors == 3
        }
        
    }

    fn count_live_neighbors(&self, row: usize, col: usize) -> usize {
        assert!(self.rows > 0);
        assert!(self.cols > 0);

        // generating coords
        let neighbor_coords = match self.detect_kind(row, col) {
            CellKind::TopLeft => vec![(0, 1), (1, 1), (1, 0)],
            CellKind::Top => vec![
                (0, col - 1),
                (0, col + 1),
                (1, col - 1),
                (1, col),
                (1, col + 1),
            ],
            CellKind::TopRight => vec![
                (0, self.col_max() - 1),
                (1, self.col_max() - 1),
                (1, self.col_max()),
            ],
            CellKind::Right => vec![
                (row - 1, col),
                (row + 1, col),
                (row - 1, col - 1),
                (row, col - 1),
                (row + 1, col - 1),
            ],
            CellKind::BottomRight => vec![(row - 1, col), (row - 1, col - 1), (row, col - 1)],
            CellKind::Bottom => vec![
                (row, col - 1),
                (row, col + 1),
                (row - 1, col - 1),
                (row - 1, col),
                (row - 1, col + 1),
            ],
            CellKind::BottomLeft => vec![(row - 1, col), (row - 1, col + 1), (row, col + 1)],
            CellKind::Left => vec![
                (row - 1, 0),
                (row + 1, 0),
                (row - 1, 1),
                (row, 1),
                (row + 1, 1),
            ],
            CellKind::Middle => vec![
                (row - 1, col - 1),
                (row - 1, col),
                (row - 1, col + 1),
                (row, col + 1),
                (row + 1, col + 1),
                (row + 1, col),
                (row + 1, col - 1),
                (row, col - 1),
            ],
        };

        neighbor_coords
            .into_iter()
            .fold(0, |count, (row, col)| count + (self.get(row, col) as usize))
    }

    pub fn all_dead(&self) -> bool {
        self.cells.iter().all(|b| !*b)
    }

    fn detect_kind(&self, row: usize, col: usize) -> CellKind {
        match (row, col) {
            (0, 0) => CellKind::TopLeft,
            (0, c) if c == self.col_max() => CellKind::TopRight,
            (r, 0) if r == self.row_max() => CellKind::BottomLeft,
            (r, c) if r == self.row_max() && c == self.col_max() => CellKind::BottomRight,
            (0, _) => CellKind::Top,
            (r, _) if r == self.row_max() => CellKind::Bottom,
            (_, 0) => CellKind::Left,
            (_, c) if c == self.col_max() => CellKind::Right,
            _ => CellKind::Middle,
        }
    }

    const fn col_max(&self) -> usize {
        self.cols - 1
    }
    const fn row_max(&self) -> usize {
        self.rows - 1
    }


     pub fn to_string(&self, turn: usize) -> String {
        let mut s = String::new();

        s.push_str(&format!("Current Turn: {turn}\n"));
        s.push_str(&"=".repeat(self.cols + 2));
        s.push('\n');

        for row in 0..self.rows {
            s.push('|');
            for col in 0..self.cols {
                let c = match self.get(row, col) {
                    true => 'O',
                    false => '.',
                };
                s.push(c);

            }
            s.push_str("|\n");
        }

        s.push_str(&"=".repeat(self.cols + 2));
        s.push('\n');

        s
    }
}


enum CellKind {
    TopLeft, //
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    Middle,
}

pub fn parse(input: &str) -> Conway {
    let lines = input.split('\n');
    let lines: Vec<_> = lines.collect();

    let [first, second, ..] = lines.as_slice() else {
        panic!("Help");
    };

    let dimension_fields = first.split('x').collect::<Vec<_>>();
    let [rows, cols, ..] = dimension_fields.as_slice() else {
        panic!("Help")
    };

    let rows: usize = rows.parse().unwrap();
    let cols: usize = cols.parse().unwrap();
    assert!(rows > 0);
    assert!(cols > 0);

    // Turn second string into a vec<bool>
    let cells: Vec<bool> = second
        .chars()
        .map(|char| -> bool { match char {
            '.' => false,
            'O' => true,
            _ => panic!("unknown life form")
        }})
        .collect();

    //Return a conway
    Conway {
        cells: cells,
        rows: rows,
        cols: cols,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "2x3\n.O.O.O";
    // oxo
    // xox
    // becomes
    // oxo
    // oxo
    // becomes
    // ooo
    // ooo

    #[test]
    fn parse_simple() {
        let conway = parse(INPUT);

        assert_eq!(conway.get(0, 0), false);
        assert_eq!(conway.get(0, 1), true);
        assert_eq!(conway.get(0, 2), false);
        assert_eq!(conway.get(1, 0), true);
        assert_eq!(conway.get(1, 1), false);
        assert_eq!(conway.get(1, 2), true);
    }

    #[test]
    fn step_simple() {
        let conway = parse(INPUT);

        let c = conway.update();
        assert_eq!(c, parse("2x3\n.O..O."));

        let c2 = c.update();
        assert_eq!(c2, parse("2x3\n......"));
    }
}

// text
// single
// standard rules
// borders: closed, then wrapping, then infinite???

// RULES
// Any live cell with fewer than two live neighbours dies, as if by underpopulation.
// Any live cell with two or three live neighbours lives on to the next generation.
// Any live cell with more than three live neighbours dies, as if by overpopulation.
// Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

// rows x columns
// <cells as bits on or off in one line>
// example below
//
// 3x3
// 010101010
// (0,0),(0,1),(0,2),(1,0),(1,1),(1,2)
// 0,1,2,3,4,5
// row|row|row
// 3 cells per row
//  rc = p
