
const ALL_DIGITS: u16 = 0b1111111110;

pub struct Solver {
    pub board: [u8; 81],

    row_mask: [u16; 9],
    col_mask: [u16; 9],
    box_mask: [u16; 9],

    row_of:[usize; 81],
    col_of:[usize; 81],
    box_of:[usize; 81],

    candidate_mask: [u16; 81],
    candidate_count: [u8; 81],

    count_mask: [u128; 10],
    neighbors: [[u8; 20]; 81], 
    pub recursive_calls: u128,
 

}

impl Solver {
    pub fn new(board: [u8; 81]) -> Self {

        let mut row_mask = [0u16; 9];
        let mut col_mask = [0u16; 9];
        let mut box_mask = [0u16; 9];
        let mut neighbors = [[0u8; 20]; 81];
        let mut candidate_mask: [u16; 81] = [0u16; 81];
        let mut candidate_count: [u8; 81] = [100; 81];
        let mut count_mask: [u128; 10] = [0u128; 10];
        let mut row_of = [0; 81];
        let mut col_of = [0; 81];
        let mut box_of = [0; 81];
        

        for (idx, cell) in board.iter().enumerate() {
            let mut count = 0;
            let row = idx / 9;
            let col = idx % 9;
            let box_idx = (row / 3) * 3 + col / 3 ;

            row_of[idx] = row;
            col_of[idx] = col;
            box_of[idx] = box_idx;

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
            if *cell != 0 {
                continue;
            }

            let row = idx / 9;
            let col = idx % 9;
            let box_idx = (row / 3) * 3 + col / 3;

            let used = row_mask[row] | col_mask[col] | box_mask[box_idx];
            let mask = !used & ALL_DIGITS;

            candidate_mask[idx] = mask;

            let count = mask.count_ones() as u8;
            candidate_count[idx] = count;

            count_mask[count as usize] |= 1u128<<idx; 

        }



        Self { board:board, row_mask:row_mask, col_mask:col_mask, box_mask:box_mask, neighbors: neighbors, 
            /*candidate_bucket: candidate_bucket, bucket_len:bucket_len, bucket_pos: bucket_pos,*/ candidate_mask, candidate_count ,
        count_mask: count_mask,   recursive_calls: 0, /*prop_digit: [0; 81], prop_idx:  [0; 81], prop_log:  [0; 81]*/row_of, col_of, box_of}

        
    }

    #[inline(always)]
    fn _insert_candidate_into_masks(&mut self, candidate: u8, idx: usize) {

        self.row_mask[self.row_of[idx]] |= 1 << candidate;
        self.col_mask[self.col_of[idx]] |= 1 << candidate;
        self.box_mask[self.box_of[idx]] |= 1 << candidate;


    }

    #[inline(always)]
    fn _remove_candidate_from_masks(&mut self, candidate: u8, idx: usize) {

        self.row_mask[self.row_of[idx]] &= !(1 << candidate); 
        self.col_mask[self.col_of[idx]] &= !(1 << candidate); 
        self.box_mask[self.box_of[idx]] &= !(1 << candidate); 
  

    }

    #[inline(always)]
    fn get_min_candidate_idx(&self) -> (usize, u8) {
        for i in 0..=9 {
            let mask = self.count_mask[i];
            if mask != 0 {
                // Rotating offset based on recursion depth (0 to 127)
                let start = (self.recursive_calls & 127) as u32;
                
                // Rotate the mask so we start searching from a dynamic position
                let rotated_mask = mask.rotate_left(start);
                
                // Find the first set bit in the rotated mask
                let pos = rotated_mask.trailing_zeros() as usize;
                
                // Mathematically reverse the rotation to get the actual cell index
                let idx = (pos + 128 - start as usize) % 128;
                
                return (idx, i as u8);
            }
        }
        (81, 0)
    }


