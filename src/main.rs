
use sudoku_solver::{solver::Sudoku, euler_parser::parse};
use std::time::Instant;

fn main() {

  let board: [[u8; 9]; 9] = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 9, 0, 0, 1, 0, 0, 3, 0],
        [0, 0, 6, 0, 2, 0, 7, 0, 0],
        [0, 0, 0, 3, 0, 4, 0, 0, 0],
        [2, 1, 0, 0, 0, 0, 0, 9, 8],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 2, 5, 0, 6, 4, 0, 0],
        [0, 8, 0, 0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0]
    ];

    

    // let mut board = [[0; 9]; 9];
    // board[8][0] = 1;
    // board[8][2] = 2;

    let mut sudoku = Sudoku::new(board);
    println!("{}", sudoku.check_valid());
    println!("{:?}", sudoku.get_min_candidate_count());

    let (m, n) = sudoku.get_len();

    for i in 0..m {
        for j in 0..n {
            print!("{} ",sudoku.board[i][j]);
        }
        println!();
    }

    println!("--------------");
    println!(" ");

    let start = Instant::now();
    if sudoku.solve() == true {
        println!("");
        for i in 0..m {
            for j in 0..n {
                print!("{} ",sudoku.board[i][j]);
            }
            
            println!("--");
        }
    }
    println!("Time taken to solve normal sudoku: {:?}", start.elapsed());

    euler_solve();

    
    
    
}



fn euler_solve() {

    let mut three: i32 = 0;
    let mut _c = 0;


    let res = parse("./data/sudoku.txt").unwrap();
    let start = Instant::now();
    for s in res.boards {
        //c += 1;
        let mut sudoku = Sudoku::new(s);
        if sudoku.solve() == true {
            three += (sudoku.board[0][0] as i32)*100 + (sudoku.board[0][1] as i32) * 10 + sudoku.board[0][2] as i32;
            continue;
        }
    }
    println!("Time taken to solve eulers 96 50 sudokus: {:?}", start.elapsed());

    println!("final Sum: {}", three);

}
