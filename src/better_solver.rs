pub struct Solver {
    pub board: [u8; 81],

    row_mask: [u16; 9],
    col_mask: [u16; 9],
    box_mask: [u16; 9],
    
    neighbors: [[u8; 20]; 81], 
    candidate_bucket: [[usize; 81]; 10],
    bucket_len: [usize; 10]

}

impl Solver {
    pub fn new(board: [u8; 81]) -> Self {

        let mut row_mask = [0u16; 9];
        let mut col_mask = [0u16; 9];
        let mut box_mask = [0u16; 9];
        let mut neighbors = [[0u8; 20]; 81];
        let mut candidate_bucket = [[ 100 as usize; 81]; 10];
        let mut bucket_len = [0usize; 10];
        

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
                let candidates_count = super::better_solver::Solver::get_candidates_count(&Self { board, row_mask, col_mask, box_mask, neighbors, candidate_bucket, bucket_len }, idx);
                candidate_bucket[candidates_count as usize][bucket_len[candidates_count as usize]]= idx;
                bucket_len[candidates_count as usize] += 1;
            }


        }


        Self { board:board, row_mask:row_mask, col_mask:col_mask, box_mask:box_mask, neighbors: neighbors, candidate_bucket: candidate_bucket, bucket_len:bucket_len }

        
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

    fn insert_candidate(&mut self, candidate: u8, idx: usize) {

        let row = idx / 9;
        let col = idx % 9;
        let box_idx = (row / 3) * 3 + col / 3 ;

        self.row_mask[row] |= 1 << candidate;
        self.col_mask[col] |= 1 << candidate;
        self.box_mask[box_idx] |= 1 << candidate;


    }

    fn remove_candidate(&mut self, candidate: u8, idx: usize) {

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

        let mut idx: usize = 0;
        
        for i in 1..=9 {
            if self.bucket_len[i] > 0 {
                idx = self.candidate_bucket[i][0];
                return idx
            }
            
        }


        81

    }

    fn update_state(&self) {

    }

    fn restore_state(&self) {
        
    }

    fn bit_mrv(&mut self) -> bool { //placement valid would be candidate_count > 1 else no placement valid

        let min_idx = self.get_min_candidate_idx();

        if min_idx != 81 {
            let candidates = self.get_candidates(min_idx);

        } else {
            return true
        }

        false

    }
}