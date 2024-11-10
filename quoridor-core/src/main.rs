use quoridor_core::boardstate::locations::{PawnLocation, WallLocation, WallOrientation};
use quoridor_core::boardstate::{Action, Boardstate};

fn main() {
    let mut boardstate = Boardstate::new();
    let _ = boardstate.play_action(Action::Wall(
        WallLocation::build(5, WallOrientation::Vertical).unwrap(),
    ));
    let _ = boardstate.play_action(Action::Wall(
        WallLocation::build(25, WallOrientation::Horizontal).unwrap(),
    ));
    boardstate.print_board_state();

    // first move
    if let Err(error) = boardstate.play_action(Action::Pawn(PawnLocation::build(6).unwrap())) {
        println!("{error}");
    } else {
        println!("New boardstate");
        boardstate.print_board_state();
    }

    // // second move
    // if boardstate.play_action(Action::Pawn(PawnLocation::build(68).unwrap())).is_ok() {
    //     boardstate.print_board_state();
    // }
}
