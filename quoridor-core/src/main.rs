use quoridor_core::actions::Action;
use quoridor_core::boardstate::{Boardstate, Status};
use quoridor_core::locations::{PawnLocation, WallLocation, WallOrientation};

fn main() -> Result<(), String> {
    // let mut boardstate = Boardstate::new();
    // let _ = boardstate.play_action(Action::Wall(
    //     WallLocation::build(4, WallOrientation::Vertical).unwrap(),
    // ));
    // let _ = boardstate.play_action(Action::Wall(
    //     WallLocation::build(25, WallOrientation::Horizontal).unwrap(),
    // ));
    // boardstate.print_board_state();
    //
    // let action = Action::Pawn(PawnLocation::build(13).unwrap());
    // if let Err(error) = boardstate.play_action(action.clone()) {
    //     println!("{error}");
    // } else {
    //     println!("Play: {}", action.get_notation());
    //     boardstate.print_board_state();
    // }
    //
    // let action = Action::Pawn(PawnLocation::build(67).unwrap());
    // if let Err(error) = boardstate.play_action(action.clone()) {
    //     println!("{error}");
    // } else {
    //     println!("Play: {}", action.get_notation());
    //     boardstate.print_board_state();
    // }

    let mut boardstate = Boardstate::start_from(
        PawnLocation::build(67).unwrap(),
        PawnLocation::build(19).unwrap(),
        vec![
            WallLocation::build(30, WallOrientation::Horizontal).unwrap(),
            WallLocation::build(42, WallOrientation::Vertical).unwrap(),
            WallLocation::build(31, WallOrientation::Horizontal).unwrap(),
        ],
    )?;

    println!("After start_from:");
    boardstate.print_board_state();

    if let Ok(game_status) = boardstate.play_action(Action::Pawn(PawnLocation::build(76).unwrap()))
    {
        if let Status::Finished(winning_player) = game_status {
            println!("Game won by: {}", winning_player);
        }
        boardstate.print_board_state();
    }

    Ok(())
}
