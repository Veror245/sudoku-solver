
use sudoku_solver::{solver::Sudoku, euler_parser::parse};
use std::time::Instant;
use std::fs;

fn main() {

//     let board: [[u8; 9]; 9] = [
//         [0, 0, 0, 0, 0, 0, 0, 0, 0],
//         [0, 9, 0, 0, 1, 0, 0, 3, 0],
//         [0, 0, 6, 0, 2, 0, 7, 0, 0],
//         [0, 0, 0, 3, 0, 4, 0, 0, 0],
//         [2, 1, 0, 0, 0, 0, 0, 9, 8],
//         [0, 0, 0, 0, 0, 0, 0, 0, 0],
//         [0, 0, 2, 5, 0, 6, 4, 0, 0],
//         [0, 8, 0, 0, 0, 0, 0, 1, 0],
//         [0, 0, 0, 0, 0, 0, 0, 0, 0]
//     ];

//     let board = [
//          [1, 0, 0, 0, 0, 7, 0, 9, 0],
//     [0, 3, 0, 0, 2, 0, 0, 0, 8],
//     [0, 0, 9, 6, 0, 0, 5, 0, 0],
//     [0, 0, 5, 3, 0, 0, 9, 0, 0],
//     [0, 1, 0, 0, 8, 0, 0, 0, 2],
//     [6, 0, 0, 0, 0, 4, 0, 0, 0],
//     [3, 0, 0, 0, 0, 0, 0, 1, 0],
//     [0, 4, 0, 0, 0, 0, 0, 0, 7],
//     [0, 0, 7, 0, 0, 0, 3, 0, 0]
//     ];

    

//     // let mut board = [[0; 9]; 9];
//     // board[8][0] = 1;
//     // board[8][2] = 2;

//     let mut sudoku = Sudoku::new(board);
//     println!("{}", sudoku.check_valid());
//     println!("{:?}", sudoku.get_min_candidate_count());

//     let (m, n) = sudoku.get_len();

//     for i in 0..m {
//         for j in 0..n {
//             print!("{} ",sudoku.board[i][j]);
//         }
//         println!();
//     }

//     println!("--------------");
//     println!(" ");

//     sudoku.recursive_calls = 0;

//     let start = Instant::now();
//     if sudoku.solve("bt") == true {
//         // println!("");
//         // for i in 0..m {
//         //     for j in 0..n {
//         //         print!("{} ",sudoku.board[i][j]);
//         //     }
            
//         //     println!("--");
//         // }
//     }
//     println!("Time taken to solve normal sudoku with backtrack: {:?} with {} calls with {} validity_checks", start.elapsed(), 
//     sudoku.recursive_calls, sudoku.validity_checks);

//   let board = [
//          [1, 0, 0, 0, 0, 7, 0, 9, 0],
//     [0, 3, 0, 0, 2, 0, 0, 0, 8],
//     [0, 0, 9, 6, 0, 0, 5, 0, 0],
//     [0, 0, 5, 3, 0, 0, 9, 0, 0],
//     [0, 1, 0, 0, 8, 0, 0, 0, 2],
//     [6, 0, 0, 0, 0, 4, 0, 0, 0],
//     [3, 0, 0, 0, 0, 0, 0, 1, 0],
//     [0, 4, 0, 0, 0, 0, 0, 0, 7],
//     [0, 0, 7, 0, 0, 0, 3, 0, 0]
//     ];

//     let mut sudoku = Sudoku::new(board);
    


//     let start = Instant::now();
//     if sudoku.solve("opt_mrv") == true {
//         // println!("");
//         // for i in 0..m {
//         //     for j in 0..n {
//         //         print!("{} ",sudoku.board[i][j]);
//         //     }
            
//         //     println!("--");
//         // }
//     }
//     println!("Time taken to solve normal sudoku with optimsed mrv: {:?} with {} calls with {} validity_checks", start.elapsed(), 
//     sudoku.recursive_calls, sudoku.validity_checks);

//     euler_solve("bt");
//     euler_solve("opt_mrv");

    let boards = load_data();
    let start = Instant::now();

    for board in &boards[..100] {
        let mut sudoku = Sudoku::new(*board);
        sudoku.solve("bt");
    }

    println!("Backtracking 100 puzzles took {:?}", start.elapsed());


    let boards = load_data();
    let start = Instant::now();

    for board in &boards[..100] {
        let mut sudoku = Sudoku::new(*board);
        sudoku.solve("mrv");
    }

    println!("Mrv 100 puzzles took {:?}", start.elapsed());


    let boards = load_data();
    let start = Instant::now();

    for board in &boards[..100] {
        let mut sudoku = Sudoku::new(*board);
        sudoku.solve("opt_mrv");
    }

    println!("Optimised Mrv 100 puzzles took {:?}", start.elapsed());
    
    
}



fn euler_solve(mode: &str) {

    let mut three: i32 = 0;
    let mut _c = 0;


    let res = parse("./data/sudoku.txt").unwrap();
    if mode == "bt" {
        let start = Instant::now();
        for s in res.boards {
            //c += 1;
            let mut sudoku = Sudoku::new(s);
            if sudoku.solve("bt") == true {
                three += (sudoku.board[0][0] as i32)*100 + (sudoku.board[0][1] as i32) * 10 + sudoku.board[0][2] as i32;
                continue;
            }
        }
        println!("Time taken to solve eulers 96 50 sudokus with normal backtrack: {:?}", start.elapsed());

        println!("final Sum: {}", three);

    } else {
        let start = Instant::now();
        for s in res.boards {
            //c += 1;
            let mut sudoku = Sudoku::new(s);
            if sudoku.solve("opt_mrv") == true {
                three += (sudoku.board[0][0] as i32)*100 + (sudoku.board[0][1] as i32) * 10 + sudoku.board[0][2] as i32;
                continue;
            }
        }
        println!("Time taken to solve eulers 96 50 sudokus with mrv: {:?}", start.elapsed());

        println!("final Sum: {}", three);
    }
    

   

}

fn load_data() -> Vec<[[u8; 9]; 9]> {
    let contents = fs::read_to_string("./data/hardest_puzzles")
    .expect("Failed to read puzzle file");

    let mut boards: Vec<[[u8; 9]; 9]> = Vec::new();

    for line in contents.lines() {
        if line.trim().is_empty() {
            continue;
        }

        assert_eq!(line.len(), 81);

        let mut board = [[0u8; 9]; 9];

        for (i, ch) in line.chars().enumerate() {
            let row = i / 9;
            let col = i % 9;

            board[row][col] = match ch {
                '.' => 0,
                d => d.to_digit(10).unwrap() as u8,
            };
        }

        boards.push(board);
    }

    println!("Loaded {} puzzles", boards.len());

    boards
}
