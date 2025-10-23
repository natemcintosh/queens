use bit_board::bitboardstatic::BitBoardStatic;
use bit_board::{DimensionMismatch, bitboard::BitBoard};

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub enum BoardPlacementResult {
    /// The queen was successfully placed on the board.
    Success(QueenBoard),
    /// The queen was not placed because the spot was already claimed by a queen.
    SpotOccupied,
    /// The queen was not placed because it was not in the color region.
    NotInColorRegion,
    /// The queen was not placed because the index at which it was attempted to be
    /// placed was out of bounds.
    IndexOutOfBounds,
    /// The bitboard says there was a dimension mismatch
    DimensionMismatch,
}

/// A BitBoard for playing Queens. Assumes boards are not larger than 192
/// positions (64 * 3).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QueenBoard(BitBoardStatic<3>);

impl QueenBoard {
    /// Create a new board
    pub fn new(n_rows: usize, n_cols: usize) -> Self {
        QueenBoard(BitBoardStatic::new(n_rows, n_cols))
    }

    /// Clears the board by setting all bits to 0.
    pub fn clear(&mut self) {
        self.0.fill(false);
    }

    /// Place the `value` at the linear index `index`.
    pub fn set_linear_index(&mut self, index: usize, value: bool) {
        self.0.board_mut().set(index, value);
    }

    /// Get the linear indices
    pub fn get_linear_indices(&self) -> impl Iterator<Item = usize> {
        self.0.board().iter_ones()
    }

    /// If a queen can be placed at index `queen_idx` on the board, place it and return
    /// the updated board as `BoardPlacementResult::Success`. If not, it will return one
    /// of the other reasons for a failure. If the queen can be placed, the board is
    /// updated by setting the bit at `queen_idx` to 1, as well as all of the bits
    /// representing the row, column, and color region that the queen is in.
    pub fn place_queen(
        &self,
        queen_idx: usize,
        color_region_mask: &QueenBoard,
    ) -> BoardPlacementResult {
        // Make sure the queen is within bounds
        if queen_idx >= self.0.n_cols * self.0.n_rows {
            return BoardPlacementResult::IndexOutOfBounds;
        }

        // Create a QueenBoard with a single bit set at the queen_idx index
        let mut queen_only_board = QueenBoard::new(self.0.n_rows, self.0.n_cols);
        queen_only_board.set_linear_index(queen_idx, true);

        // Make sure the queen is within the color region
        if (color_region_mask.0.and(&queen_only_board.0))
            .expect("Boards had mismatched sizes")
            .board()
            .not_any()
        {
            return BoardPlacementResult::NotInColorRegion;
        }

        // Make sure the spot is not already occupied
        if self.0.board().get(queen_idx).is_some() {
            return BoardPlacementResult::SpotOccupied;
        }

        // Place the queen by blocking of that spot, that row, that column, and the
        // color region
        match self
            .0
            .or(&self.fill_queen_reach(queen_idx, color_region_mask).0)
        {
            Ok(board) => BoardPlacementResult::Success(QueenBoard(board)),
            Err(DimensionMismatch) => BoardPlacementResult::DimensionMismatch,
        }
    }

