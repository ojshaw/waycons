use std::collections::HashSet;

#[derive(Debug)]
pub struct Conway {
    // what goes here?
    pub cells: Vec<bool>,
    pub rows: usize,
    pub cols: usize,
}

impl Conway {
    pub fn get(&self, row: usize, col: usize) -> bool {
        // row * cols + col
        let index = row  * self.cols + col;
        self.cells[index]
    }
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

    // Turn second string into a vec<bool>
    let cells: Vec<bool> = second
        .chars()
        .map(|char| -> bool { char.to_digit(2).unwrap() != 0 })
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

    const INPUT: &'static str = "2x3\n010101";

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
}

// text
// single
// standard rules
// wrap around the edges

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
