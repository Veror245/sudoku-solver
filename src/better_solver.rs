pub struct Solver {
    pub board: [u8; 81],

    row_mask: [u16; 9],
    col_mask: [u16; 9],
    box_mask: [u16; 9],
    
    neighbors: [[u8; 20]; 81], 
    candidate_bucket: [[usize; 81]; 10],
    bucket_len: [usize; 10],
    bucket_pos: [usize; 81]

}

impl Solver {
    pub fn new(board: [u8; 81]) -> Self {

        let mut row_mask = [0u16; 9];
        let mut col_mask = [0u16; 9];
        let mut box_mask = [0u16; 9];
        let mut neighbors = [[0u8; 20]; 81];
        let mut candidate_bucket = [[ 100 as usize; 81]; 10];
        let mut bucket_len = [0usize; 10];
        let mut bucket_pos: [usize; 81] = [100usize; 81];
        

        for (idx, cell) in board.iter().enumerate() {
            let mut count = 0;
            let row = idx / 9;
            let col = idx % 9;
            let box_idx = (row / 3) * 3 + col / 3 ;

            if *cell != 0 {
                row_mask[row] |= 1 << *cell;
                col_mask[col] |= 1 << *cell;
                box_mask[box_idx] |= 1 << *cell;

            }
            

            for i in 0..81 {
                let row_n = i / 9;
                let col_n = i % 9;
                let box_idxn = (row_n / 3) * 3 + col_n / 3 ;

                if (row_n == row || col_n == col || box_idx == box_idxn) && i!=idx{
                    neighbors[idx][count] = i as u8;
                    count += 1;
                }

            }
        }

        for (idx, cell) in board.iter().enumerate() {

            if *cell == 0 {
                let candidates_count = super::better_solver::Solver::get_candidates_count(&Self { board, row_mask, col_mask, box_mask, neighbors, candidate_bucket, bucket_len, bucket_pos }, idx);
                candidate_bucket[candidates_count as usize][bucket_len[candidates_count as usize]]= idx;
                bucket_pos[idx] = bucket_len[candidates_count as usize];
                bucket_len[candidates_count as usize] += 1;
                
            }


        }


        Self { board:board, row_mask:row_mask, col_mask:col_mask, box_mask:box_mask, neighbors: neighbors, 
            candidate_bucket: candidate_bucket, bucket_len:bucket_len, bucket_pos: bucket_pos }

        
    }

    fn get_candidates(&self, idx: usize) -> u16{

        const ALL_DIGITS: u16 = 0b1111111110;

        let row = idx / 9;
        let col = idx % 9;
        let box_idx = (row / 3) * 3 + col / 3 ;

        let used = self.row_mask[row] | self.col_mask[col] | self.box_mask[box_idx];
        let candidates = !used & ALL_DIGITS;

        return candidates
        

    }

    fn get_candidates_count(&self, idx: usize) -> u32{

        const ALL_DIGITS: u16 = 0b1111111110;

        let row = idx / 9;
        let col = idx % 9;
        let box_idx = (row / 3) * 3 + col / 3 ;

        let used = self.row_mask[row] | self.col_mask[col] | self.box_mask[box_idx];
        let candidates = !used & ALL_DIGITS;

        let count = candidates.count_ones();

        count

    }

    fn _insert_candidate_into_masks(&mut self, candidate: u8, idx: usize) {

        let row = idx / 9;
        let col = idx % 9;
        let box_idx = (row / 3) * 3 + col / 3 ;

        self.row_mask[row] |= 1 << candidate;
        self.col_mask[col] |= 1 << candidate;
        self.box_mask[box_idx] |= 1 << candidate;


    }

    fn _remove_candidate_from_masks(&mut self, candidate: u8, idx: usize) {

        let row = idx / 9;
        let col = idx % 9;
        let box_idx = (row / 3) * 3 + col / 3;

        self.row_mask[row] ^= 1 << candidate; 
        self.col_mask[col] ^= 1 << candidate; 
        self.box_mask[box_idx] ^= 1 << candidate; 

    }

    fn check_if_present(&self, candidate: u8, idx: usize) -> bool {

        let row = idx / 9;
        let col = idx % 9;
        let box_idx = (row / 3) * 3 + col / 3;        

        (self.row_mask[row] & (1<<candidate) != 0) || 
        (self.col_mask[col] & (1<<candidate) != 0) || 
        (self.box_mask[box_idx] & (1 << candidate) != 0)
    }

    fn get_min_candidate_idx(&self) -> usize {

        let idx: usize;
        
        for i in 1..=9 {
            if self.bucket_len[i] > 0 {
                idx = self.candidate_bucket[i][0];
                return idx
            }
            
        }


        81

    }

