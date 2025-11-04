use bit_board::bitboard::BitBoard;
use bit_board::bitboardstatic::BitBoardStatic;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt;

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

/// A `BitBoard` for playing Queens. Assumes boards are not larger than 192
/// positions (64 * 3).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QueenBoard(BitBoardStatic<3>);

impl fmt::Display for QueenBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // column indices
        write!(f, "   ")?; // space for row labels
        for col in 0..self.0.n_cols() {
            write!(f, "{}", col % 15)?; // wrap every 15 for readability
        }
        writeln!(f)?;

        for row in 0..self.0.n_rows() {
            // row index, right-aligned to 2 spaces
            write!(f, "{:>2} ", row)?;
            for col in 0..self.0.n_cols() {
                let bit = self.0.get(row, col);
                let c = if bit { 'X' } else { '.' };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl QueenBoard {
    /// Create a new board
    #[must_use]
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

    /// Check that none of the spots in the one away diagonals are set
    #[must_use]
    pub fn one_off_diagonals_are_empty(&self, linear_idx: usize) -> bool {
        // First get the row and column of this `linear_idx`
        let (row, col) = self.0.row_col_of(linear_idx);

        // Then get the spots above left, above right, below left, below right
        // assuming there is such a spot on the board

        // Above left
        if (row > 0) && (col > 0) && self.0.get(row - 1, col - 1) {
            return false;
        }

        // Above right
        if (row > 0) && (col < self.0.n_cols() - 1) && self.0.get(row - 1, col + 1) {
            return false;
        }

        // Below left
        if (row < self.0.n_rows() - 1) && (col > 0) && self.0.get(row + 1, col - 1) {
            return false;
        }

        // Below right
        if (row < self.0.n_rows() - 1)
            && (col < self.0.n_cols() - 1)
            && self.0.get(row + 1, col + 1)
        {
            return false;
        }

        true
    }

    /// Check that this row is empty
    #[must_use]
    pub fn row_is_empty(&self, row: usize) -> bool {
        // Use the bitboard's method to inspect each item in the row.
        // Come here and use the underlying bitvec get() method over
        // a range of indices if more performance is required.
        self.0.get_row(row).all(|item| !item)
    }

    /// Check that this column is empty
    #[must_use]
    pub fn col_is_empty(&self, col: usize) -> bool {
        self.0.get_col(col).all(|item| !item)
    }
}

/// Take a set of indices, and insert each into a bitset.
#[must_use]
pub fn build_bit_set_from_inds(inds: &[usize]) -> u64 {
    let mut bitset = 0;
    for &idx in inds {
        bitset |= 1 << idx;
    }
    bitset
}

/// Take a set of indices, and insert each into a bitset.
#[must_use]
pub fn build_queen_board_from_inds(inds: &[usize], n_rows: usize, n_cols: usize) -> QueenBoard {
    let mut board = QueenBoard::new(n_rows, n_cols);
    for &spot_ind in inds {
        board.set_linear_index(spot_ind, true);
    }
    board
}

/// The user passes in the color regions, as a `HashMap<char, Vec<u64>>`. For each pair
/// in the hash map, a `QueenBoard` is created.
#[must_use]
pub fn parse_color_region_boards(
    input: &HashMap<char, Vec<usize>>,
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
            boards[cr_idx].set_linear_index(spot_ind, true);
        }
    }

    boards
}