    /// Fill the color region, the row, the column, and the immediate diagonals.
    pub fn fill_queen_reach(&self, queen_idx: usize, color_region_mask: &QueenBoard) -> QueenBoard {
        let (row, col) = self.0.row_col_of(queen_idx);

        // Create a new board for the queen reach
        let mut new_board = *self;

        // Fill the row, column, and diagonals
        new_board.0.set_row(row, true);
        new_board.0.set_col(col, true);
        new_board.0.set_diagonals(row, col, true);

        // Combine with the color mask
        new_board = QueenBoard(
            new_board
                .0
                .or(&color_region_mask.0)
                .expect("Color region didn't match size with the queen board"),
        );

        new_board
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitvec::field::BitField;

    use rstest::rstest;

    #[rstest]
    // Test with no color region (0), because that part is fairly obviously correct
    #[case(
        0,
        0,
        0b00000001_00000001_00000001_00000001_00000001_00000001_00000011_11111111
    )]
    #[case(
        9,
        0,
        0b00000010_00000010_00000010_00000010_00000010_00000111_11111111_00000111
    )]
    #[case(
        18,
        0,
        0b00000100_00000100_00000100_00000100_00001110_11111111_00001110_00000100
    )]
    #[case(
        27,
        0,
        0b00001000_00001000_00001000_00011100_11111111_00011100_00001000_00001000
    )]
    #[case(
        36,
        0,
        0b00010000_00010000_00111000_11111111_00111000_00010000_00010000_00010000
    )]
    #[case(
        45,
        0,
        0b00100000_01110000_11111111_01110000_00100000_00100000_00100000_00100000
    )]
    #[case(
        54,
        0,
        0b11100000_11111111_11100000_01000000_01000000_01000000_01000000_01000000
    )]
    #[case(
        63,
        0,
        0b11111111_11000000_10000000_10000000_10000000_10000000_10000000_10000000
    )]
    fn test_fill_queen_reach(#[case] queen_idx: u64, #[case] color_region: u64, #[case] want: u64) {
        let board = QueenBoard::new(8, 8);
        let mut color_region_board = QueenBoard::new(8, 8);
        for idx in get_inds_from_u64(color_region) {
            color_region_board.set_linear_index(idx, true);
        }
        let queen_reach = board.fill_queen_reach(queen_idx as usize, &color_region_board);
        assert_eq!(queen_reach.0.board().load_le::<u64>(), want);
    }

    #[test]
    fn test_place_queen_invalid() {
        let board = QueenBoard::new(8, 8);

        // Test index out of bounds
        assert_eq!(
            board.place_queen(64, &QueenBoard::new(8, 8)),
            BoardPlacementResult::IndexOutOfBounds
        );

        // Test spot occupied by placing a queen at the 0 index
        let board = if let BoardPlacementResult::Success(b) =
            board.place_queen(0, &QueenBoard::new(8, 8))
        {
            b
        } else {
            panic!("Placing queen failed unexpectedly");
        };
        assert_eq!(
            board.place_queen(0, &QueenBoard::new(8, 8)),
            BoardPlacementResult::SpotOccupied
        );

        // Test not in color region
        let mut color_region = QueenBoard::new(8, 8);
        color_region.set_linear_index(2, true);
        assert_eq!(
            board.place_queen(1, &color_region),
            BoardPlacementResult::NotInColorRegion
        );
    }

    #[rstest]
    #[case(vec![], 0, 0, 0b00000001_00000001_00000001_00000001_00000001_00000001_00000011_11111111, "place queen on empty board at (0,0)")]
    #[case(vec![], 18, 0, 0b00000100_00000100_00000100_00000100_00001110_11111111_00001110_00000100, "place queen on empty board at (2,2)")]
    #[case(vec![(0, 0)], 10, 0, 0b00000101_00000101_00000101_00000101_00000101_00001111_11111111_11111111, "place queen on board with one queen")]
    #[case(vec![(0, 0), (63, 0)], 18, 0, 0b11111111_11000101_10000101_10000101_10001111_11111111_10001111_11111111, "place queen on board with two queens")]
    #[case(vec![], 2, 1 << 2, 0b00000100_00000100_00000100_00000100_00000100_00000100_00001110_11111111, "place queen with simple color region")]
    #[case(vec![], 10, (1 << 10) | (1 << 18), 0b00000100_00000100_00000100_00000100_00000100_00001110_11111111_00001110, "place queen with complex color region")]
    fn test_place_queen_valid(
        #[case] initial_placements: Vec<(u64, u64)>,
        #[case] new_queen_idx: u64,
        #[case] new_color_region: u64,
        #[case] expected_board_val: u64,
        #[case] _description: &str,
    ) {
        let mut board = QueenBoard::new(8, 8);
        for (idx, color) in initial_placements {
            let mut color_region = QueenBoard::new(8, 8);
            color_region.set_linear_index(color as usize, true);
            board = if let BoardPlacementResult::Success(b) =
                board.place_queen(idx as usize, &color_region)
            {
                b
            } else {
                panic!("Failed to setup board for test");
            };
        }

        let mut color_region = QueenBoard::new(8, 8);
        color_region.set_linear_index(new_color_region as usize, true);
        let result = board.place_queen(new_queen_idx as usize, &color_region);
        let mut expected_board = QueenBoard::new(8, 8);
        expected_board.set_linear_index(expected_board_val as usize, true);
        assert_eq!(result, BoardPlacementResult::Success(expected_board));
    }
    #[test]
    fn test_good_region_input() {
        let string = "11233456 12234456 11233456 12273456 11233456 88885556 66888886 66666666";
        let (map, n_rows, n_cols) = parse_color_region_inds(string);
        let regions = parse_color_region_boards(&map, n_rows, n_cols);
        assert_eq!(
            regions,
            [
                build_queen_board_from_inds(&[0, 1, 8, 16, 17, 24, 32, 33], n_rows, n_cols),
                build_queen_board_from_inds(&[2, 9, 10, 18, 25, 26, 34], n_rows, n_cols),
                build_queen_board_from_inds(&[3, 4, 11, 19, 20, 28, 35, 36], n_rows, n_cols),
                build_queen_board_from_inds(&[5, 12, 13, 21, 29, 37], n_rows, n_cols),
                build_queen_board_from_inds(&[6, 14, 22, 30, 38, 44, 45, 46], n_rows, n_cols),
                build_queen_board_from_inds(
                    &[
                        7, 15, 23, 31, 39, 47, 55, 48, 49, 56, 57, 58, 59, 60, 61, 62, 63
                    ],
                    n_rows,
                    n_cols
                ),
                build_queen_board_from_inds(&[27], n_rows, n_cols),
                build_queen_board_from_inds(&[40, 41, 42, 43, 50, 51, 52, 53, 54], n_rows, n_cols),
            ]
        );
    }

    #[test]
    fn test_regions_are_columns() {
        // In this test, the color regions are the rows, and we test that it can
        // successfully return any solution
        let raw_color_regions =
            "12345678 12345678 12345678 12345678 12345678 12345678 12345678 12345678";
        // Make sure it returns sucess
        let res = solve(raw_color_regions, false);
        assert!(res.0.is_some());
    }

    #[test]
    fn test_regions_are_rows() {
        // In this test, the color regions are the rows, and we test that it can
        // successfully return any solution
        let raw_color_regions =
            "11111111 22222222 33333333 44444444 55555555 66666666 77777777 88888888";
        let res = solve(raw_color_regions, false);
        assert!(res.0.is_some());
    }

    #[test]
    fn test_actual_board() {
        // In this test, we use an actual board
        let raw_color_regions =
            "11112333 11222344 11255346 77253344 73355334 77335344 87355333 77333333";
        let res = solve(raw_color_regions, false);
        assert!(res.0.is_some());
    }

    fn get_inds_from_u64(n: u64) -> Vec<usize> {
        let mut result = Vec::new();
        // Iterate over the bits in the board, and return the index of each bit set to 1
        for i in 0..64 {
            if n & (1 << i) != 0 {
                result.push(i);
            }
        }
        result
    }
}