    fn update_state(&mut self, idx: usize, candidate: u8) { //update row mask, col mask, box mask, candidate map, candidate bucket len
        /*candidate_bucket: [[usize; 81]; 10],
            bucket_len: [usize; 10] */


        let candidate_count = self.get_candidates_count(idx) as usize;
        let mut neighbor_cc:u32; 
        self._insert_candidate_into_masks(candidate, idx);

        let neighbors = self.neighbors[idx];

        for n in neighbors {
            if self.check_if_present(candidate, n as usize) == false {
                neighbor_cc = self.get_candidates_count(n as usize);
                if self.bucket_len[neighbor_cc as usize] > 0 {
                    let last_index = self.bucket_len[neighbor_cc as usize] - 1;
                    //remove
                    self.candidate_bucket[neighbor_cc as usize][self.bucket_pos[n as usize]] = self.candidate_bucket[neighbor_cc as usize][last_index]; // the last cell index in the candidate bucket
                    self.bucket_pos[self.candidate_bucket[neighbor_cc as usize][last_index]] = self.bucket_pos[n as usize]; // changing the bucket pos of the cell index
                    self.bucket_len[neighbor_cc as usize] -= 1;
                    //insert
                    //neighbor_cc = self.get_candidates_count(n as usize);
                    self.candidate_bucket[neighbor_cc as usize][self.bucket_len[neighbor_cc as usize]] = n as usize;
                    self.bucket_pos[n as usize] = self.bucket_len[neighbor_cc as usize];
                    self.bucket_len[neighbor_cc as usize] += 1;
                }
            }
            
        }

        if self.bucket_len[candidate_count] > 0 {
            let last_index = self.bucket_len[candidate_count] - 1;
            self.candidate_bucket[candidate_count][self.bucket_pos[idx]] = self.candidate_bucket[candidate_count][last_index]; // the last cell index in the candidate bucket
            self.bucket_pos[self.candidate_bucket[candidate_count][last_index]] = self.bucket_pos[idx]; // changing the bucket pos of the cell index
            self.bucket_len[candidate_count] -= 1;
        }


    }

    fn restore_state(&self) {
        
    }

    fn bit_mrv(&mut self) -> bool { //placement valid would be candidate_count > 1 else no placement valid

        let min_idx = self.get_min_candidate_idx();
        let row = min_idx / 9;
        let col = min_idx % 9;
        let box_idx = (row / 3) * 3 + col / 3;  

        if min_idx != 81 {
            let mut candidates = self.get_candidates(min_idx);
            let trailing_zeroes = candidates.trailing_zeros(); //trailing zeroes gives the digits ayo, gotta update the candidates mask too
            candidates &= !(1 << trailing_zeroes);



        } else {
            return true
        }

        false

    }



    
}


#[cfg(test)]
impl Solver {
    pub fn debug_check(&self) {
        let mut empty_cells = 0;

        for idx in 0..81 {
            if self.board[idx] != 0 {
                continue;
            }

            empty_cells += 1;

            let count = self.get_candidates_count(idx) as usize;
            let pos = self.bucket_pos[idx];

            assert!(
                pos < self.bucket_len[count],
                "Cell {} has invalid bucket_pos {} for bucket {}",
                idx,
                pos,
                count
            );

            assert_eq!(
                self.candidate_bucket[count][pos],
                idx,
                "Cell {} is not at its recorded bucket position",
                idx
            );
        }

        let total: usize = self.bucket_len.iter().skip(1).sum();

        assert_eq!(
            total,
            empty_cells,
            "Bucket lengths ({}) != empty cells ({})",
            total,
            empty_cells
        );

        println!("✅ Bucket invariants OK");

        for bucket in 1..=9 {
        for i in 0..self.bucket_len[bucket] {
        let cell = self.candidate_bucket[bucket][i];

        assert_eq!(self.bucket_pos[cell], i);
        assert_eq!(self.get_candidates_count(cell) as usize, bucket);
    }
}
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_invariants() {
        let board = [
    9,8,0,7,0,0,6,0,0,
    7,5,0,0,4,0,0,0,0,
    0,0,3,0,0,8,0,7,0,
    5,0,0,0,0,7,0,3,0,
    0,0,9,4,0,0,0,0,0,
    0,0,0,2,0,1,0,0,3,
    0,0,0,0,0,0,1,0,9,
    0,0,0,5,0,8,0,0,0,
    5,2,0,0,0,0,0,6,0,
];

        let mut solver = Solver::new(board);

        solver.debug_check();

        

        solver.update_state(2, 1);
        solver.debug_check();
        }
}