    #[inline(always)]
    fn update_state_bit(&mut self, idx: usize, candidate: u8) -> u128 {
        /*
        candidate mask uses cell idx as its idx, same for candidate count 
        */
        let candidate_count = self.candidate_count[idx] as usize;
        self.board[idx] = candidate;

        let mut affected_neighbors = 0u128;

        for n in self.neighbors[idx] {
            let n = n as usize;

            if self.board[n] != 0 {
                continue;
            }

            if (self.candidate_mask[n] & (1 << candidate)) == 0 {
                continue;
            }

            self.count_mask[self.candidate_count[n] as usize] &= !(1u128<<n);


            self.candidate_mask[n] &= !(1 << candidate);
            self.candidate_count[n] -= 1;

            self.count_mask[self.candidate_count[n] as usize] |= 1u128<<n;

            affected_neighbors |= 1u128<<n;

        }

        self.count_mask[candidate_count] &= !(1u128 << idx);

        self._insert_candidate_into_masks(candidate, idx);

        affected_neighbors
    }

    #[inline(always)]
    fn restore_state_bit(&mut self, affected_neighbors: &mut u128, idx: usize, candidate: u8) {

        self.board[idx] = 0;

        while *affected_neighbors != 0 {
            let n = (*affected_neighbors).trailing_zeros() as usize;

            self.count_mask[self.candidate_count[n] as usize] &= !(1u128<<n);


            self.candidate_mask[n] |= 1 << candidate;
            self.candidate_count[n] += 1;

            self.count_mask[self.candidate_count[n] as usize] |= 1u128<<n;

            *affected_neighbors &= !(1u128 << n) ;
        }

        let candidate_count = self.candidate_count[idx] as usize;

        self.count_mask[candidate_count] |= 1u128 << idx;

        self._remove_candidate_from_masks(candidate, idx);

    }

    #[inline(always)]
    fn find_hidden_single(&self, idx: usize) -> Option<(usize, u8)> {
        let r = self.row_of[idx];
        let c = self.col_of[idx];
        let b = self.box_of[idx];

        let mut last_pos = [0usize; 10];

        // Check Row
        let mut once = 0u16;
        let mut multiple = 0u16;
        for i in 0..9 {
            let cell = r * 9 + i;
            if self.board[cell] != 0 { continue; }
            let mut mask = self.candidate_mask[cell];
            while mask != 0 {
                let d = mask.trailing_zeros();
                mask &= mask - 1; // Clear lowest set bit
                let bit = 1u16 << d;
                if (once & bit) != 0 {
                    multiple |= bit;
                } else {
                    once |= bit;
                    last_pos[d as usize] = cell;
                }
            }
        }
        let singles = once & !multiple;
        if singles != 0 {
            let d = singles.trailing_zeros() as u8;
            return Some((last_pos[d as usize], d));
        }

        // Check Col
        once = 0u16;
        multiple = 0u16;
        for i in 0..9 {
            let cell = i * 9 + c;
            if self.board[cell] != 0 { continue; }
            let mut mask = self.candidate_mask[cell];
            while mask != 0 {
                let d = mask.trailing_zeros();
                mask &= mask - 1;
                let bit = 1u16 << d;
                if (once & bit) != 0 {
                    multiple |= bit;
                } else {
                    once |= bit;
                    last_pos[d as usize] = cell;
                }
            }
        }
        let singles = once & !multiple;
        if singles != 0 {
            let d = singles.trailing_zeros() as u8;
            return Some((last_pos[d as usize], d));
        }

        // Check Box
        once = 0u16;
        multiple = 0u16;
        let br = (b / 3) * 3;
        let bc = (b % 3) * 3;
        for i in 0..3 {
            for j in 0..3 {
                let cell = (br + i) * 9 + (bc + j);
                if self.board[cell] != 0 { continue; }
                let mut mask = self.candidate_mask[cell];
                while mask != 0 {
                    let d = mask.trailing_zeros();
                    mask &= mask - 1;
                    let bit = 1u16 << d;
                    if (once & bit) != 0 {
                        multiple |= bit;
                    } else {
                        once |= bit;
                        last_pos[d as usize] = cell;
                    }
                }
            }
        }
        let singles = once & !multiple;
        if singles != 0 {
            let d = singles.trailing_zeros() as u8;
            return Some((last_pos[d as usize], d));
        }

        None
    }