/// Take a set of indices, and insert each into a bitset.
pub fn build_bit_set_from_inds(inds: &[u64]) -> u64 {
    // Make sure all indices are valid
    assert!(inds.iter().all(|&idx| idx < 64));

    let mut bitset = 0;
    for &idx in inds {
        bitset |= 1 << idx;
    }
    bitset
}

/// Take a set of indices, and insert each into a bitset.
pub fn build_queen_board_from_inds(inds: &[u64], n_rows: usize, n_cols: usize) -> QueenBoard {
    let mut board = QueenBoard::new(n_rows, n_cols);
    for &spot_ind in inds {
        board.set_linear_index(spot_ind as usize, true);
    }
    board
}

/// The user passes in the color regions, as a `HashMap<char, Vec<u64>>`. For each pair
/// in the hash map, a `QueenBoard` is created.
pub fn parse_color_region_boards(
    input: &HashMap<char, Vec<u64>>,
    n_rows: usize,
    n_cols: usize,
) -> Vec<QueenBoard> {
    // Create the vector to hold the boards
    let mut boards: Vec<QueenBoard> = Vec::with_capacity(input.len());

    // Create the empty boards inside the vector
    for _ in 0..input.len() {
        boards.push(QueenBoard::new(n_rows, n_cols));
    }

    // Iterate over each of the color regions
    for (cr_idx, inds) in input.values().enumerate() {
        // For this color region, set the inds to true
        for &spot_ind in inds {
            boards[cr_idx].set_linear_index(spot_ind as usize, true);
        }
    }

    boards
}

