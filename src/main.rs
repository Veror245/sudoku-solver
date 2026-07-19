
use sudoku_solver::{solver::Sudoku, euler_parser::parse};

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

    let mut sudoku = Sudoku::new(board);
    println!("{}", sudoku.check_valid());

    let (m, n) = sudoku.get_len();

    for i in 0..m {
        for j in 0..n {
            print!("{} ",sudoku.board[i][j]);
        }
        println!();
    }

    println!("--------------");
    println!(" ");

    if sudoku.solve() == true {
        println!("");
        for i in 0..m {
            for j in 0..n {
                print!("{} ",sudoku.board[i][j]);
            }
            
            println!("--");
        }
    }

    let mut three: i32 = 0;
    let mut c = 0;

    let res = parse("../data/sudoku.txt").unwrap();
    for s in res.boards {
        c += 1;
        println!("Solving Sudoku number: {}", c);
        let mut sudoku = Sudoku::new(s);
        if sudoku.solve() == true {
            three += (sudoku.board[0][0] as i32)*100 + (sudoku.board[0][1] as i32) * 10 + sudoku.board[0][2] as i32;
            continue;
        }
    }

    println!("final Sum: {}", three);
    
}




