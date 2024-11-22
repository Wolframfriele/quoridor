use anyhow::Result;

use quoridor_core::actions::Action;
use quoridor_core::boardstate::Boardstate;
//use quoridor_core::gamestate::GameStatus;
use quoridor_core::locations::{Coordinate, PawnLocation, WallLocation, WallOrientation};
use quoridor_core::visualize::{print_action, print_board_state};

fn main() -> Result<()> {
    // let mut boardstate = Boardstate::new();
    // let _ = boardstate.apply_action(Action::Wall(
    //     WallLocation::build(4, WallOrientation::Vertical).unwrap(),
    // ));
    // let _ = boardstate.apply_action(Action::Wall(
    //     WallLocation::build(25, WallOrientation::Horizontal).unwrap(),
    // ));
    //
    // println!("   Starting state:\n");
    // print_board_state(&boardstate);
    //
    // let action = Action::Pawn(PawnLocation::build(13).unwrap());
    // if let Err(error) = boardstate.apply_action(action.clone()) {
    //     println!("{error}");
    // } else {
    //     print_action(&boardstate, &action);
    // }
    //
    // let action = Action::Pawn(PawnLocation::build(67).unwrap());
    // if let Err(error) = boardstate.apply_action(action.clone()) {
    //     println!("{error}");
    // } else {
    //     print_action(&boardstate, &action);
    // }
    //
    // let action = Action::Wall(WallLocation::build(33, WallOrientation::Horizontal).unwrap());
    // if let Err(error) = boardstate.apply_action(action.clone()) {
    //     println!("{error}");
    // } else {
    //     print_action(&boardstate, &action);
    // }

    // let mut boardstate = Boardstate::start_from(
    //     PawnLocation::build(67).unwrap(),
    //     PawnLocation::build(19).unwrap(),
    //     vec![
    //         WallLocation::build(30, WallOrientation::Horizontal).unwrap(),
    //         WallLocation::build(42, WallOrientation::Vertical).unwrap(),
    //         WallLocation::build(31, WallOrientation::Horizontal).unwrap(),
    //     ],
    // )?;
    //
    // println!("After start_from:");
    // print_board_state(&boardstate);
    //
    // if let Ok(game_status) = boardstate.apply_action(Action::Pawn(PawnLocation::build(76).unwrap()))
    // {
    //     if let GameStatus::Finished{won_by, reason} = game_status {
    //         println!("Game won by: {:?}, reason: {:?}", won_by, reason);
    //     }
    //     print_board_state(&boardstate);
    // }
    //
    let boardstate = Boardstate::start_from(
            PawnLocation::build(4).unwrap(),
            PawnLocation::build(76).unwrap(),
            vec![
                WallLocation::build(36, WallOrientation::Horizontal).unwrap(),
                WallLocation::build(38, WallOrientation::Horizontal).unwrap(),
                WallLocation::build(40, WallOrientation::Horizontal).unwrap(),
                //WallLocation::build(42, WallOrientation::Horizontal).unwrap(),
                WallLocation::build(70, WallOrientation::Vertical).unwrap(),
                //WallLocation::build(52, WallOrientation::Horizontal).unwrap(),
                WallLocation::build(12, WallOrientation::Vertical).unwrap(),
            ],
            None,
        )
        .unwrap();
    print_board_state(&boardstate);
    println!("{:?}", &boardstate.get_wall_at_coordinate(Coordinate::from_square(70)));

    Ok(())
}
