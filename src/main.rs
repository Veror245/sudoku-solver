
use std::{collections::HashMap};

fn main() {

    let board = [
        [7, 0, 8, 5, 0, 1, 0, 0, 0],
        [0, 0, 9, 0, 0, 0, 6, 1, 5],
        [1, 0, 5, 0, 0, 0, 0, 8, 0],
        [5, 0, 0, 0, 3, 0, 0, 0, 7],
        [0, 0, 0, 1, 0, 2, 0, 0, 0],
        [9 ,0, 0, 0, 8, 0, 0, 0, 1],
        [0, 4, 0, 0, 0, 0, 9, 0, 2],
        [0, 9, 3, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 6, 0, 9, 4, 0, 8]
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
    
}


#[derive(Debug)]
pub struct Sudoku {
    board: [[u8; 9]; 9],
    is_solved: bool
}

impl Sudoku {
    pub fn new(board: [[u8; 9]; 9]) -> Self {
        Self { board: board, is_solved: false }
    }

    pub fn get_len(&self) -> (usize, usize) {
        let row_size = self.board.len();
        let col_size = self.board[0].len();

        (row_size, col_size)
    }

    pub fn check_valid(&self) -> bool {
        let (m, n) = self.get_len();

       
        for i in 0..m{
            let mut row_map = HashMap::new();
            let mut col_map = HashMap::new();
            for j in 0..n {
                if self.board[i][j] != 0{
                    let rc = row_map.entry(self.board[i][j]).or_insert(0);
                    *rc += 1;
                    if *rc > 1 {return false}
                }
                if self.board[j][i] != 0{
                    let cc = col_map.entry(self.board[j][i]).or_insert(0);
                    *cc += 1;
                    if *cc > 1 {return  false}
                }     
            }  
        }


        for i in (0..m).step_by(3) {
            for j in (0..n).step_by(3) {
                let mut freq_map = HashMap::new();
                let q1 = &self.board[j][i..i+3];
                let q2 = &self.board[j+1][i..i+3];
                let q3 = &self.board[j+2][i..i+3];
                let r = [q1, q2, q3];
                for k in r {
                    for l in k {
                        if *l != 0 {
                            let count = freq_map.entry(l).or_insert(0);
                            *count += 1;
                            match count {
                                1 => continue,
                                _ => return false
                            }
                        }
                    }
                }
            }
        }

        true
    }

    fn is_filled(&self) -> bool {
        
        let (m, n) = self.get_len();

        for i in 0..m {
            for j in 0..n {
                if self.board[i][j] == 0 {
                    return false;
                }
            }
        }

        true
    }

    pub fn solve(&mut self) -> bool {
        
        let board = self.board;
        self.is_solved = self.backtrack(0, 0, &board);
        self.is_solved 
    }

    fn backtrack(&mut self, a:usize, b:usize, board: &[[u8; 9]; 9]) -> bool {

        let curr = self.board[a][b];

        if (a, b) == (8,8) && self.check_valid() == true {
            return true;
        } else {
            if board[a][b] == 0 {
                for v in curr+1..=9 {
                    self.board[a][b] = v;
                    if self.check_valid() == false && v < 9 {continue;}
                    else if self.check_valid() == false && v >=9 {
                        self.board[a][b] = 0;
                        return false;
                    }
                    else {
                        if b+1 <= 8 {
                            if self.backtrack(a, b+1, board) == false {
                                
                                continue;
                            } else {
                                
                                return true;
                            }
                        }
                        else {
                            if self.backtrack(a+1, 0, board) == false {
                                
                                continue; 
                            }else {
                                
                                return true;
                            }

                        }
                        
                    }
                }       
            } else {
                if b+1 <= 8 {
                    
                    if self.backtrack(a, b+1, board) == false {
                        
                        return false;
                    }
                    else {return true;}
                } else {
                   
                    if self.backtrack(a+1, 0, board) == false{
                        
                        return false
                    }
                    else {return true;}
                }
            }

        }
        self.board[a][b] = 0;
        false
    }   
}