    fn _bit_mrv(&mut self) -> bool { /// the no buvket version

        let (min_idx, candidate_count) = self.get_min_candidate_idx();
        self.recursive_calls += 1;
        
        if min_idx != 81 {
            let mut candidates = self.candidate_mask[min_idx]; 
            while candidates != 0 {
                let digit = candidates.trailing_zeros(); //trailing zeroes gives the digits ayo, gotta update the candidates mask too
                candidates &= !(1 << digit);
                let mut an:u128 = self.update_state_bit(min_idx, digit as u8);
                if self._bit_mrv() == false {
                    self.restore_state_bit(&mut an,  min_idx, digit as u8);
                }  
                else {
                    return true;
                }
            }
            
        } else {
            return true
        }

        false

    }

    fn _bit_mrv_singles_(&mut self) -> bool { //placement valid would be candidate_count > 1 else no placement valid, the no buvket version

        let (min_idx, candidate_count) = self.get_min_candidate_idx();
        self.recursive_calls += 1;
        
        if min_idx != 81 {
            let mut candidates = self.candidate_mask[min_idx]; 
            'outer: while candidates != 0 {
                let mut prop_length = 0;
                let mut prop_idx = [0; 81];
                let mut prop_digit = [0; 81];
                let mut prop_log = [0u128; 81];
                let digit = candidates.trailing_zeros(); //trailing zeroes gives the digits ayo, gotta update the candidates mask too
                candidates &= !(1 << digit);
                let mut an:u128 = self.update_state_bit(min_idx, digit as u8);
                while self.count_mask[1] != 0 {
                    let idx = self.count_mask[1].trailing_zeros() as usize;
                    let prop_dig = self.candidate_mask[idx].trailing_zeros();
                    let anp:u128 = self.update_state_bit(idx, prop_dig as u8);
                    prop_idx[prop_length] = idx;
                    prop_digit[prop_length] = prop_dig as u8;
                    prop_log[prop_length] = anp;
                    prop_length += 1;
                    if self.count_mask[0] != 0 {
                        for i in (0..prop_length).rev() {
                        self.restore_state_bit(&mut prop_log[i], prop_idx[i], prop_digit[i] as u8);
                        }
                        self.restore_state_bit(&mut an, min_idx, digit as u8);
                        continue 'outer;
                    }
                    
                }
                if self._bit_mrv_singles_() == false {
                    for i in (0..prop_length).rev() {
                        self.restore_state_bit(&mut prop_log[i], prop_idx[i], prop_digit[i] as u8);
                    }

                    self.restore_state_bit(&mut an,  min_idx, digit as u8);
                }  
                else {
                    return true;
                }
            }
            
        } else {
            return true
        }

