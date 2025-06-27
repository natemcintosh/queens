use queens::Board;

fn main() {
    let board = Board::new();
    // Place a queen at position (2, 2)
    let res = board.place_queen(2, 0b11111111);
    let b2 = match res {
        queens::BoardPlacementResult::Success(board) => board,
        queens::BoardPlacementResult::SpotOccupied => {
            println!("That spot was occupied");
            return;
        }
        queens::BoardPlacementResult::NotInColorRegion => {
            println!("Not in the allowed region");
            return;
        }
        queens::BoardPlacementResult::IndexOutOfBounds => {
            println!("Not in the board");
            return;
        }
    };
    println!("{b2}");
    let res = b2.place_queen(11, 0b00011111_00000000);
    let b3 = match res {
        queens::BoardPlacementResult::Success(board) => board,
        queens::BoardPlacementResult::SpotOccupied => {
            println!("That spot was occupied");
            return;
        }
        queens::BoardPlacementResult::NotInColorRegion => {
            println!("Not in the allowed region");
            return;
        }
        queens::BoardPlacementResult::IndexOutOfBounds => {
            println!("Not in the board");
            return;
        }
    };
    println!("{b3}");
}
