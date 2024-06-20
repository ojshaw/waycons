use std::collections::HashSet;

#[derive(Debug)]
pub struct Conway {
    // what goes here?
    pub cells: Vec<bool>,
    pub rows: usize,
    pub cols: usize
}

impl Conway {
    pub fn get(&self, row: usize, col: usize) -> bool {
        todo!()
    }
}

pub fn parse(input: &str) -> Conway {
    let lines = input.split('\n');
    let lines: Vec<_>  = lines.collect();

    let [first, second, ..] = lines.as_slice() else {
        panic!("Help");
    };

    let [rows, col, ..] = first.split('x').collect::<Vec<_>>().as_slice() else {
        panic!("Help")
    };

    
    todo!()
}

// pub Result<T, E> {
//     Ok(T),
//     Err(E)
// }


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: & 'static str =  "3x3\n010101010";

    #[test]
    fn parse_simple() {
        let conway = parse(INPUT);

        assert_eq!(conway.get(0,0), false);
        assert_eq!(conway.get(0,1), true);
        assert_eq!(conway.get(0,2), false);
        assert_eq!(conway.get(1,0), true);
        assert_eq!(conway.get(1,1), false);
        assert_eq!(conway.get(1,2), true);
        assert_eq!(conway.get(2,0), false);
        assert_eq!(conway.get(2,1), true);
        assert_eq!(conway.get(2,2), false);
        // assert_eq!(result, 4);
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


// 3x3 
// 010101010