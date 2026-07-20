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

    fn is_placement_valid(&self, a:usize, b:usize) -> bool {

        let q1: &[u8];
        let q2: &[u8];
        let q3: &[u8];
        
        match (a, b) {
            (0..3,0..3) => {
                 q1 = &self.board[0][0..3];
                 q2 = &self.board[1][0..3];
                 q3 = &self.board[2][0..3];

            }
            (0..3,3..6) => {
                 q1= &self.board[0][3..6];
                 q2 = &self.board[1][3..6];
                 q3 = &self.board[2][3..6];

            }
            (0..3, 6..9) => {
                 q1= &self.board[0][6..9];
                 q2 = &self.board[1][6..9];
                 q3 = &self.board[2][6..9];
            }
            (3..6,0..3) => {
                 q1 = &self.board[3][0..3];
                 q2 = &self.board[4][0..3];
                 q3 = &self.board[5][0..3];

            }
            (3..6,3..6) => {
                 q1= &self.board[3][3..6];
                 q2 = &self.board[4][3..6];
                 q3 = &self.board[5][3..6];

            }
            (3..6, 6..9) => {
                 q1= &self.board[3][6..9];
                 q2 = &self.board[4][6..9];
                 q3 = &self.board[5][6..9];
            }
            (6..9,0..3) => {
                 q1 = &self.board[6][0..3];
                 q2 = &self.board[7][0..3];
                 q3 = &self.board[8][0..3];

            }
            (6..9,3..6) => {
                 q1= &self.board[6][3..6];
                 q2 = &self.board[7][3..6];
                 q3 = &self.board[8][3..6];

            }
            (6..9, 6..9) => {
                 q1= &self.board[6][6..9];
                 q2 = &self.board[7][6..9];
                 q3 = &self.board[8][6..9];
            }
            (_, _) => {
                 q1= &self.board[6][6..9];
                 q2 = &self.board[7][6..9];
                 q3 = &self.board[8][6..9];
            }
           
        }

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
        for v in self.board[a] {
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
            let v = self.board[j][b];
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

        if (a, b) == (8,8) && self.is_placement_valid(a, b) == true && self.board[a][b] != 0{
            return true;
        } else {
            if board[a][b] == 0 {
                for v in curr+1..=9 {
                    self.board[a][b] = v;
                    if self.is_placement_valid(a, b) == false {
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

    fn get_candidate_count(&self) -> [[u8; 9]; 9] {

        let count: [[u8; 9]; 9] = [[0; 9]; 9];

        let (m, n) = self.get_len();

        for i in 0..m {
            for j in 0..n {
                for v in 0..m {

                }
            }
        }

        count

    }
}