/// The user passes in the color regions by assigning letters to each region.
/// Then they enter each row, left to right, top to bottom, with a space between the
/// rows. This function returns the color regions as numbers for simplicity.
///
/// # Panics
///
/// Panics if no whitespace found in the input
#[must_use]
pub fn parse_color_region_inds(input: &str) -> (HashMap<char, Vec<usize>>, usize, usize) {
    // How many rows does this input array have?
    let n_rows = input.split_whitespace().count();
    // println!("Found {n_rows} rows in the input");

    // How many columns does this input array have?
    let n_cols = input.split_whitespace().next().unwrap().len();
    // println!("Found {n_cols} columns in the input");

    // How many unique characters are there? Remove the whitespace, and then count how
    // many unique characters are left
    let n_unique_chars = input
        .replace(' ', "")
        .chars()
        .collect::<HashSet<char>>()
        .len();
    println!("Found {n_unique_chars} unique characters in the input");

    // Create a hashmap to store the indices of each color region
    let mut regions: HashMap<char, Vec<usize>> = HashMap::new();

    for (row_idx, row) in input.split_whitespace().enumerate() {
        for (col_idx, id) in row.chars().enumerate() {
            let region = regions.entry(id).or_default();
            let linear_idx = (row_idx * n_cols) + col_idx;
            region.push(linear_idx);
        }
    }

    (regions, n_rows, n_cols)
}

