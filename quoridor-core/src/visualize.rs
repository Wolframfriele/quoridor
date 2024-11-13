use crate::boardstate::Boardstate;
use crate::locations::{Coordinate, PawnLocation};

pub fn print_board_state(boardstate: &Boardstate) {
    for y in (0..=8u8).rev() {
        let mut horizontal_walls = String::from("  |");
        for x in 0..=8u8 {
            let location = PawnLocation::from_coordinate(Coordinate { x, y })
                .expect("The x and y range are made to be small enough");
            horizontal_walls.push_str(format_horizontal_wall(boardstate, location).as_str());
        }
        println!("{horizontal_walls}");

        let mut vertical_walls_and_paws = format!("{} |", y + 1);
        for x in 0..=8u8 {
            let location = PawnLocation::from_coordinate(Coordinate { x, y })
                .expect("The x and y range are made to be small enough");
            vertical_walls_and_paws.push_str(
                format!(
                    "  {}  {}",
                    format_pawn(
                        boardstate.get_position_white_pawn(),
                        boardstate.get_position_black_pawn(),
                        location.get_square()
                    ),
                    format_vertical_wall(boardstate, location)
                )
                .as_str(),
            );
        }
        println!("{vertical_walls_and_paws}");
    }
    println!("  |-----|-----|-----|-----|-----|-----|-----|-----|-----|");
    println!("     A     B     C     D     E     F     G     H     I   \n\n");
}

fn format_pawn(
    white_pawn_position: &PawnLocation,
    black_pawn_position: &PawnLocation,
    square: u8,
) -> char {
    if white_pawn_position.get_square() == square {
        return 'O';
    }
    if black_pawn_position.get_square() == square {
        return 'X';
    }
    ' '
}

// TODO Make use off wall orientations instead off the wall blocks
fn format_horizontal_wall(boardstate: &Boardstate, location: PawnLocation) -> String {
    let mut horizontal_line = String::from("--");
    if boardstate.is_blocked_horizontal(location.clone()) {
        horizontal_line.push('#')
    } else {
        horizontal_line.push('-')
    }
    let square = location.get_square();
    if square < 70 && boardstate.get_wall_positions()[usize::from(square)].is_some() {
        horizontal_line.push_str("--#");
    } else {
        horizontal_line.push_str("--|");
    }
    horizontal_line
}

// TODO Make use off wall orientations instead off the wall blocks
fn format_vertical_wall(boardstate: &Boardstate, location: PawnLocation) -> char {
    if boardstate.is_blocked_vertical(location) {
        return '#';
    }
    '|'
}