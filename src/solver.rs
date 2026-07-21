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

#[derive(Copy, Clone)]
struct Change {
    row: usize,
    col: usize,
    val: u8,
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
            self.is_solved = self.mrv();
            return self.is_solved
        } else if mode == "opt_mrv" {
            let mut cells = self.get_candidates();
            self.is_solved = self.opt_mrv(&mut cells);
            return self.is_solved
        }
        
       false
        
    }

    fn is_placement_valid(&self, a:usize, b:usize, board: &[[u8; 9]; 9], val:u8) -> bool {

       

        for i in 0..9 {
            if board[a][i] == val && i != b {
                return false;
            }
        }

        for i in 0..9 {
            if board[i][b] == val && i != a {
                return false;
            }
        }

        let sr = (a / 3) * 3;
        let sc = (b / 3) * 3;

        for i in sr..sr+3 {
            for j in sc..sc+3 {
                if (i, j) != (a, b) && board[i][j] == val {
                    return false;
                }
            }
        }
        
        true
    }

    fn backtrack(&mut self, a:usize, b:usize, board: &[[u8; 9]; 9]) -> bool {

        let curr = self.board[a][b];
        self.recursive_calls  += 1;

        self.validity_checks += 1;
        if (a, b) == (8,8) && self.is_placement_valid(a, b, &self.board, self.board[a][b]) == true && self.board[a][b] != 0{
            return true;
        } else {
            if board[a][b] == 0 {
                for v in curr+1..=9 {
                    self.board[a][b] = v;
                    self.validity_checks += 1;
                    if self.is_placement_valid(a, b, &self.board, self.board[a][b]) == false {
                        
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
                        if self.is_placement_valid(i, j, &board, board[i][j]) {
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

    pub fn get_min_candidate_count_better(&mut self, cells: &Cells) -> (usize, usize, [u8; 9], usize) {

        let c1 = cells.cells.iter().next().unwrap();
        let mut min = c1.len;
        let mut a: usize;
        let mut b: usize;
        (a,b) = c1.value;
        let mut candidates:[u8; 9] = c1.candidates;

        for cell in &cells.cells {
            if cell.len < min {
                min = cell.len;
                (a,b) = cell.value;
                candidates = cell.candidates;
            }
            
        }
        (a, b, candidates, min)

    }

    fn update_neighbors(&self, a:usize, b:usize, val: u8, cells: &mut Cells) -> ([Change; 20], Cell) {

        let mut changes: [Change; 20] = [Change{row:10, col:10, val:10}; 20];
        let mut len = 0;
        let mut idx_r = 0;

        for (idx, cell) in &mut cells.cells.iter_mut().enumerate() {
            let (i, j) = cell.value;
            if a==i || b==j || (i/3 == a/3 && j/3 == b/3) {
                if !(a == i && b == j) {
                    if cell.candidates[(val-1) as usize] != 0 {
                        cell.candidates[(val-1) as usize] = 0;
                        cell.len -= 1;
                        changes[len] = Change { row: i, col: j, val: val };
                        len += 1;
                    }
                    
                }
                else {
                    idx_r = idx;
                }
                
            }
        }

        let cell = cells.cells.swap_remove(idx_r);

        (changes, cell)

    }

    fn restore_neighbors(&self, changes: &mut [Change; 20], cells: &mut Cells, cell: Cell) {
        for change in changes {
            if change.row != 10 && change.col != 10 {
                let (crow, ccol) = (change.row, change.col);
                for cell in &mut cells.cells {
                    let (cellrow, cellcol) = cell.value;
                    if  cellrow == crow && cellcol == ccol {
                        if cell.candidates[change.val as usize-1] == 0 {
                            cell.candidates[change.val as usize-1] = change.val;
                            cell.len += 1;
                        }
                    }
                }
            }
            
        }

        cells.cells.push(cell);

        
        
    }

    fn mrv(&mut self) -> bool {

        let i: usize;
        let j: usize;
        let candidates: [u8; 9];
        self.recursive_calls += 1;

        
        (i, j, candidates) = self.get_min_candidate_count();
        //println!("Currently at: ({}, {})", i,j);
        if i != 10 && j != 10 {
            for v in candidates.iter().filter(|x| **x!= 0) {
                self.board[i][j] = *v;
                self.validity_checks += 1;
                if self.mrv() == false {
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

        let i: usize;
        let j: usize;
        let candidates: [u8; 9];
        let len:usize;
        self.recursive_calls += 1;

        
        
        if cells.cells.len() != 0 {
            (i, j, candidates, len) = self.get_min_candidate_count_better(cells);
            //println!("Currently at: ({}, {})", i,j);
            //let mut cands = candidates.iter().filter(|x| **x!= 0);
            if len == 0{
                return false;
            }
            for v in candidates.iter().filter(|x| **x!= 0) {
                self.board[i][j] = *v;
                let (mut changes, cell) = self.update_neighbors(i, j, *v, cells);
                //println!("Remaining cells: {}", cells.cells.len());
                self.validity_checks += 1;
                if self.opt_mrv(cells) == false {
                    self.board[i][j] = 0;
                    self.restore_neighbors(&mut changes, cells, cell);
                    continue;
                } else{
                    return true;
                }    
            }
        }
        else {
            //println!("Solved");
           return true;
        }

        false

    } 

    fn get_candidates(&mut self) -> Cells {
        let mut board = self.board;

        let (m, n) = self.get_len();
        // let len = candidates.iter().filter(|x| **x != 0).collect();
        let mut count = 0;
        //let mut index: usize = 0;

        let mut candidates = [0; 9];
        let mut cells_c = Cells{
            cells : Vec::new(),
        };
        

        for i in 0..m {
            for j in 0..n {
                if self.board[i][j] == 0 {
                    for v in 1..=m {
                    board[i][j] = v as u8;
                    if self.is_placement_valid(i, j, &board, board[i][j]) {
                        candidates[v-1] = v as u8;
                        count += 1;
                    }
                }
                let cell = Cell{value: (i, j) , candidates: candidates, len: count};
                cells_c.cells.push(cell);
                // index += 1;
                candidates = [0; 9];
                count = 0;
                board[i][j] = 0;
                } else {continue;}
                
            }
        }

        cells_c
    }

}

