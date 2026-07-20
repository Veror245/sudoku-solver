use std::collections::HashMap;


#[derive(Debug)]
pub struct Sudoku {
    pub board: [[u8; 9]; 9],
    pub is_solved: bool,
    pub recursive_calls: u32,
    pub validity_checks: u32
}

pub struct Cell {
    value: (usize, usize),
    candidates: [u8; 9],
    len: usize,
}

pub struct Cells {
    cells: Vec<Cell>
}

impl Cells {
    fn push_cell(&mut self, cell: Cell) {
        self.cells.push(cell);
    }
}

impl Sudoku {
    pub fn new(board: [[u8; 9]; 9]) -> Self {
        Self { board: board, is_solved: false, recursive_calls: 0, validity_checks: 0 }
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

    pub fn solve(&mut self, mode: &str) -> bool {

        if mode == "bt" {
            let board = self.board;
            self.is_solved = self.backtrack(0, 0, &board);
            return self.is_solved 
        } else if mode == "mrv" {
            let mut cells = self.get_candidates();
            self.is_solved = self.mrv(&mut cells);
            return self.is_solved
        } else if mode == "opt_mrv" {
            let mut cells = self.get_candidates();
            self.is_solved = self.opt_mrv(&mut cells);
            return self.is_solved
        }
        
       false
        
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
        self.recursive_calls  += 1;

        self.validity_checks += 1;
        if (a, b) == (8,8) && self.is_placement_valid(a, b, &self.board) == true && self.board[a][b] != 0{
            return true;
        } else {
            if board[a][b] == 0 {
                for v in curr+1..=9 {
                    self.board[a][b] = v;
                    self.validity_checks += 1;
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

    pub fn get_min_candidate_count(&mut self) -> (usize, usize, [u8; 9]) {

        let mut count = 0;
        let mut board= self.board;

        let (m, n) = self.get_len();

        let mut min = 10;

        let mut a: usize = 10;
        let mut b: usize = 10;

        let mut candidates:[u8; 9] = [0; 9];
        let mut best_cands :[u8; 9] = [0; 9];

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 0 {
                    for v in 1..=m {
                        board[i][j] = v as u8;
                        self.validity_checks += 1;
                        if self.is_placement_valid(i, j, &board) {
                            
                            count += 1;
                            candidates[v-1] = v as u8;
                        }
                        if count > min {
                            break;
                        }
                    }
                    if count != 0 && count < min {
                        min = count;
                        (a, b) = (i, j);
                        best_cands = candidates.clone();
                    } if count == 1 {
                        return (a, b, best_cands);
                    }
                    count = 0;
                    candidates = [0; 9];
                    board[i][j] = 0;
                } else {continue;}
               
            }
        }
        (a, b, best_cands)

    }

    pub fn get_min_candidate_count_better(&mut self, cells: &Cells) -> (usize, usize, [u8; 9]) {

        let c1 = cells.cells.iter().next().unwrap();
        let mut min = c1.len;
        let mut a: usize = 0;
        let mut b: usize = 0;
        let mut candidates:[u8; 9] = [0; 9];

        for cell in &cells.cells {
            if cell.len < min {
                min = cell.len;
                (a,b) = cell.value;
                candidates = cell.candidates;
            }
            
        }
        (a, b, candidates)

    }

    fn update_neighbors(&self, a:usize, b:usize, val: u8, cells: &mut Cells) {

        for cell in &mut cells.cells {
            let (i, j) = cell.value;
            if a==i || b==j || (i/3 == a/3 && j/3 == b/3) {
                if !(a == i && b == j) {
                    if cell.candidates[(val-1) as usize] != 0 {
                        cell.candidates[(val-1) as usize] = 0;
                        cell.len -= 1;
                    }
                    
                }
                
            }
        }
        

    }

    fn mrv(&mut self, cells: &mut Cells) -> bool {

        let i: usize;
        let j: usize;
        let candidates: [u8; 9];
        self.recursive_calls += 1;

        
        (i, j, candidates) = self.get_min_candidate_count_better(cells);
        //println!("Currently at: ({}, {})", i,j);
        if i != 10 && j != 10 {
            for v in candidates.iter().filter(|x| **x!= 0) {
                self.board[i][j] = *v;
                self.validity_checks += 1;
                if self.mrv(cells) == false {
                    self.board[i][j] = 0;
                    continue;
                } else{
                    return true;
                }    
            }
        }
        else {
            if self._is_filled() == true {
                return true;
            }
            else {
                return false;
            }
        }

        false
    }  

    fn opt_mrv(&mut self, cells: &mut Cells) -> bool{

        for cell in &cells.cells {
            let (a, b) = cell.value;
            for v in cell.candidates {
                self.board[a][b] = v;
                
            }
        }


        false

    } 

    fn get_candidates(&mut self) -> Cells {
        let mut board = self.board;

        let (m, n) = self.get_len();
        // let len = candidates.iter().filter(|x| **x != 0).collect();
        let mut count = 0;

        let mut candidates = [0; 9];
        let mut cells_c = Cells{
            cells : Vec::new()
        };
        

        for i in 0..m {
            for j in 0..n {
                if self.board[i][j] == 0 {
                    for v in 1..=m {
                    board[i][j] = v as u8;
                    if self.is_placement_valid(i, j, &board) {
                        candidates[v-1] = v as u8;
                        count += 1;
                    }
                }
                let cell = Cell{value: (i, j) , candidates: candidates, len: count};
                cells_c.cells.push(cell);
                candidates = [0; 9];
                count = 0;
                board[i][j] = 0;
                } else {continue;}
                
            }
        }

        cells_c
    }

}