        false

    }

    fn _bit_mrv_hidden_singles(&mut self) -> bool { //placement valid would be candidate_count > 1 else no placement valid, the no buvket version

        let (min_idx, candidate_count) = self.get_min_candidate_idx();
        self.recursive_calls += 1;
        
        if min_idx != 81 {
            let mut candidates = self.candidate_mask[min_idx]; 
            'outer: while candidates != 0 {
                let mut prop_length = 0;
                let mut prop_idx = [0; 81];
                let mut prop_digit = [0; 81];
                let mut prop_log = [0u128; 81];
                
                // Hidden single check queue
                let mut check_idx = [0usize; 81];
                let mut check_len = 0;
                
                let digit = candidates.trailing_zeros();
                candidates &= !(1u16 << digit);
                
                let mut an: u128 = self.update_state_bit(min_idx, digit as u8);
                
                prop_idx[prop_length] = min_idx;
                prop_digit[prop_length] = digit as u8;
                prop_log[prop_length] = an;
                prop_length += 1;
                
                check_idx[check_len] = min_idx;
                check_len += 1;
                
                loop {
                    if self.count_mask[0] != 0 {
                        for i in (0..prop_length).rev() {
                            let mut log = prop_log[i];
                            self.restore_state_bit(&mut log, prop_idx[i], prop_digit[i]);
                        }
                        continue 'outer;
                    }
                    
                    if self.count_mask[1] != 0 {
                        let idx = self.count_mask[1].trailing_zeros() as usize;
                        let prop_dig = self.candidate_mask[idx].trailing_zeros() as u8;
                        let anp: u128 = self.update_state_bit(idx, prop_dig);
                        
                        prop_idx[prop_length] = idx;
                        prop_digit[prop_length] = prop_dig;
                        prop_log[prop_length] = anp;
                        prop_length += 1;
                        
                        check_idx[check_len] = idx;
                        check_len += 1;
                    } else {
                        let mut found = false;
                        while check_len > 0 {
                            check_len -= 1;
                            let c_idx = check_idx[check_len];
                            
                            if let Some((h_idx, h_dig)) = self.find_hidden_single(c_idx) {
                                let anp: u128 = self.update_state_bit(h_idx, h_dig);
                                
                                prop_idx[prop_length] = h_idx;
                                prop_digit[prop_length] = h_dig;
                                prop_log[prop_length] = anp;
                                prop_length += 1;
                                
                                check_idx[check_len] = h_idx;
                                check_len += 1;
                                
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            break;
                        }
                    }
                }
                
                if self._bit_mrv_hidden_singles() == false {
                    for i in (0..prop_length).rev() {
                        let mut log = prop_log[i];
                        self.restore_state_bit(&mut log, prop_idx[i], prop_digit[i]);
                    }
                }  
                else {
                    return true;
                }
            }
            
        } else {
            return true
        }

        false
    }

    pub fn solve(&mut self) -> bool {
        return self._bit_mrv_hidden_singles();
        
    }

    
}


// #[cfg(test)]
// impl Solver {
//     pub fn debug_check(&self) {
//         let mut empty_cells = 0;

//         for idx in 0..81 {
//             if self.board[idx] != 0 {
//                 continue;
//             }

//             empty_cells += 1;

          

//             let count = self._get_candidates_count(idx) as usize;
//             let pos = self.bucket_pos[idx];

//             assert!(
//                 pos < self.bucket_len[count],
//                 "Cell {} has invalid bucket_pos {} for bucket {}",
//                 idx,
//                 pos,
//                 count
//             );

//             assert_eq!(
//                 self.candidate_bucket[count][pos],
//                 idx,
//                 "Cell {} is not at its recorded bucket position",
//                 idx
//             );
//         }

//         let total: usize = self.bucket_len.iter().skip(1).filter(|x| **x != 100).sum();

//         assert_eq!(
//             total,
//             empty_cells,
//             "Bucket lengths ({}) != empty cells ({})",
//             total,
//             empty_cells
//         );

//         println!("✅ Bucket invariants OK");

//         for bucket in 1..=9 {
//                 for i in 0..self.bucket_len[bucket] {
//                 let cell = self.candidate_bucket[bucket][i];

//                 assert_eq!(self.bucket_pos[cell], i);
//                 assert_eq!(self._get_candidates_count(cell) as usize, bucket);
//             }
//         }

//         let total: usize = self.bucket_len.iter().skip(1).sum();

// if total != empty_cells {
//     println!("==========================");

//     for idx in 0..81 {
//         if self.board[idx] == 0 {
//             let mut found = false;

//             for bucket in 1..=9 {
//                 for i in 0..self.bucket_len[bucket] {
//                     if self.candidate_bucket[bucket][i] == idx {
//                         found = true;
//                     }
//                 }
//             }

