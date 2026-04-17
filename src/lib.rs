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
            write!(f, "{row:>2} ")?;
            for col in 0..self.0.n_cols() {
                let bit = self.0.get(row, col);
                let c = if bit { 'X' } else { '.' };
                write!(f, "{c}")?;
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

    // A valid Queens board has exactly one queen per row, column, and color
    // region, so the number of distinct region characters must equal n_cols.
    let n_unique_chars = input
        .replace(' ', "")
        .chars()
        .collect::<HashSet<char>>()
        .len();
    assert_eq!(
        n_unique_chars, n_cols,
        "expected {n_cols} color regions (one per column), found {n_unique_chars}"
    );

    // Verify all rows have the same number of columns
    for (row_idx, row) in input.split_whitespace().enumerate() {
        if row.len() != n_cols {
            panic!(
                "Row {row_idx} has {} columns, but expected {n_cols} (based on the first row)",
                row.len()
            );
        }
    }

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

/// Validate that `board` is a correct Queens solution for `color_regions`.
///
/// Checks: total queen count equals the number of regions; exactly one queen per row,
/// per column, and per region; and no two queens are one-off diagonally adjacent.
#[must_use]
pub fn is_valid_solution(board: &QueenBoard, color_regions: &[Vec<usize>]) -> bool {
    let queen_inds: Vec<usize> = board.get_linear_indices().collect();
    let n = color_regions.len();

    if queen_inds.len() != n {
        return false;
    }

    let mut rows = HashSet::with_capacity(n);
    let mut cols = HashSet::with_capacity(n);
    for &idx in &queen_inds {
        let (row, col) = board.0.row_col_of(idx);
        if !rows.insert(row) || !cols.insert(col) {
            return false;
        }
    }

    for &idx in &queen_inds {
        if !board.one_off_diagonals_are_empty(idx) {
            return false;
        }
    }

    let queen_set: HashSet<usize> = queen_inds.into_iter().collect();
    for region in color_regions {
        let count = region.iter().filter(|i| queen_set.contains(i)).count();
        if count != 1 {
            return false;
        }
    }

    true
}

/// Public entry point for solving a Queens puzzle. Currently dispatches to the
/// brute-force solver; will be switched to `solve_backtracking` once that lands.
pub fn solve(raw_color_regions: &str, verbose: bool) -> (Option<QueenBoard>, usize) {
    solve_brute_force(raw_color_regions, verbose)
}

/// Backtracking solver. Not yet implemented — returns `(None, 0)` as a placeholder
/// so test scaffolding can be written against the intended API.
pub fn solve_backtracking(_raw_color_regions: &str, _verbose: bool) -> (Option<QueenBoard>, usize) {
    (None, 0)
}

/// Solve the puzzle by brute force, attempting all possible combinations until one
/// works. Return a vector of each queen's index (not a mask), or None if it failed to
/// find a solution.
pub fn solve_brute_force(raw_color_regions: &str, verbose: bool) -> (Option<QueenBoard>, usize) {
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

    /// Parse raw color regions into the `Vec<Vec<usize>>` form the validator expects.
    fn regions_as_vec(raw: &str) -> Vec<Vec<usize>> {
        let (regions, _, _) = parse_color_region_inds(raw);
        regions.values().cloned().collect()
    }

    #[test]
    fn test_regions_are_columns() {
        let raw = "12345 12345 12345 12345 12345";
        let regions = regions_as_vec(raw);
        let (board_opt, _) = solve(raw, false);
        let board = board_opt.expect("expected a solution");
        assert!(is_valid_solution(&board, &regions));
    }

    #[test]
    fn test_regions_are_rows() {
        let raw = "11111 22222 33333 44444 55555";
        let regions = regions_as_vec(raw);
        let (board_opt, _) = solve(raw, false);
        let board = board_opt.expect("expected a solution");
        assert!(is_valid_solution(&board, &regions));
    }

    #[test]
    fn test_actual_board() {
        let raw = "11112333 11222344 11255346 77253344 73355334 77335344 87355333 77333333";
        let regions = regions_as_vec(raw);
        let (board_opt, _) = solve(raw, false);
        let board = board_opt.expect("expected a solution");
        assert!(is_valid_solution(&board, &regions));
    }

    /// Correctness table: for each input, `solve` must agree with `expected_solvable`,
    /// and any returned board must pass the validator. Targets `solve`, which is
    /// currently the brute-force solver and will later dispatch to the backtracker —
    /// the same table exercises both phases.
    #[rstest]
    #[case("1", true, "1x1 trivial")]
    #[case("11 22", false, "2x2 rows unsolvable")]
    #[case("111 222 333", false, "3x3 rows unsolvable")]
    #[case("1111 2222 3333 4444", true, "4x4 rows solvable")]
    #[case("12345 12345 12345 12345 12345", true, "5x5 cols")]
    #[case("11111 22222 33333 44444 55555", true, "5x5 rows")]
    #[case(
        "11112333 11222344 11255346 77253344 73355334 77335344 87355333 77333333",
        true,
        "8x8 real puzzle"
    )]
    #[case("12c ccc ccc", false, "forced A(0,0)+B(0,1) same-row conflict")]
    fn test_solve_correctness_table(
        #[case] raw: &str,
        #[case] expected_solvable: bool,
        #[case] description: &str,
    ) {
        let regions = regions_as_vec(raw);
        let (board_opt, _) = solve(raw, false);
        assert_eq!(
            board_opt.is_some(),
            expected_solvable,
            "solvability mismatch: {description}"
        );
        if let Some(board) = board_opt {
            assert!(
                is_valid_solution(&board, &regions),
                "returned board is invalid: {description}"
            );
        }
    }

    /// Real-world regression puzzles. Each case is a Queens puzzle known to be
    /// solvable; the test asserts `solve` produces a board that validates. Add new
    /// puzzles here as they're encountered in the wild.
    #[rstest]
    #[case(
        "11112333 11222344 11255346 77253344 73355334 77335344 87355333 77333333",
        "original 8x8 from test_actual_board"
    )]
    #[case(
        "aabccefg abbceefg aabccefg abbgcdef aabccedf hhhheeef ffhhhhhf ffffffff",
        "8x8 from main.rs docstring example"
    )]
    fn test_real_world_puzzles(#[case] raw: &str, #[case] description: &str) {
        let regions = regions_as_vec(raw);
        let (board_opt, _) = solve(raw, false);
        let board = board_opt.unwrap_or_else(|| panic!("expected solution for: {description}"));
        assert!(
            is_valid_solution(&board, &regions),
            "returned board is invalid: {description}"
        );
    }

    /// Equivalence: brute-force and backtracking must agree on solvability, and both
    /// returned boards (when present) must pass the validator. The `#[ignore]` will be
    /// lifted once `solve_backtracking` is implemented.
    #[rstest]
    #[case("12345 12345 12345 12345 12345")]
    #[case("11111 22222 33333 44444 55555")]
    #[case("11112333 11222344 11255346 77253344 73355334 77335344 87355333 77333333")]
    #[ignore = "enable after solve_backtracking is implemented"]
    fn test_solvers_agree(#[case] raw: &str) {
        let regions = regions_as_vec(raw);
        let (bf, _) = solve_brute_force(raw, false);
        let (bt, _) = solve_backtracking(raw, false);
        assert_eq!(bf.is_some(), bt.is_some(), "solvability disagreement");
        if let Some(b) = bf {
            assert!(
                is_valid_solution(&b, &regions),
                "brute-force returned invalid"
            );
        }
        if let Some(b) = bt {
            assert!(
                is_valid_solution(&b, &regions),
                "backtracking returned invalid"
            );
        }
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

    #[test]
    #[should_panic(expected = "Row 1 has 2 columns, but expected 3")]
    fn test_parse_mismatched_row_lengths() {
        let _ = parse_color_region_inds("111 22 333");
    }

    #[test]
    fn test_parse_uniform_row_lengths() {
        let (regions, n_rows, n_cols) = parse_color_region_inds("123 123 123");
        assert_eq!(n_rows, 3);
        assert_eq!(n_cols, 3);
        assert_eq!(regions.len(), 3);
    }

    #[test]
    #[should_panic(expected = "expected 3 color regions")]
    fn test_parse_rejects_wrong_region_count() {
        // 3x3 grid but 9 unique chars — violates the Queens invariant.
        let _ = parse_color_region_inds("123 456 789");
    }

    /// A known-valid 5x5 solution: queens at (0,0), (1,2), (2,4), (3,1), (4,3).
    /// Regions are the rows.
    fn known_valid_5x5() -> (QueenBoard, Vec<Vec<usize>>) {
        let queens = [0, 7, 14, 16, 23];
        let board = build_queen_board_from_inds(&queens, 5, 5);
        let regions = vec![
            vec![0, 1, 2, 3, 4],
            vec![5, 6, 7, 8, 9],
            vec![10, 11, 12, 13, 14],
            vec![15, 16, 17, 18, 19],
            vec![20, 21, 22, 23, 24],
        ];
        (board, regions)
    }

    #[test]
    fn test_validator_accepts_valid_5x5() {
        let (board, regions) = known_valid_5x5();
        assert!(is_valid_solution(&board, &regions));
    }

    #[test]
    fn test_validator_accepts_trivial_1x1() {
        let board = build_queen_board_from_inds(&[0], 1, 1);
        let regions = vec![vec![0]];
        assert!(is_valid_solution(&board, &regions));
    }

    #[test]
    fn test_validator_rejects_too_few_queens() {
        let board = build_queen_board_from_inds(&[0, 7], 5, 5);
        let (_, regions) = known_valid_5x5();
        assert!(!is_valid_solution(&board, &regions));
    }

    #[test]
    fn test_validator_rejects_too_many_queens() {
        let board = build_queen_board_from_inds(&[0, 7, 14, 16, 23, 11], 5, 5);
        let (_, regions) = known_valid_5x5();
        assert!(!is_valid_solution(&board, &regions));
    }

    #[test]
    fn test_validator_rejects_duplicate_row() {
        // Queens at (0,0) and (0,1) — same row.
        let board = build_queen_board_from_inds(&[0, 1], 4, 4);
        let regions = vec![vec![0], vec![1]];
        assert!(!is_valid_solution(&board, &regions));
    }

    #[test]
    fn test_validator_rejects_duplicate_column() {
        // Queens at (0,0) and (1,0) — same column.
        let board = build_queen_board_from_inds(&[0, 4], 4, 4);
        let regions = vec![vec![0], vec![4]];
        assert!(!is_valid_solution(&board, &regions));
    }

    #[test]
    fn test_validator_rejects_one_off_diagonal() {
        // Queens at (0,0) and (1,1) — one-off diagonal.
        let board = build_queen_board_from_inds(&[0, 5], 4, 4);
        let regions = vec![vec![0], vec![5]];
        assert!(!is_valid_solution(&board, &regions));
    }

    #[test]
    fn test_validator_rejects_two_queens_in_one_region() {
        // Valid board placement from known_valid_5x5, but regions are rigged so that
        // region A holds two queens and region E holds zero.
        let (board, _) = known_valid_5x5();
        let regions = vec![
            vec![0, 16],   // two queens here (indices 0 and 16)
            vec![7],       // one queen
            vec![14],      // one queen
            vec![23],      // one queen
            vec![1, 2, 3], // zero queens
        ];
        assert!(!is_valid_solution(&board, &regions));
    }
}
