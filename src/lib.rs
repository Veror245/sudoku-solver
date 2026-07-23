pub mod solver;
pub mod euler_parser;
pub mod better_solver;



// #[cfg(test)]
// mod test {
//     pub fn debug_check(&self) {
//         let mut empty_cells = 0;

//         for idx in 0..81 {
//             if self.board[idx] != 0 {
//                 continue;
//             }

//             empty_cells += 1;

//             let count = self.get_candidates_count(idx) as usize;
//             let pos = self.bucket_pos[idx];

//             // Position must be inside the bucket
//             assert!(
//                 pos < self.bucket_len[count],
//                 "Cell {} has invalid bucket_pos {} for bucket {}",
//                 idx,
//                 pos,
//                 count
//             );

//             // Bucket must actually contain this cell
//             assert_eq!(
//                 self.candidate_bucket[count][pos],
//                 idx,
//                 "Cell {} is not at its recorded bucket position",
//                 idx
//             );
//         }

//         // Bucket lengths should add up to the number of empty cells
//         let total: usize = self.bucket_len.iter().skip(1).sum();

//         assert_eq!(
//             total,
//             empty_cells,
//             "Bucket lengths ({}) != empty cells ({})",
//             total,
//             empty_cells
//         );

//         println!("✅ Bucket invariants OK");
//     }
// }