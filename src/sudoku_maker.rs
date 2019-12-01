use std::env;
use std::fs::File;

fn new_board(diff: Difficulty) -> String {
    let mut board = [[usize; 9]; 9]
}

enum Difficulty {
    EASY,
    MED,
    HARD,
}

struct Sudoku{
    board: [[usize; 9]; 9],
}

impl Sudoku {
    fn new(diff:Difficulty) -> Sudoku {
        let mut val:[[usize; 9]; 9] = [[0;9]; 9];
        match diff {
            Difficulty::EASY => {
                
            },
        }
    }

    /// Returns one of the sudoku "boxes"
    ///
    /// # Arguments
    /// * `i` - 1 | 2 | 3
    ///         4 | 5 | 6
    ///         7 | 8 | 9
    fn get_box<'a>(&'a mut self, box_num: usize) -> &'a mut [& [usize]; 3] {
        let j:[&[usize]; 3] = [ &[0, 0, 0], &[0, 0, 0], &[0, 0, 0] ];
        let mut counter = 0;
        let rownum = (box_num - 1) / 3;
        let colnum = (box_num - 1) % 3;
        for row in &(self.board)[rownum..rownum + 3] {
            j[counter] = &row[colnum..colnum + 3];
        }
        j
    }
}
