use quoridor_core::boardstate::locations::{PawnLocation, WallLocation, WallOrientation};
use quoridor_core::boardstate::{Action, Boardstate};

fn main() {
    let mut boardstate = Boardstate::new();
    let _ = boardstate.play_action(Action::Wall(
        WallLocation::build(4, WallOrientation::Vertical).unwrap(),
    ));
    let _ = boardstate.play_action(Action::Wall(
        WallLocation::build(25, WallOrientation::Horizontal).unwrap(),
    ));
    boardstate.print_board_state();

    let action = Action::Pawn(PawnLocation::build(13).unwrap());
    if let Err(error) = boardstate.play_action(action.clone()) {
        println!("{error}");
    } else {
        println!("Play: {}", action.get_notation());
        boardstate.print_board_state();
    }

    let action = Action::Pawn(PawnLocation::build(67).unwrap());
    if let Err(error) = boardstate.play_action(action.clone()) {
        println!("{error}");
    } else {
        println!("Play: {}", action.get_notation());
        boardstate.print_board_state();
    }
}
