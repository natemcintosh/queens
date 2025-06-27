/// Represents an 8x8 board, where each bit in the `u64` represents one spot. Indexing
/// goes from left to right, top to bottom. If a bit is 0, that means it is "open" for
/// placement. If it is 1, that means it is "occupied" by a queen OR is blocked by a
/// queen.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Board(u64);

use std::fmt;

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..8 {
            for col in 0..8 {
                let idx = row * 8 + col;
                let mask = 1 << idx;
                if self.0 & mask == 0 {
                    write!(f, ". ")?;
                } else {
                    write!(f, "X ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq)]
pub enum BoardPlacementResult {
    /// The queen was successfully placed on the board.
    Success(Board),
    /// The queen was not placed because the spot was already claimed by a queen.
    SpotOccupied,
    /// The queen was not placed because it was not in the color region.
    NotInColorRegion,
    /// The queen was not placed because the index at which it was attempted to be
    /// placed was out of bounds.
    IndexOutOfBounds,
}

impl Board {
    /// Creates a new empty board.
    pub fn new() -> Self {
        Board(0)
    }

    /// If a queen can be placed at index `queen_idx` on the board, place it and return
    /// the updated board as `Some(Board)`. If not, return `None`. If the queen can be
    /// placed, the board is updated by setting the bit at `queen_idx` to 1, as well as
    /// all of the bits representing the row, column, and color region that the queen is
    /// in.
    pub fn place_queen(&self, queen_idx: u64, color_region: u64) -> BoardPlacementResult {
        // Make sure the index is within bounds
        if queen_idx >= 64 {
            return BoardPlacementResult::IndexOutOfBounds;
        }

        // Create a mask with a single bit set at the queen_idx index
        let queen_only_mask = 1 << queen_idx;

        // Make sure the queen is in the color region
        if color_region != 0 && color_region & queen_only_mask == 0 {
            return BoardPlacementResult::NotInColorRegion;
        }

        if self.0 & queen_only_mask == 0 {
            // If the spot is empty, block off all of the spots in the same row, same
            // column, and same color region.
            BoardPlacementResult::Success(Board(
                self.0 | self.fill_queen_reach(queen_idx, color_region),
            ))
        } else {
            BoardPlacementResult::SpotOccupied
        }
    }

    /// For a given queen spot, fill in all of the spots that the queen can reach
    fn fill_queen_reach(&self, queen_idx: u64, color_region_mask: u64) -> u64 {
        let row_mask = self.fill_row(queen_idx);
        let col_mask = self.fill_column(queen_idx);
        let diagonal_mask = self.fill_diagonals(queen_idx);
        row_mask | col_mask | diagonal_mask | color_region_mask
    }

    /// Fills in the spots to the 4 diagonal squares of the index. If along the edges,
    /// and no such diagonal exists on the board, skip that diagonal.
    fn fill_diagonals(&self, queen_idx: u64) -> u64 {
        let row = queen_idx / 8;
        let col = queen_idx % 8;

        let mut mask = 0;

        // Fill in the top-left diagonal
        if row > 0 && col > 0 {
            // The index is up one row (-8) and left one (-1)
            mask |= 1 << (queen_idx - 8 - 1);
        }

        // Fill in the top-right diagonal
        if row > 0 && col < 7 {
            // The index is up one row (-8) and right one (+1)
            mask |= 1 << (queen_idx - 8 + 1);
        }

        // Fill in the bottom-left diagonal
        if row < 7 && col > 0 {
            // The index is down one row (+8) and left one (-1)
            mask |= 1 << (queen_idx + 8 - 1);
        }

        // Fill in the bottom-right diagonal
        if row < 7 && col < 7 {
            // The index is down one row (+8) and right one (+1)
            mask |= 1 << (queen_idx + 8 + 1);
        }

        mask
    }

    /// Fill in all of the bits that represent the row of the given spot index, where
    /// the idx represents the index (as a value) at which the queen is located.
    fn fill_row(&self, idx: u64) -> u64 {
        // Calculate which row this spot is in (0-7)
        let row = idx / 8;
        // Create a mask with all 8 bits set for this row
        0xFF_u64 << (row * 8)
    }

    /// Fill in all of the bits that represent the column of the given spot index, where
    /// the idx represents the index (as a value) at which the queen is located.
    fn fill_column(&self, idx: u64) -> u64 {
        // Calculate which column this spot is in (0-7)
        let col = idx % 8;
        // Create a mask with the bit pattern for this column repeated across all rows
        // Each row needs the same column bit set, so we use the pattern (1 << col) repeated 8 times
        (1u64 << col) * 0x0101010101010101u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, build_bit_set_from_inds(&[9]))]
    #[case(1, build_bit_set_from_inds(&[8, 10]))]
    #[case(2, build_bit_set_from_inds(&[9, 11]))]
    #[case(7, build_bit_set_from_inds(&[14]))]
    #[case(8, build_bit_set_from_inds(&[1, 17]))]
    #[case(9, build_bit_set_from_inds(&[0, 2, 16, 18]))]
    #[case(56, build_bit_set_from_inds(&[49]))]
    #[case(57, build_bit_set_from_inds(&[48, 50]))]
    #[case(62, build_bit_set_from_inds(&[53, 55]))]
    #[case(63, build_bit_set_from_inds(&[54]))]
    #[case(28, build_bit_set_from_inds(&[19, 21, 35, 37]))]
    fn test_fill_diagonals(#[case] queen_idx: u64, #[case] want_diags: u64) {
        let board = Board::new();
        let got = board.fill_diagonals(queen_idx);
        assert_eq!(got, want_diags);
    }

    #[rstest]
    #[case(0, 0b11111111)]
    #[case(1, 0b11111111)]
    #[case(2, 0b11111111)]
    #[case(3, 0b11111111)]
    #[case(4, 0b11111111)]
    #[case(5, 0b11111111)]
    #[case(6, 0b11111111)]
    #[case(7, 0b11111111)]
    #[case(8, 0b11111111_00000000)]
    #[case(9, 0b11111111_00000000)]
    #[case(10, 0b11111111_00000000)]
    #[case(11, 0b11111111_00000000)]
    #[case(12, 0b11111111_00000000)]
    #[case(13, 0b11111111_00000000)]
    #[case(14, 0b11111111_00000000)]
    #[case(15, 0b11111111_00000000)]
    #[case(16, 0b11111111_00000000_00000000)]
    #[case(17, 0b11111111_00000000_00000000)]
    #[case(18, 0b11111111_00000000_00000000)]
    #[case(19, 0b11111111_00000000_00000000)]
    #[case(20, 0b11111111_00000000_00000000)]
    #[case(21, 0b11111111_00000000_00000000)]
    #[case(22, 0b11111111_00000000_00000000)]
    #[case(23, 0b11111111_00000000_00000000)]
    #[case(24, 0b11111111_00000000_00000000_00000000)]
    #[case(25, 0b11111111_00000000_00000000_00000000)]
    #[case(26, 0b11111111_00000000_00000000_00000000)]
    #[case(27, 0b11111111_00000000_00000000_00000000)]
    #[case(28, 0b11111111_00000000_00000000_00000000)]
    #[case(29, 0b11111111_00000000_00000000_00000000)]
    #[case(30, 0b11111111_00000000_00000000_00000000)]
    #[case(31, 0b11111111_00000000_00000000_00000000)]
    #[case(32, 0b11111111_00000000_00000000_00000000_00000000)]
    #[case(33, 0b11111111_00000000_00000000_00000000_00000000)]
    #[case(34, 0b11111111_00000000_00000000_00000000_00000000)]
    #[case(35, 0b11111111_00000000_00000000_00000000_00000000)]
    #[case(36, 0b11111111_00000000_00000000_00000000_00000000)]
    #[case(37, 0b11111111_00000000_00000000_00000000_00000000)]
    #[case(38, 0b11111111_00000000_00000000_00000000_00000000)]
    #[case(39, 0b11111111_00000000_00000000_00000000_00000000)]
    #[case(40, 0b11111111_00000000_00000000_00000000_00000000_00000000)]
    #[case(41, 0b11111111_00000000_00000000_00000000_00000000_00000000)]
    #[case(42, 0b11111111_00000000_00000000_00000000_00000000_00000000)]
    #[case(43, 0b11111111_00000000_00000000_00000000_00000000_00000000)]
    #[case(44, 0b11111111_00000000_00000000_00000000_00000000_00000000)]
    #[case(45, 0b11111111_00000000_00000000_00000000_00000000_00000000)]
    #[case(46, 0b11111111_00000000_00000000_00000000_00000000_00000000)]
    #[case(47, 0b11111111_00000000_00000000_00000000_00000000_00000000)]
    #[case(48, 0b11111111_00000000_00000000_00000000_00000000_00000000_00000000)]
    #[case(49, 0b11111111_00000000_00000000_00000000_00000000_00000000_00000000)]
    #[case(50, 0b11111111_00000000_00000000_00000000_00000000_00000000_00000000)]
    #[case(51, 0b11111111_00000000_00000000_00000000_00000000_00000000_00000000)]
    #[case(52, 0b11111111_00000000_00000000_00000000_00000000_00000000_00000000)]
    #[case(53, 0b11111111_00000000_00000000_00000000_00000000_00000000_00000000)]
    #[case(54, 0b11111111_00000000_00000000_00000000_00000000_00000000_00000000)]
    #[case(55, 0b11111111_00000000_00000000_00000000_00000000_00000000_00000000)]
    #[case(
        56,
        0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000
    )]
    #[case(
        57,
        0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000
    )]
    #[case(
        58,
        0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000
    )]
    #[case(
        59,
        0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000
    )]
    #[case(
        60,
        0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000
    )]
    #[case(
        61,
        0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000
    )]
    #[case(
        62,
        0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000
    )]
    #[case(
        63,
        0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000
    )]
    fn test_fill_row(#[case] spot_idx: u64, #[case] want: u64) {
        let board = Board(0);
        let filled_row = board.fill_row(spot_idx);
        assert_eq!(filled_row, want);
    }

    #[rstest]
    #[case(
        0,
        0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001
    )]
    #[case(
        1,
        0b00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000010
    )]
    #[case(
        2,
        0b00000100_00000100_00000100_00000100_00000100_00000100_00000100_00000100
    )]
    #[case(
        3,
        0b00001000_00001000_00001000_00001000_00001000_00001000_00001000_00001000
    )]
    #[case(
        4,
        0b00010000_00010000_00010000_00010000_00010000_00010000_00010000_00010000
    )]
    #[case(
        5,
        0b00100000_00100000_00100000_00100000_00100000_00100000_00100000_00100000
    )]
    #[case(
        6,
        0b01000000_01000000_01000000_01000000_01000000_01000000_01000000_01000000
    )]
    #[case(
        7,
        0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000
    )]
    #[case(
        8,
        0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001
    )]
    #[case(
        9,
        0b00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000010
    )]
    #[case(
        10,
        0b00000100_00000100_00000100_00000100_00000100_00000100_00000100_00000100
    )]
    #[case(
        11,
        0b00001000_00001000_00001000_00001000_00001000_00001000_00001000_00001000
    )]
    #[case(
        12,
        0b00010000_00010000_00010000_00010000_00010000_00010000_00010000_00010000
    )]
    #[case(
        13,
        0b00100000_00100000_00100000_00100000_00100000_00100000_00100000_00100000
    )]
    #[case(
        14,
        0b01000000_01000000_01000000_01000000_01000000_01000000_01000000_01000000
    )]
    #[case(
        15,
        0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000
    )]
    #[case(
        16,
        0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001
    )]
    #[case(
        17,
        0b00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000010
    )]
    #[case(
        18,
        0b00000100_00000100_00000100_00000100_00000100_00000100_00000100_00000100
    )]
    #[case(
        19,
        0b00001000_00001000_00001000_00001000_00001000_00001000_00001000_00001000
    )]
    #[case(
        20,
        0b00010000_00010000_00010000_00010000_00010000_00010000_00010000_00010000
    )]
    #[case(
        21,
        0b00100000_00100000_00100000_00100000_00100000_00100000_00100000_00100000
    )]
    #[case(
        22,
        0b01000000_01000000_01000000_01000000_01000000_01000000_01000000_01000000
    )]
    #[case(
        23,
        0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000
    )]
    #[case(
        24,
        0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001
    )]
    #[case(
        25,
        0b00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000010
    )]
    #[case(
        26,
        0b00000100_00000100_00000100_00000100_00000100_00000100_00000100_00000100
    )]
    #[case(
        27,
        0b00001000_00001000_00001000_00001000_00001000_00001000_00001000_00001000
    )]
    #[case(
        28,
        0b00010000_00010000_00010000_00010000_00010000_00010000_00010000_00010000
    )]
    #[case(
        29,
        0b00100000_00100000_00100000_00100000_00100000_00100000_00100000_00100000
    )]
    #[case(
        30,
        0b01000000_01000000_01000000_01000000_01000000_01000000_01000000_01000000
    )]
    #[case(
        31,
        0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000
    )]
    #[case(
        32,
        0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001
    )]
    #[case(
        33,
        0b00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000010
    )]
    #[case(
        34,
        0b00000100_00000100_00000100_00000100_00000100_00000100_00000100_00000100
    )]
    #[case(
        35,
        0b00001000_00001000_00001000_00001000_00001000_00001000_00001000_00001000
    )]
    #[case(
        36,
        0b00010000_00010000_00010000_00010000_00010000_00010000_00010000_00010000
    )]
    #[case(
        37,
        0b00100000_00100000_00100000_00100000_00100000_00100000_00100000_00100000
    )]
    #[case(
        38,
        0b01000000_01000000_01000000_01000000_01000000_01000000_01000000_01000000
    )]
    #[case(
        39,
        0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000
    )]
    #[case(
        40,
        0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001
    )]
    #[case(
        41,
        0b00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000010
    )]
    #[case(
        42,
        0b00000100_00000100_00000100_00000100_00000100_00000100_00000100_00000100
    )]
    #[case(
        43,
        0b00001000_00001000_00001000_00001000_00001000_00001000_00001000_00001000
    )]
    #[case(
        44,
        0b00010000_00010000_00010000_00010000_00010000_00010000_00010000_00010000
    )]
    #[case(
        45,
        0b00100000_00100000_00100000_00100000_00100000_00100000_00100000_00100000
    )]
    #[case(
        46,
        0b01000000_01000000_01000000_01000000_01000000_01000000_01000000_01000000
    )]
    #[case(
        47,
        0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000
    )]
    #[case(
        48,
        0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001
    )]
    #[case(
        49,
        0b00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000010
    )]
    #[case(
        50,
        0b00000100_00000100_00000100_00000100_00000100_00000100_00000100_00000100
    )]
    #[case(
        51,
        0b00001000_00001000_00001000_00001000_00001000_00001000_00001000_00001000
    )]
    #[case(
        52,
        0b00010000_00010000_00010000_00010000_00010000_00010000_00010000_00010000
    )]
    #[case(
        53,
        0b00100000_00100000_00100000_00100000_00100000_00100000_00100000_00100000
    )]
    #[case(
        54,
        0b01000000_01000000_01000000_01000000_01000000_01000000_01000000_01000000
    )]
    #[case(
        55,
        0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000
    )]
    #[case(
        56,
        0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001
    )]
    #[case(
        57,
        0b00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000010
    )]
    #[case(
        58,
        0b00000100_00000100_00000100_00000100_00000100_00000100_00000100_00000100
    )]
    #[case(
        59,
        0b00001000_00001000_00001000_00001000_00001000_00001000_00001000_00001000
    )]
    #[case(
        60,
        0b00010000_00010000_00010000_00010000_00010000_00010000_00010000_00010000
    )]
    #[case(
        61,
        0b00100000_00100000_00100000_00100000_00100000_00100000_00100000_00100000
    )]
    #[case(
        62,
        0b01000000_01000000_01000000_01000000_01000000_01000000_01000000_01000000
    )]
    #[case(
        63,
        0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000
    )]
    fn test_fill_column(#[case] spot_idx: u64, #[case] want: u64) {
        let board = Board(0);
        let filled_column = board.fill_column(spot_idx);
        assert_eq!(filled_column, want);
    }

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
        let board = Board(0);
        let queen_reach = board.fill_queen_reach(queen_idx, color_region);
        assert_eq!(queen_reach, want);
    }

    #[test]
    fn test_place_queen_invalid() {
        let board = Board::new();

        // Test index out of bounds
        assert_eq!(
            board.place_queen(64, 0),
            BoardPlacementResult::IndexOutOfBounds
        );

        // Test spot occupied
        let board = if let BoardPlacementResult::Success(b) = board.place_queen(0, 0) {
            b
        } else {
            panic!("Placing queen failed unexpectedly");
        };
        assert_eq!(board.place_queen(0, 0), BoardPlacementResult::SpotOccupied);

        // Test not in color region
        assert_eq!(
            board.place_queen(1, 1 << 2),
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
        let mut board = Board::new();
        for (idx, color) in initial_placements {
            board = if let BoardPlacementResult::Success(b) = board.place_queen(idx, color) {
                b
            } else {
                panic!("Failed to setup board for test");
            };
        }

        let result = board.place_queen(new_queen_idx, new_color_region);
        assert_eq!(
            result,
            BoardPlacementResult::Success(Board(expected_board_val))
        );
    }
    #[test]
    fn test_good_region_input() {
        let string = "11233456 12234456 11233456 12273456 11233456 88885556 66888886 66666666";
        let regions = parse_color_regions(string);
        assert_eq!(
            regions,
            [
                build_bit_set_from_inds(&[0, 1, 8, 16, 17, 24, 32, 33]),
                build_bit_set_from_inds(&[2, 9, 10, 18, 25, 26, 34]),
                build_bit_set_from_inds(&[3, 4, 11, 19, 20, 28, 35, 36]),
                build_bit_set_from_inds(&[5, 12, 13, 21, 29, 37]),
                build_bit_set_from_inds(&[6, 14, 22, 30, 38, 44, 45, 46]),
                build_bit_set_from_inds(&[
                    7, 15, 23, 31, 39, 47, 55, 48, 49, 56, 57, 58, 59, 60, 61, 62, 63
                ]),
                build_bit_set_from_inds(&[27]),
                build_bit_set_from_inds(&[40, 41, 42, 43, 50, 51, 52, 53, 54]),
            ]
        );
    }

    #[test]
    fn test_bad_region_input() {}
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

/// The user passes in the color regions by assigning numbers to each region.
/// Then they enter each row, left to right, top to bottom, with a space between the
/// rows. This function will verify that there are exactly 8 unique numbers used
/// An example input string might look like
/// "11233456 12234456 11233456 12273456 11233456 88885556 66888886 66666666"
pub fn parse_color_regions(input: &str) -> [u64; 8] {
    let mut regions = [0; 8];

    for (row_idx, row) in input.split_whitespace().enumerate() {
        for (col_idx, color) in row.chars().enumerate() {
            let color = color.to_digit(10).expect("Could not parse into int");
            if color > 8 {
                panic!("Color region number must be between 1 and 8");
            }
            let linear_idx = (8 * row_idx) + col_idx;
            regions[color as usize - 1] |= 1 << linear_idx;
        }
    }

    regions
}
