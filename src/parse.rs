
use crate::Conway;

// 1. get first line
//      - split into rows, cols
// 1.5 init cells
// 2. for each next line
//      - for each char cells.push
//            - '.' => false
//            - 'O' => true

// validations
// 0.9. ensure there is at least 1 row/line
// 0.99 ensure that line 1 parses to `rows` and `cols`
// 1. given rows, cols
//      - ensure there `rows` number of rows
//      - ensure every row has `cols` cells
//

pub fn parse(input: &str) -> Option<Conway> {
    let lines = input.split('\n');
    let lines: Vec<_> = lines.collect();

    if lines.is_empty() {
        return None;
    }

    let first = &lines[0];

    // step 1
    let dimension_fields = first.split('x').collect::<Vec<_>>();
    let [rows, cols, ..] = dimension_fields.as_slice() else {
        return None;
    };

    let rows: usize = rows.parse().ok()?;
    let cols: usize = cols.parse().ok()?;

    if rows == 0 || cols == 0 {
        return None;
    }

    // step 1.5
    let mut cells = Vec::with_capacity(rows * cols);
    let rest = &lines[1..];

    if rest.len() != rows {
        return None;
    }

    // step 2
    for line in rest {
        if line.len() != cols {
            return None;
        }
        for c in line.chars() {
            cells.push(match c {
                '.' => false,
                'X' => true,
                _ => return None,
            })
        }
    }

    //Return a conway
    Some(Conway {
        cells,
        rows,
        cols,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("./parse/input1.wc");
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
        let conway = parse(INPUT).unwrap();

        assert_eq!(conway.get(0, 0), false);
        assert_eq!(conway.get(0, 1), true);
        assert_eq!(conway.get(0, 2), false);
        assert_eq!(conway.get(1, 0), true);
        assert_eq!(conway.get(1, 1), false);
        assert_eq!(conway.get(1, 2), true);
    }

    #[test]
    fn parse_doesnt_panic() {
        assert!(parse("").is_none());
        assert!(parse("2x2\n.X\n.").is_none());
        assert!(parse("2x2\n.X").is_none());
        assert!(parse("2x2\n.X\n.T").is_none());
        assert!(parse("rggn3434j309J)Q#*@$").is_none());
    }
}
