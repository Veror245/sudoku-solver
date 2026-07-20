use std::collections::HashMap;


#[derive(Debug)]
pub struct Sudoku {
    pub board: [[u8; 9]; 9],
    pub is_solved: bool
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

    fn _is_filled(&self) -> bool {
        
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

    fn is_placement_valid(&self, a:usize, b:usize, board: &[[u8; 9]; 9]) -> bool {

        let q1: &[u8];
        let q2: &[u8];
        let q3: &[u8];

        let box_row_start = (a/3)*3;
        let box_col_start =(b/3)*3;

        q1 = &board[box_row_start][box_col_start..box_col_start+3];
        q2 = &board[box_row_start+1][box_col_start..box_col_start+3];
        q3 = &board[box_row_start+2][box_col_start..box_col_start+3];

        let mut check = [false; 10];

        let r = [q1, q2, q3];
        for k in r {
            for l in k {
                if *l == 0 {
                    continue;
                }
                let v = *l;
                if check[v as usize] == true {
                    return false;
                }
                check[v as usize] = true;
            }
        }
        
        let mut check = [false; 10];
        for v in board[a] {
            if v == 0 {
                continue;
            }
            if check[v as usize] == true {
                return false;
            }
            check[v as usize] = true;
        }

        let mut check = [false; 10];
        for j in 0..9 {
            let v = board[j][b];
            if v == 0 {
                continue;
            }
            if check[v as usize] == true {
                return false;
            }
            check[v as usize] = true;
        }
        
        true
    }

    fn backtrack(&mut self, a:usize, b:usize, board: &[[u8; 9]; 9]) -> bool {

        let curr = self.board[a][b];

        if (a, b) == (8,8) && self.is_placement_valid(a, b, &self.board) == true && self.board[a][b] != 0{
            return true;
        } else {
            if board[a][b] == 0 {
                for v in curr+1..=9 {
                    self.board[a][b] = v;
                    if self.is_placement_valid(a, b, &self.board) == false {
                        if v < 9 {
                            continue;
                        }
                        if v >=9 {
                            self.board[a][b] = 0;
                            return false;
                        }
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
                            if a+1 <= 8 {
                                if self.backtrack(a+1, 0, board) == false {
                                
                                continue; 
                                }else {
                                    
                                    return true;
                                }
                            }
                            else {
                                if self.backtrack(a, 8, board) == false {
                                
                                continue; 
                                }else {
                                    
                                    return true;
                                }
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

                    if a+1 <= 8 {
                        if self.backtrack(a+1, 0, board) == false{
                        
                            return false
                        }
                        else {return true;}
                    }
                    else {
                        if self.backtrack(a, 8, board) == false{
                        
                            return false
                        }
                        else {return true;}
                    }
                   
                    
                }
            }

        }
        self.board[a][b] = 0;
        false
    }   

    pub fn get_min_candidate_count(&self) -> (usize, usize) {

        let mut count = 0;
        let mut board= self.board;

        let (m, n) = self.get_len();

        let mut min = 10;

        let mut a: usize = 10;
        let mut b: usize = 10;

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 0 {
                    for v in 1..=m {
                        board[i][j] = v as u8;
                        if self.is_placement_valid(i, j, &board) {
                            count += 1;
                        }
                    }
                    if count != 0 && count < min {
                        min = count;
                        (a, b) = (i, j);
                    }
                    count = 0;
                    board[i][j] = 0;
                } else {continue;}
               
            }
        }
        (a, b)

    }

    
}