/// Solve the puzzle by brute force, attempting all possible combinations until one
/// works. Return a vector of each queen's index (not a mask), or None if it failed to
/// find a solution.
pub fn solve(raw_color_regions: &str, verbose: bool) -> (Option<QueenBoard>, usize) {
    // First parse the regions into a nested vec of the indices that make up this color
    // region
    let (color_regions, n_rows, n_cols) = parse_color_region_inds(raw_color_regions);

    // Get just the indices
    let color_region_inds: Vec<Vec<usize>> = color_regions.values().cloned().collect();

    // Let the user know how many possible positions are being checked.
    if verbose {
        let possible_combos: usize = color_region_inds.iter().map(Vec::len).product();
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

        // Add the queens of this `queen_placement` to the board
        for queen_idx in &queen_placement {
            b.set_linear_index(**queen_idx, true);
        }

        // For each queen, check that no other queen is in its
        // row, column, or one-away diagonals.
        for queen_idx in &queen_placement {
            // Remove this queen from the board for now so we don't accidentally
            // count it
            b.set_linear_index(**queen_idx, false);

            // If there is a queen in one of the diagonal spots, continue
            // to the next set of placements
            if !b.one_off_diagonals_are_empty(**queen_idx) {
                continue 'outer;
            }

            // Get the row and column for this spot
            let (row, col) = b.0.row_col_of(**queen_idx);

            // If there is a queen in this row, continue to the next set of
            // placements
            if !b.row_is_empty(row) {
                continue 'outer;
            }

            // If there is a queen in this column, continue to the next
            // set of placements
            if !b.col_is_empty(col) {
                continue 'outer;
            }

            // If we made it this far, then "replace" this queen
            b.set_linear_index(**queen_idx, true);
        }

        // If we made it this far, then this is a valid set of placements
        return (Some(b), gidx);
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
///
/// # Panics
///
/// Panics if digits cannot be parsed into UTF8
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

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    // #[test]
    // fn test_good_region_input() {
    //     let string = "11233456 12234456 11233456 12273456 11233456 88885556 66888886 66666666";
    //     let (map, n_rows, n_cols) = parse_color_region_inds(string);
    //     let regions = parse_color_region_boards(&map, n_rows, n_cols);
    //     assert_eq!(
    //         regions,
    //         [
    //             build_queen_board_from_inds(&[0, 1, 8, 16, 17, 24, 32, 33], n_rows, n_cols),
    //             build_queen_board_from_inds(&[2, 9, 10, 18, 25, 26, 34], n_rows, n_cols),
    //             build_queen_board_from_inds(&[3, 4, 11, 19, 20, 28, 35, 36], n_rows, n_cols),
    //             build_queen_board_from_inds(&[5, 12, 13, 21, 29, 37], n_rows, n_cols),
    //             build_queen_board_from_inds(&[6, 14, 22, 30, 38, 44, 45, 46], n_rows, n_cols),
    //             build_queen_board_from_inds(
    //                 &[
    //                     7, 15, 23, 31, 39, 47, 55, 48, 49, 56, 57, 58, 59, 60, 61, 62, 63
    //                 ],
    //                 n_rows,
    //                 n_cols
    //             ),
    //             build_queen_board_from_inds(&[27], n_rows, n_cols),
    //             build_queen_board_from_inds(&[40, 41, 42, 43, 50, 51, 52, 53, 54], n_rows, n_cols),
    //         ]
    //     );
    // }

    #[test]
    fn test_regions_are_columns() {
        // In this test, the color regions are the columns, and we test that it can
        // successfully return any solution
        let raw_color_regions = "12345 12345 12345 12345 12345 ";
        // Make sure it returns sucess
        let res = solve(raw_color_regions, false);
        assert!(res.0.is_some());
    }

    #[test]
    fn test_regions_are_rows() {
        // In this test, the color regions are the rows, and we test that it can
        // successfully return any solution
        let raw_color_regions = "11111 22222 33333 44444 55555";
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

    #[rstest]
    #[case(0, &[], true, "empty board")]
    #[case(4, &[0], false, "above left occupied")]
    #[case(4, &[2], false, "above right occupied")]
    #[case(4, &[6], false, "below left occupied")]
    #[case(4, &[8], false, "below right occupied")]
    #[case(4, &[1, 3, 5, 7], true, "no diagonals")]
    #[case(9, &[0, 1], true, "place to check is off the board")]
    fn test_one_off_diagonals_are_empty(
        #[case] idx: u64,
        #[case] initial_placements: &[usize],
        #[case] expected: bool,
        #[case] description: &str,
    ) {
        let board = build_queen_board_from_inds(initial_placements, 3, 3);
        assert_eq!(
            board.one_off_diagonals_are_empty(idx as usize),
            expected,
            "{}",
            description
        );
    }

    #[rstest]
    #[case(0, &[], true, "empty board")]
    #[case(1, &[], true, "empty board")]
    #[case(0, &[2, 3], true, "empty row")]
    #[case(1, &[0, 1], true, "empty row")]
    #[case(0, &[0], false, "row is not empty")]
    #[case(0, &[0, 1], false, "row is not empty")]
    #[case(0, &[1], false, "row is not empty")]
    #[case(1, &[2], false, "row is not empty")]
    #[case(1, &[2, 3], false, "row is not empty")]
    #[case(1, &[3], false, "row is not empty")]
    fn test_row_is_empty(
        #[case] row: usize,
        #[case] initial_placements: &[usize],
        #[case] expected: bool,
        #[case] description: &str,
    ) {
        let board = build_queen_board_from_inds(initial_placements, 2, 2);
        assert_eq!(expected, board.row_is_empty(row), "{}", description);
    }

    #[rstest]
    #[case(0, &[], true, "empty board")]
    #[case(1, &[], true, "empty board")]
    #[case(0, &[1, 3], true, "empty col")]
    #[case(1, &[0, 2], true, "empty col")]
    #[case(0, &[0], false, "col is not empty")]
    #[case(0, &[0, 2], false, "col is not empty")]
    #[case(0, &[2], false, "col is not empty")]
    #[case(1, &[1], false, "col is not empty")]
    #[case(1, &[1, 3], false, "col is not empty")]
    #[case(1, &[3], false, "col is not empty")]
    fn test_col_is_empty(
        #[case] col: usize,
        #[case] initial_placements: &[usize],
        #[case] expected: bool,
        #[case] description: &str,
    ) {
        let board = build_queen_board_from_inds(initial_placements, 2, 2);
        assert_eq!(expected, board.col_is_empty(col), "{}", description);
    }
    // fn get_inds_from_u64(n: u64) -> vec<usize> {
    //     let mut result = vec::new();
    //     // iterate over the bits in the board, and return the index of each bit set to 1
    //     for i in 0..64 {
    //         if n & (1 << i) != 0 {
    //             result.push(i);
    //         }
    //     }
    //     result
    // }
}
