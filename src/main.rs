


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
    dbg!(&sudoku);

    // let m = &sudoku.board.len(); // no of rows
    // let n = &sudoku.board[0].len(); //no of columns in the first row

    let (m, n) = sudoku.get_len();

    for i in 0..m {
        for j in 0..n {
            print!("{} ",sudoku.board[i][j]);
        }
        println!();

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
}