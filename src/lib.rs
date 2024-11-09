pub mod parse;

#[derive(Debug, Default, PartialEq)]
pub struct Conway {
    // what goes here?
    pub cells: Vec<bool>,
    pub rows: usize,
    pub cols: usize,
    border_kind: BorderKind,
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
        let mut new = Conway {
            rows: self.rows,
            cols: self.cols,
            cells: vec![false; self.cells.len()],
            border_kind: self.border_kind,
        };

        for row in 0..self.rows {
            for col in 0..self.cols {
                let alive = self.should_live(row, col);
                new.set(row, col, alive);
            }
        }

        new
    }

    pub fn all_dead(&self) -> bool {
        self.cells.iter().all(|b| !*b)
    }
    // RULES
    // Any live cell with fewer than two live neighbours dies, as if by underpopulation.
    // Any live cell with two or three live neighbours lives on to the next generation.
    // Any live cell with more than three live neighbours dies, as if by overpopulation.
    // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    fn should_live(&self, row: usize, col: usize) -> bool {
        let neighbors = self.count_live_neighbors(row, col);
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
        // TODO: pattern match on self.border_kind and generate coordinates differently
        // for wrapping games. Use our modular addition helper to get neighboring
        // Need new test cases for wrapping games
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

#[derive(Default, Debug, PartialEq, Copy, Clone)]
enum BorderKind {
    Wrap,
    #[default]
    Nonwrap,
}

fn mod_add(limit: isize, left: isize, to_add: isize) -> isize {
    let raw = dbg!(left + to_add) % limit;
    if raw < 0 {
        raw + limit
    } else {
        raw
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parse::parse;

    const INPUT: &'static str = "2x3\n.X.\nX.X";
    // oxo
    // xox
    // becomes
    // oxo
    // oxo
    // becomes
    // ooo
    // ooo
    #[test]
    fn step_simple() {
        let conway = parse(INPUT).unwrap();

        let c = conway.update();
        assert_eq!(c, parse("2x3\n.X.\n.X.").unwrap());

        let c2 = c.update();
        assert_eq!(c2, parse("2x3\n...\n...").unwrap());
    }

    #[test]
    fn hours() {
        assert_eq!(mod_add(12, 1, 12), 1);
        assert_eq!(mod_add(12, 1, 11), 0);

        assert_eq!(mod_add(12, 1, 6), 7);
        assert_eq!(mod_add(12, 0, -1), 11);
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

// TODO: Conway TUI
// 1. accept conway input at startup (stdin)
//      - if parsing fails, ask for new input,
//                - repeat in a loop until killed, or parsing succeeds
// 2. simulate conway maybe forever
//      - check if everyone is dead, and quit if so?
