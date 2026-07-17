
use std::collections::HashMap;

fn main() {
    // let mut board: [[u8; 9]; 9] = [[0; 9]; 9];

    let board = [
        [0, 0, 0, 0, 7, 0, 0, 0, 6],
        [4, 0, 0, 2, 0, 0, 8, 0, 0],
        [0, 9, 2, 8, 0, 0, 0, 0, 0],
        [9, 0, 5, 0, 0, 4, 0, 0, 0],
        [0, 4, 0, 0, 3, 0, 0, 6, 0],
        [0 ,0, 0, 6, 0, 0, 4, 0, 7],
        [0, 0, 0, 0, 0, 7, 5, 1, 0],
        [0, 0, 8, 0, 0, 1, 0, 0, 4],
        [5, 0, 0, 0, 9, 0, 0, 0, 0]
    ];

    let sudoku = Sudoku::new(board);
    println!("{}", sudoku.check_valid());

    // let m = &sudoku.board.len(); // no of rows
    // let n = &sudoku.board[0].len(); //no of columns in the first row

    let (m, n) = sudoku.get_len();

    for i in 0..m {
        for j in 0..n {
            print!("{} ",sudoku.board[i][j]);
        }
        println!();

       }

    for i in (1..9).rev() {
        println!("{}", 27%i);
    }
    
}


#[derive(Debug)]
struct Sudoku {
    board: [[u8; 9]; 9],
    is_solved: bool
}

impl Sudoku {
    fn new(board: [[u8; 9]; 9]) -> Self {
        Self { board: board, is_solved: false }
    }

    fn get_len(&self) -> (usize, usize) {
        let row_size = self.board.len();
        let col_size = self.board[0].len();

        (row_size, col_size)
    }

    fn check_valid(&self) -> bool {
        let (m, n) = self.get_len();
        let flag = true;

        for i in 0..m{
            for j in 0..n-1 {
                if (self.board[i][j] == self.board[i][j+1]) && 
                (self.board[i][j] != 0) 
                {return false;}
            }
        }

        for i in 0..m{
            for j in 0..n-1 {
                if (self.board[j][i] == self.board[j+1][i]) && 
                (self.board[j][i] != 0) 
                {return false}
            }
        }

        let mut c = 0;

        //TODO: fix index out of bounds

        while c < 27 {
            let mut freq_map = HashMap::new();

            for i in c..c+3 {
                for j in c..c+3 {
                    if self.board[i][j] != 0 {
                        let count = freq_map.entry(self.board[i][j]).or_insert(0);
                        *count += 1;
                    } 
                }
            }

            for (_k, v) in freq_map {
                if v > 1 {return false}
            }

            c += 3;
        }

      


        flag
    }

}