//             if !found {
//                 println!("Missing cell: {}", idx);
//                 println!("bucket_pos = {}", self.bucket_pos[idx]);
//                 println!("candidate_count = {}", self.candidate_count[idx]);
//                 println!("candidate_mask = {:09b}", self.candidate_mask[idx]);
//             }
//         }
//     }

//     panic!(
//         "Bucket lengths ({}) != empty cells ({})",
//         total,
//         empty_cells
//     );
// }
//     }
// }




// #[cfg(test)]
// mod tests {
//     use super::*;

//     const BOARD: [u8; 81] = [
//         9,8,0,7,0,0,6,0,0,
//         7,5,0,0,4,0,0,0,0,
//         0,0,3,0,0,8,0,7,0,
//         5,0,0,0,0,7,0,3,0,
//         0,0,9,4,0,0,0,0,0,
//         0,0,0,0,2,0,1,0,0,
//         3,0,0,0,0,0,0,0,1,
//         0,9,0,0,0,5,0,8,0,
//         0,0,5,2,0,0,0,0,6,
//     ];

//     fn verify_candidate_state(a: &Solver, b: &Solver) {
//         assert_eq!(a.board, b.board);

//         for i in 0..81 {
//             assert_eq!(
//                 a.candidate_mask[i],
//                 b.candidate_mask[i],
//                 "candidate_mask mismatch at cell {}",
//                 i
//             );

//             assert_eq!(
//                 a.candidate_count[i],
//                 b.candidate_count[i],
//                 "candidate_count mismatch at cell {}",
//                 i
//             );
//         }
//     }

//     #[test]
//     fn test_state_sync() {
//         let mut bucket = Solver::new(BOARD);
//         let mut bit = Solver::new(BOARD);

//         let mut bucket_hist = Vec::new();
//         let mut bit_hist = Vec::new();

//         for step in 0..10 {
//             let (idx, _) = bucket.get_min_candidate_idx_nobit();
//             let digit = bucket.get_candidates(idx).trailing_zeros() as u8;

//             let bucket_state = bucket.update_state(idx, digit);
//             let bit_state = bit.update_state_bit(idx, digit);

//             bucket_hist.push((idx, digit, bucket_state));
//             bit_hist.push((idx, digit, bit_state));

//             verify_candidate_state(&bucket, &bit);

//             println!("Step {} OK", step);
//         }

//         while let Some((idx, digit, (an, anl))) = bucket_hist.pop() {
//             bucket.restore_state(an, anl, idx, digit);
//         }

//         while let Some((idx, digit, mut an)) = bit_hist.pop() {
//             bit.restore_state_bit(&mut an, idx, digit);
//         }

//         verify_candidate_state(&bucket, &bit);
//         assert_eq!(bucket.board, BOARD);
//         assert_eq!(bit.board, BOARD);
//     }

//     #[test]
//     fn test_mrv_equivalence() {
//         let mut bucket = Solver::new(BOARD);
//         let mut bit = Solver::new(BOARD);

//         for step in 0..20 {
//             let (idx_bucket, cnt_bucket) = bucket.get_min_candidate_idx_nobit();
//             let (idx_bit, cnt_bit) = bit.get_min_candidate_idx();

//             assert_eq!(
//                 cnt_bucket, cnt_bit,
//                 "MRV count diverged at step {}",
//                 step
//             );

//             if idx_bucket != idx_bit {
//     println!(
//         "Tie: bucket chose {}, bit chose {} (count={})",
//         idx_bucket, idx_bit, cnt_bucket
//     );
// }

//             let digit = bucket.get_candidates(idx_bucket).trailing_zeros() as u8;

//             bucket.update_state(idx_bucket, digit);
//             bit.update_state_bit(idx_bucket, digit);

//             verify_candidate_state(&bucket, &bit);
//         }
//     }
// }