/// The user passes in the color regions by assigning letters to each region.
/// Then they enter each row, left to right, top to bottom, with a space between the
/// rows. This function returns the color regions as numbers for simplicity.
pub fn parse_color_region_inds(input: &str) -> (HashMap<char, Vec<u64>>, usize, usize) {
    // How many rows does this input array have?
    let n_rows = input.split_whitespace().count();
    println!("Found {n_rows} rows in the input");

    // How many columns does this input array have?
    let n_cols = input.split_whitespace().next().unwrap().len();
    println!("Found {n_cols} columns in the input");

    // How many unique characters are there? Remove the whitespace, and then count how
    // many unique characters are left
    let n_unique_chars = input
        .replace(" ", "")
        .chars()
        .collect::<HashSet<char>>()
        .len();
    println!("Found {n_unique_chars} unique characters in the input");

    // Create a hashmap to store the indices of each color region
    let mut regions: HashMap<char, Vec<u64>> = HashMap::new();

    for (row_idx, row) in input.split_whitespace().enumerate() {
        for (col_idx, id) in row.chars().enumerate() {
            let region = regions.entry(id).or_default();
            let linear_idx = (row_idx * n_cols) + col_idx;
            region.push(
                linear_idx
                    .try_into()
                    .expect("Could not convert usize to u64"),
            );
        }
    }

    (regions, n_rows, n_cols)
}

/// Solve the puzzle by brute force, attempting all possible combinations until one
/// works. Return a vector of each queen's index (not a mask), or None if it failed to
/// find a solution.
pub fn solve(raw_color_regions: &str, verbose: bool) -> (Option<Vec<u64>>, usize) {
    // First parse the regions into a nested vec of the indices that make up this color
    // region
    let (color_regions, n_rows, n_cols) = parse_color_region_inds(raw_color_regions);

    // Get just the indices
    let color_region_inds: Vec<Vec<u64>> = color_regions.values().cloned().collect();

    // Create QueenBoards for each color region
    let color_region_boards = parse_color_region_boards(&color_regions, n_rows, n_cols);

    // Let the user know how many possible positions are being checked.
    if verbose {
        let possible_combos: usize = color_region_inds
            .iter()
            .map(|region| region.len())
            .product();
        // Format it with commas every 3 digits
        let formatted_combo = format_thousands(possible_combos);
        println!("Will search up to {formatted_combo} positions");
    }

    // A mutable board to reduce allocations
    let mut b = QueenBoard::new(n_rows, n_cols);
    let mut gidx: usize = 0;

    'outer: for queen_placement in color_region_inds.iter().multi_cartesian_product() {
        // Update the global index
        gidx += 1;

        // Make sure the board is empty
        b.clear();

        // Try this placement by placing one queen at a time
        for (idx, &&queen_idx) in queen_placement.iter().enumerate() {
            // Grab a copy of the color region board for this queen
            let color_region = color_region_boards[idx];

            // Attempt to place the queen in the color region
            match b.place_queen(queen_idx as usize, &color_region) {
                BoardPlacementResult::Success(board) => {
                    // We were able to place this queen.
                    // Update the board with the new queen's position
                    b = board;
                }
                BoardPlacementResult::SpotOccupied => {
                    // Overlap, cannot do this spot, try next placement
                    continue 'outer;
                }
                BoardPlacementResult::NotInColorRegion => {
                    unreachable!("The queen is not in the color region it should be in")
                }
                BoardPlacementResult::IndexOutOfBounds => {
                    unreachable!("The queen is not in the color region it should be in")
                }
                BoardPlacementResult::DimensionMismatch => {
                    unreachable!("Dimension mismatch")
                }
            }
        }

        return (Some(queen_placement.iter().map(|&&q| q).collect()), gidx);
    }

    // If we get here, we did not find a solution
    (None, gidx)
}

/// Print out the state of a board by placing an 'X' wherever one of the bits in the u64
/// is set to 1, and a '.' wherever it is set to 0.
pub fn disp_u64(board: u64) {
    for row in 0..8 {
        for col in 0..8 {
            let idx = row * 8 + col;
            let mask = 1 << idx;
            if board & mask == 0 {
                print!(". ");
            } else {
                print!("X ");
            }
        }
        println!();
    }
}

/// Format a number with commas every 3 digits
pub fn format_thousands(n: usize) -> String {
    // Format it with commas every 3 digits
    n.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",")
}
