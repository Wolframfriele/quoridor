use quoridor_core::boardstate::locations::{WallLocation, WallOrientation};
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
    let moves = boardstate.get_legal_moves();
    println!("Possible pawn moves:");
    for new_location in moves.get_pawn_actions(){
        println!("{}", new_location.get_notation());
    }
}
